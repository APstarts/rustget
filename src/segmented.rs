use crate::metadata::FileMetaData;
use anyhow::{Result, bail};
use futures_util::StreamExt;
use reqwest::Client;
use reqwest::header::RANGE;
use std::path::Path;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::io::AsyncSeekExt;
use tokio::io::AsyncWriteExt;

#[derive(Debug, Clone, Copy)]
pub struct Segment {
    pub start: u64,
    pub end: u64,
}

impl Segment {
    pub fn len(&self) -> u64 {
        self.end - self.start + 1
    }
    pub fn range_header(&self) -> String {
        format!("bytes={}-{}", self.start, self.end)
    }
}

pub struct SegmentResult {
    pub segment: Segment,
    pub bytes_written: u64,
}

async fn download_segment(
    client: Client,
    url: String,
    output_path: PathBuf,
    segment: Segment,
) -> Result<SegmentResult> {
    let range = segment.range_header();
    let response = client.get(&url).header(RANGE, range).send().await?; //requesting
    //the
    //range
    //by
    //sending
    //header
    //information
    //about
    //required
    //range

    if response.status() != reqwest::StatusCode::PARTIAL_CONTENT {
        //checking 206 partial content
        bail!("Expected 206 Partial Content, got {}", response.status());
    }
    let mut file = OpenOptions::new().write(true).open(&output_path).await?; //configuring how
    file.seek(std::io::SeekFrom::Start(segment.start)).await?; //open the file at the exact position

    // streaming instead of capturing the complete range into ram. This keeps the memory usage low.
    let mut stream = response.bytes_stream();
    let mut bytes_written: u64 = 0; //keeping log of the bytes actually written
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        file.write_all(&chunk).await?; //writing to the file.
        bytes_written += chunk.len() as u64;
    }
    // file.flush().await?; //flush() tells the os to push the bytes currently in your application-side
    //buffer into the operating system's kernel buffer. While this doesn't
    //guaranttee the data has reached the physical disk(that requires
    //file.sync_call() it ensures that OS is now responsible for the data.
    // Here we don't need file.flush() as when the file goes out of scope rust closes the file and
    // closing the file causes the OS to flush the buffered data. removing this flush code will
    // cause one less system call.

    Ok(SegmentResult {
        segment: segment,
        bytes_written: bytes_written,
    })
}

/// Takes in the total size and number of connections to calculate the ranges to download
pub fn calculate_ranges(total_size: u64, connections: u64) -> Vec<Segment> {
    println!("Total size: {total_size}");
    println!("connections: {connections}");
    let range_size = total_size / connections;
    let mut ranges: Vec<Segment> = Vec::new();
    for i in 0..connections {
        let start = i * range_size;
        let end = if i == connections - 1 {
            total_size - 1
        } else {
            (i + 1) * range_size - 1
        };
        ranges.push(Segment { start, end });
    }
    return ranges;
}

/// Create the file in advance using this function which can then be used to open independent
/// handles on it to write data into it concurrently
pub async fn prepare_file(path: &Path, total_size: u64) -> Result<()> {
    let file = File::create(path).await?; //create the file using path that is the output path
    //created inside the segmented_download function
    //why file is not set to mut? because set_len() modifies the file on disk and not the rust
    //variable
    file.set_len(total_size).await?; //setting the size of the file
    Ok(())
}

pub async fn segmented_download(client: &Client, metadata: &FileMetaData, url: &str) -> Result<()> {
    let total_size = metadata
        .content_length
        .expect("Segmented download requires content length");

    let connections: u64 = 4;
    let output_path = PathBuf::from(&metadata.filename);
    prepare_file(&output_path, total_size).await?;

    let mut handles = Vec::new(); //number of concurrent
    //tasks
    let segments = calculate_ranges(total_size, connections);
    for segment in segments {
        let client = client.clone();
        let url = url.to_owned();
        let output_path = output_path.clone();

        let handle =
            tokio::spawn(async move { download_segment(client, url, output_path, segment).await });

        handles.push(handle);
    }

    let mut results = Vec::new();

    for handle in handles {
        results.push(handle.await??);
    }

    for result in &results {
        println!(
            "{}-{} -> {} bytes",
            result.segment.start, result.segment.end, result.bytes_written
        );
    }

    println!("Download Segments:");

    Ok(())
}
