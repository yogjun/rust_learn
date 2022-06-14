use anyhow::Result;
use polars::prelude::*;
use std::io::Cursor;
use queryer::query;

#[tokio::main]
async fn main() -> Result<()> {
    // tracing_subscriber::fmt::init();
    // let url = "file:///Users/miaojun/code/rust/rust_learn/querys/queryer/csv/co.csv";
    // // 使用 sql 从 URL 里获取数据
    // let sql = format!( "SELECT location name, total_cases, new_cases, total_deaths, new_deaths  FROM {} where new_deaths >= 500 ORDER BY new_cases DESC", url );
    // let df1 = query(sql).await?;
    // println!("{:?}", df1);
    // Ok(())
    // 第一个，寄件信息数据
    tracing_subscriber::fmt::init();
    let url = "file:///Users/miaojun/code/rust/rust_learn/querys/queryer/csv/sender.csv";
    // 使用 sql 从 URL 里获取数据
    let sql = format!("SELECT name,phone,country,crovince,city,post_code,add1,add2,add3,enterprise_name,enterprise_number,email,tax1,tax2,tax3  FROM {}", url );
    let df1 = query(sql).await?;
    


    // println!("{:?}", df1);
    println!("{:?}", df1.get_columns());
    Ok(())
}
