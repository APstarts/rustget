use crate::{progress::ProgressTracker, utils::infer_filename};
use anyhow::{Result, bail};
use futures_util::StreamExt;
use reqwest::Client;
use std::io::{Write, stdout};
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn download(client: Client, url: String) -> Result<()> {
    if url.is_empty() {
        bail!("URL cannot be empty");
    }

    println!("Connecting...");

    let response = reqwest::get(&url).await?;

    let status = response.status();

    if !status.is_success() {
        bail!("Server returned {}", status);
    }

    let content_length = response.content_length();

    let filename = infer_filename(&url).unwrap();

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
