use anyhow::Result;
use queryer::query;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // let url = "file:///Users/miaojun/code/rust/rust_learn/querys/queryer/csv/co.csv";
    // let url = "https://raw.githubusercontent.com/yogjun/rust_learn/main/queryer/csv/co.csv";
    // let sql = format!( "SELECT location name, total_cases, new_cases, total_deaths, new_deaths  FROM {} where new_deaths >= 500 ORDER BY new_cases DESC", url );
    // let sql = format!( "SELECT t1.location name, t1.total_cases, t1.new_cases, t1.total_deaths, t1.new_deaths  FROM {} as t1 where t1.new_deaths >= 500 ORDER BY t1.new_cases DESC", url );
    // let sql = format!( "SELECT a.location name FROM {} as a left join {} as b on a.new_deaths = b.new_deaths where a.new_deaths >= 500 GROUP BY a.new_deaths", url,url );

    let url1 = "file:///Users/miaojun/code/rust/rust_learn/querys/queryer/csv/co1.csv";
    let url2 = "https://raw.githubusercontent.com/yogjun/rust_learn/main/queryer/csv/co2.csv";
    // let sql = format!( "SELECT location name, total_cases, new_cases, total_deaths, new_deaths  FROM {} where new_deaths >= 500 ORDER BY new_cases DESC", url );
    // let sql = format!( "SELECT t1.location name, t1.total_cases, t1.new_cases, t1.total_deaths, t1.new_deaths  FROM {} as t1 where t1.new_deaths >= 500 ORDER BY t1.new_cases DESC", url );
    let sql = format!( "SELECT a1,a2 FROM {} left join {} on a1 = a2 where f1 >= 500", url1,url2 );

    let df1 = query(sql).await?;
    println!("{:?}", df1);
    Ok(())
}
