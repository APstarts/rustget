use anyhow::{Result, bail};
use reqwest::Client;
use reqwest::header::{ACCEPT_RANGES, CONTENT_LENGTH};

use crate::utils::resolve_filename;

pub struct FileMetaData {
    pub content_length: Option<u64>,
    pub accept_ranges: bool,
    pub filename: String,
}

impl FileMetaData {
    /// if metadata supports segmented downloading then only utilise the segmented downloading
    pub fn supports_segmented_download(&self) -> bool {
        self.accept_ranges && self.content_length.is_some()
    }
}

pub async fn get_metadata(client: &Client, url: &str) -> Result<FileMetaData> {
    let response = client.head(url).send().await?;

    if !response.status().is_success() {
        bail!("Server returned {}", response.status());
    }
    let content_length = response
        .headers()
        .get(CONTENT_LENGTH)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<u64>().ok());
    println!("Content Length: {}", content_length.unwrap());
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
