use anyhow::{Result, bail};
use clap::Parser;
use futures_util::StreamExt;
use reqwest;
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Parser)]
struct DownloadOptions {
    url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let options = DownloadOptions::parse();
    download(options).await?;
    Ok(())
}

async fn download(options: DownloadOptions) -> Result<()> {
    //Validate input
    if options.url.is_empty() {
        bail!("URL cannot be empty");
    }
    println!("Downloading: {}", options.url);

    //Send HTTP Response
    let response = reqwest::get(&options.url).await?;
    //inspect metadata

    let status = response.status();
    println!("Status: {}", status);

    if !status.is_success() {
        bail!("Server returned {}", status);
    }

    match response.content_length() {
        Some(size) => {
            println!("File size: {} byes", size);
        }
        None => {
            println!("Uknown file size");
        }
    }

    //crate file
    let mut file = File::create("download.bin").await?;

    // Turn the response body into an async stream
    let mut stream = response.bytes_stream();

    // Read one chunk at a time
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?;
    }
    println!("Download completed");
    Ok(())
}
