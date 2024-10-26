use std::{ffi::{OsStr, OsString}, path::Path};
use crate::config::Config;
use tokio::{fs::read, io::AsyncReadExt};
use reqwest::{multipart::{Form, Part}, Client, StatusCode};

pub enum UploadError {
    NoFileFound,
    ReqwestError { err: reqwest::Error },
    FailStatusCode { status_code: StatusCode }
}

// https://stackoverflow.com/questions/70926337/how-to-post-a-multipart-form-using-async-version-of-reqwest-crate
// 
pub async fn upload_file(path: &Path, config: &Config) -> Result<String, UploadError> {
    println!("Uploading {:?}...", path.file_name().unwrap_or_default());

    let client = Client::new();

    let fname: String = path.file_name().unwrap().to_string_lossy().to_string();

    let multipart_form = Form::new()
        .file(fname, path).await
        .map_err(|_| UploadError::NoFileFound)?;

    let endpoint: String = format!("{}{}", config.cloud_url, "media");
    let res = client.post(&endpoint)
        .multipart(multipart_form)
        .send().await.map_err(|e| UploadError::ReqwestError { err: e })?;

    if !res.status().is_success() {
        return Err(UploadError::FailStatusCode { status_code: res.status() });
    }

    let media_endpoint = res.text().await
        .map_err(|e| UploadError::ReqwestError { err: e })?.to_string();
    
    Ok(format!("{}media/{}", config.cloud_url, media_endpoint))
}