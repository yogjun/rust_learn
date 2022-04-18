use anyhow::Result;
use polars::prelude::*;
use std::io::Cursor;
use queryer::query;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let url = "https://raw.githubusercontent.com/yogjun/rust_learn/main/queryer/csv/co.csv";
    // let data = reqwest::get(url).await?.text().await?;
    // let df = CsvReader::new(Cursor::new(data))
    //     .infer_schema(Some(16))
    //     .finish()?;

    // let filtered = df.filter(&df["new_deaths"].gt(500))?;

    // println!(
    //     "{:?}",
    //     filtered.select((
    //         "location",
    //         "total_cases",
    //         "new_cases",
    //         "total_deaths",
    //         "new_deaths"
    //     ))
    // );

    // 使用 sql 从 URL 里获取数据
    // let sql = format!( "SELECT location name, total_cases, new_cases, total_deaths, new_deaths  FROM {} where new_deaths >= 500 ORDER BY new_cases DESC", url );
    let sql = format!( "SELECT a.location name FROM {} as a left join {} as b on a.new_deaths = b.new_deaths where a.new_deaths >= 500 GROUP BY a.new_deaths", url,url );

    let df1 = query(sql).await?;
    println!("{:?}", df1);
    Ok(())
}
