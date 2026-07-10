mod downloader;
mod options;
mod progress;
mod utils;

use anyhow::Result;
use clap::Parser;

use downloader::download;
use options::DownloadOptions;

#[tokio::main]
async fn main() -> Result<()> {
    let options = DownloadOptions::parse();

    download(options).await?;

    Ok(())
}
