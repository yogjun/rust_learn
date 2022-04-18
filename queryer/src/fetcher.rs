use anyhow::{anyhow, Result, Ok};
use async_trait::async_trait;
use tokio::fs;

#[async_trait]
pub trait Fetch {
    type Error = Error;async fn fetch(&self) -> Result<String,Self::Error>;
}

pub async fn retrieve_data(source: impl AsRef<str>) -> Result<String> {
    let name = source.as_ref();
    match &name[..4] {
        "http" => Urlfetcher(name).fetch().await,
        "file" => FileFetcher(name).fetch().await,
        _ => return Err(anyhow!("We only support http/https/file at the moment")),
    }
}

struct UrlFetcher<'a>(pub(crate) &'a str);
struct FileFetcher<'a>(pub(crate) &'a str);

#[async_trait]
impl<'a> Fetcher for UrlFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String,Self::Error> {
        Ok(reqwest::get(self.0).await?.text().await?)
    }
}

#[async_trait]
impl<'a> Fetcher for FileFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String,Self::Error> {
        Ok(fs::read_to_string(&self.0[7..]).await?)
    }
}


