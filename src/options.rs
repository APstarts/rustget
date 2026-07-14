use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
#[arg(short, long, default_value_t = 4)]
pub struct DownloadOptions {
    pub urls: Vec<String>,
    pub connections: usize,
}
