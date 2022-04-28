use anyhow::Result;
use queryer::query;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let url = "file:///Users/miaojun/code/rust/rust_learn/querys/queryer/csv/co.csv";
    // let url = "https://raw.githubusercontent.com/yogjun/rust_learn/main/queryer/csv/co.csv";

    // 使用 sql 从 URL 里获取数据
    let sql = format!( "SELECT location name, total_cases, new_cases, total_deaths, new_deaths  FROM {} where new_deaths >= 500 ORDER BY new_cases DESC", url );
    // let sql = format!( "SELECT location name, total_cases, new_cases, total_deaths, new_deaths  FROM {} where new_deaths >= 500 ORDER BY new_cases DESC", url );
    // let sql = format!( "SELECT a.location name FROM {} as a left join {} as b on a.new_deaths = b.new_deaths where a.new_deaths >= 500 GROUP BY a.new_deaths", url,url );

    let df1 = query(sql).await?;
    println!("{:?}", df1);
    Ok(())
}
