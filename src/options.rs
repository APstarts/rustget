use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "A segmented downloader")]
pub struct DownloadOptions {
    pub urls: Vec<String>,
    #[arg(short, long, default_value_t = 4)]
    pub connections: usize,
}
