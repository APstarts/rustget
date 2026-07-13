use crate::metadata::FileMetaData;
use anyhow::Result;
use reqwest::Client;

pub async fn segmented_download(client: &client, metadata: &FileMetaData, url: &str) -> Result<()> {
    //todo
}
