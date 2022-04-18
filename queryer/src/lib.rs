use anyhow::{anyhow, Result, Ok};
use polars::prelude::*;
use sqlparser::parser::Parser;
use std::convert::TryInto;
use std::ops::{Deref, DerefMut};
use tracing::info;
mod convert;
mod dialect;
mod fetcher;
mod loader;
use convert::Sql;
pub use dialect::example_sql;
pub use dialect::TyrDialect;
use fetcher::retrieve_data;
use loader::detect_content;

#[derive(Debug)]
pub struct DataSet(DataFrame);

/// 我们自己的类型 Dataset 使用起来Dataframe
impl Deref for DataSet {
    type Target = DataFrame;
    fn deref(&self) -> &self::Target {
        &self.0
    }
}

impl DerefMut for DataSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl DataSet {
    pub fn to_csv(&self) -> Result< String> {
        let mut buf = Vec::new();
        let writer = CsvWriter::new(&mut buf);
        writer.finish(self)?;
        Ok(String::from_utf8(buf)?)
    }
}

/// 从 from 中获取数据，从 where 中过滤，最后选取需要返回的列
pub async fn query<T: AsRef<str>>(sql: T) -> Result<DataSet> {
    let ast = Parser::parse_sql(&TyrDialect::default(),sql.as_ref())?;

    if ast.len() != 1 {
        return Err(anyhow!("Only support single sql at the moment"));
    }

    let sql = &ast[0];

    let Sql {
        source,condition,selection,offset,limit,order_by,
    } = sql.try_into()?;

    info!("retrieving data from source: {}",source);

    // 从source 读入一个DataSet
    // detect_content
    let ds = detect_content(retrieve_data(source).await?).load()?;

    let mut filtered = match condition {
        Some(expr) => ds.0.lazy().filter(expr),
        None => ds.0.lazy(),
    };

    filtered = order_by.into_iter().fold(filtered, |acc,(col,desc)| acc.sort(&col,desc).collect());

    if offset.is_some() || limit.is_some() {
        filtered = filtered.slice(offset.unwrap_or(0),limit.unwrap_or(usize::MAX));
    }

    Ok(DataSet(filtered.select(selection).collect()?))

}