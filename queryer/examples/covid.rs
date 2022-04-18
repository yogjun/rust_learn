use anyhow::Result;
use polars::prelude::*;
use std::io::Cursor;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let url = "https://raw.githubusercontent.com/yogjun/rust_learn/main/queryer/csv/co.csv";
    let data = reqwest::get(url).await?.text().await?;
    let df = CsvReader::new(Cursor::new(data))
        .infer_schema(Some(16))
        .finish()?;

    let filtered = df.filter(&df["new_deaths"].gt(500))?;

    println!(
        "{:?}",
        filtered.select((
            "location",
            "total_cases",
            "new_cases",
            "total_deaths",
            "new_deaths"
        ))
    );
    Ok(())
}
