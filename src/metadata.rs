use anyhow::{Result, bail};
use reqwest::Client;
use reqwest::header::ACCEPT_RANGES;

use crate::utils::resolve_filename;

pub struct FileMetaData {
    pub content_length: Option<u64>,
    pub accept_ranges: bool,
    pub filename: String,
}

pub async fn get_metadata(client: &Client, url: &str) -> Result<FileMetaData> {
    let response = client.head(url).send().await?;

    if !response.status().is_success() {
        bail!("Server returned {}", response.status());
    }
    let content_length = response.content_length();
    let headers = response.headers();
    let accept_ranges = headers
        .get(ACCEPT_RANGES)
        .is_some_and(|val| val.as_bytes().eq_ignore_ascii_case(b"bytes"));
    let filename = resolve_filename(&headers, &url)?;
    Ok(FileMetaData {
        content_length: content_length,
        accept_ranges: accept_ranges,
        filename: filename,
    })
}
