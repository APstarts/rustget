use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct DownloadOptions {
    pub urls: Vec<String>,
}
