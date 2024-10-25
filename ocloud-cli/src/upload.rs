use std::path::Path;
use anyhow::Result;
use crate::config::Config;
use tokio::{fs::read, io::AsyncReadExt};
use reqwest::multipart::Form;

// https://stackoverflow.com/questions/70926337/how-to-post-a-multipart-form-using-async-version-of-reqwest-crate
// 
pub async fn upload_file(path: &Path, config: &Config) -> Result<String> {
    let multipart_form = Form::new();
    
    let url = String::new();
    Ok(url)
}