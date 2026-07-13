use anyhow::Result;
use reqwest::header::CONTENT_DISPOSITION;
use reqwest::header::HeaderMap;
use std::path::Path;
use url::Url;

pub fn resolve_filename(headers: &HeaderMap, url: &str) -> Result<String> {
    let filename = extract_filename_from_content_disposition(headers)
        .or_else(|| infer_filename(url))
        .unwrap_or_else(|| "download.bin".to_string());
    Ok(filename)
}

pub fn extract_filename_from_content_disposition(headers: &HeaderMap) -> Option<String> {
    let header_str = headers.get(CONTENT_DISPOSITION)?.to_str().ok()?;

    for directive in header_str.split(";") {
        let directive = directive.trim();

        if let Some(filename_part) = directive.strip_prefix("filename=") {
            //strip surrounding quotes, which are standard in the HTTP spec.
            return Some(filename_part.trim_matches('"').to_string());
        }
    }
    None
}

pub fn infer_filename(url: &str) -> Option<String> {
    let parsed = Url::parse(url).ok()?;

    let path = Path::new(parsed.path());

    let filename = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("download.bin");
    Some(filename.to_string())
}
