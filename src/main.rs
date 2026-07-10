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

    let mut handles = Vec::new();

    for url in options.urls {
        let handle = tokio::spawn(async move {
            //tokio spawn
            //creates a
            //lightweight
            //async task
            //just like
            //thread::spawn
            //creates an
            //OS thread
            download(url).await
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await??;
    }

    Ok(())
}
