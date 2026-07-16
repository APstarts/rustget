use crate::metadata::{FileMetaData, get_metadata};
use crate::progress::ProgressTracker;
use crate::segmented::segmented_download;
use anyhow::{Result, bail};
use futures_util::StreamExt;
use reqwest::Client;
use std::io::{Write, stdout};
use tokio::{fs::File, io::AsyncWriteExt};

enum DownloadStrategy {
    Normal,
    Segmented,
}

fn download_strategy(metadata: &FileMetaData) -> DownloadStrategy {
    if metadata.supports_segmented_download() {
        DownloadStrategy::Segmented
    } else {
        DownloadStrategy::Normal
    }
}
pub async fn download(client: Client, url: String) -> Result<()> {
    if url.is_empty() {
        bail!("URL cannot be empty");
    }

    let metadata = get_metadata(&client, &url).await?;

    println!("Downloading filename: {}", metadata.filename);

    match download_strategy(&metadata) {
        DownloadStrategy::Segmented => {
            println!("Using segmented download");

            segmented_download(&client, &metadata, &url).await?;
        }
        DownloadStrategy::Normal => {
            normal_download(&client, &metadata, &url).await?;
        }
    }

    println!("Connecting...");
    Ok(())
}

async fn normal_download(client: &Client, metadata: &FileMetaData, url: &str) -> Result<()> {
    let response = client.get(url).send().await?;

    let status = response.status();

    if !status.is_success() {
        bail!("Server returned {}", status);
    }

    let content_length = response.content_length();

    let filename = &metadata.filename;

    println!("Saving as: {}", filename);

    let mut file = File::create(&filename).await?;

    let mut tracker = ProgressTracker::new(content_length);

    let mut stream = response.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;

        file.write_all(&chunk).await?;

        tracker.add_bytes(chunk.len() as u64);

        if let Some(percent) = tracker.percentage() {
            print!(
                "\rDownloading {:>6.2}% ({}/{:?} bytes)",
                percent,
                tracker.downloaded(),
                tracker.total(),
            );

            stdout().flush()?;
        } else {
            print!("\rDownloaded {} bytes", tracker.downloaded());

            stdout().flush()?;
        }
    }

    println!();
    println!("Download complete!");

    Ok(())
}
