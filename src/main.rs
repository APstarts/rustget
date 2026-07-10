use anyhow::{Result, bail};
use clap::Parser;
use futures_util::StreamExt;
use reqwest;
use std::path::Path;
use tokio::{fs::File, io::AsyncWriteExt};
use url::Url;

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

    //checking the file size reported by server
    match response.content_length() {
        Some(size) => {
            println!("File size: {} byes", size);
        }
        None => {
            println!("Uknown file size");
        }
    }

    let file_name = infer_filename(&options.url).expect("couldn't infer file name");
    //crate file
    let mut file = File::create(file_name).await?;

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

fn infer_filename(url: &str) -> anyhow::Result<String> {
    let parsed = Url::parse(url)?; //to parse the url correctly
    let path = Path::new(parsed.path()); //parsed.path() here gives out something like
    //files.report.pdf and Path::new() actually
    //converts it into a file system like path so that
    //we can use methods like file_name() to get the
    //file name from the converted path.
    let file_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("download.bin"); // since
    // Path::new()
    // returns
    // OsStr
    // which
    // needs
    // to
    // be
    // converted
    // into
    // str
    Ok(file_name.to_string())
}
