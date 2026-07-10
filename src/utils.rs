use anyhow::Result;
use std::path::Path;
use url::Url;

pub fn infer_filename(url: &str) -> Result<String> {
    let parsed = Url::parse(url)?;

    let path = Path::new(parsed.path());

    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("download.bin");

    Ok(filename.to_string())
}
