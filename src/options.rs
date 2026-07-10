use clap::Parser;

#[derive(Parser, Debug)]
pub struct DownloadOptions {
    pub urls: Vec<String>,
}
