mod downloader;
mod options;
mod progress;
mod utils;
use anyhow::Result;
use clap::Parser;
use std::sync::Arc;

use downloader::download;
use options::DownloadOptions;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() -> Result<()> {
    const MAX_CONCURRENT_DOWNLOADS: usize = 4;
    let options = DownloadOptions::parse();
    let semaphore = Arc::new(Semaphore::new(MAX_CONCURRENT_DOWNLOADS)); //to limit the number of concurrent tasks
    //allowed at a time.

    let mut handles = Vec::new();

    for url in options.urls {
        let semaphore_arcloned = Arc::clone(&semaphore);
        let handle = tokio::spawn(async move {
            //tokio spawn
            //creates a
            //lightweight
            //async task
            //just like
            //thread::spawn
            //creates an
            //OS thread
            let _permit = semaphore_arcloned.acquire_owned().await?; //When should a task a permit? Immediately
            //before download(url) Why? Because waiting
            //for  a permit is asynchronous. The task
            //can happily exist while waiting.
            download(url).await
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await??;
    }

    Ok(())
}
