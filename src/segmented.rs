use crate::metadata::FileMetaData;
use anyhow::{Result, bail};
use reqwest::Client;
use reqwest::header::RANGE;
use tokio::fs::OpenOptions;
use tokio::io::AsyncSeekExt;
use tokio::{fs::File, io::AsyncWriteExt};

pub struct SegmentResult {
    pub start: u64,
    pub end: u64,
    pub bytes_written: u64,
}

async fn download_segment(
    client: Client,
    url: String,
    start: u64,
    end: u64,
) -> Result<SegmentResult> {
    let range = format!("bytes={}-{}", start, end);

    let response = client.get(&url).header(RANGE, range).send().await?;

    if response.status() != reqwest::StatusCode::PARTIAL_CONTENT {
        bail!("Expected 206 Partial Content, got {}", response.status());
    }

    let bytes = response.bytes().await?;

    println!("Downloaded range {}-{} ({} bytes)", start, end, bytes.len());

    Ok(SegmentResult {
        start,
        end,
        bytes_written: bytes.len() as u64,
    })
}

pub fn calculate_ranges(total_size: u64, connections: u64) -> Vec<(u64, u64)> {
    let range_size = total_size / connections;
    let mut ranges: Vec<(u64, u64)> = Vec::new();
    for i in 0..connections {
        let start = i * range_size;
        let end = if i == 3 {
            total_size - 1
        } else {
            (i + 1) * range_size - 1
        };
        let range = (start, end);
        ranges.push(range);
    }
    return ranges;
}

pub fn prepare_file(filename: &String, total_size: u64) {}

pub async fn segmented_download(client: &Client, metadata: &FileMetaData, url: &str) -> Result<()> {
    let filename = &metadata.filename;
    let file = File::create(&filename).await?;

    let total_size = metadata
        .content_length
        .expect("Segmented download requires content length");

    let connections: u64 = 4;

    let chunk_size = total_size / connections;

    let mut handles = Vec::new(); //number of concurrent
    //tasks

    for i in 0..connections {
        let start = i * chunk_size;
        let end = if i == connections - 1 {
            total_size - 1
        } else {
            start + chunk_size - 1
        };

        let client = client.clone();
        let url = url.to_owned();

        let handle = tokio::spawn(async move { download_segment(client, url, start, end).await });
        handles.push(handle);
    }

    let mut results = Vec::new();

    for handle in handles {
        let result = handle.await??;
        results.push(result);
    }

    println!("Download Segments:");

    for result in results {
        println!(
            "{}-{} -> {} bytes",
            result.start, result.end, result.bytes_written
        );
    }

    Ok(())
}
