use anyhow::{Result, bail};
use futures_util::StreamExt;
use std::io::{Write, stdout};
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{options::DownloadOptions, progress::ProgressTracker, utils::infer_filename};

pub async fn download(options: DownloadOptions) -> Result<()> {
    if options.url.is_empty() {
        bail!("URL cannot be empty");
    }

    println!("Connecting...");

    let response = reqwest::get(&options.url).await?;

    let status = response.status();

    if !status.is_success() {
        bail!("Server returned {}", status);
    }

    let content_length = response.content_length();

    let filename = infer_filename(&options.url)?;

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
