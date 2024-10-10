use std::{future::Future, path::{Path, PathBuf}, pin::Pin};

use axum::extract::multipart::Field;
use futures::{Stream, StreamExt};
use serde::{Deserialize, Serialize};
use sha2::{ Digest, Sha256};
use sqlx::PgPool;
use tokio::io::{AsyncRead, AsyncReadExt};
use bytes::Bytes;
use crate::{cdn_settings, error::{Error, Result}};

// A row from the database.
#[derive(Serialize)]
pub struct Media {
    pub id: i64,
    // Unix timestamp in milliseconds.
    pub uploaded_time: i64,
    // Unix timestamp in milliseconds. Will be used to implement caching later.
    pub accessed_time: i64,
    // Unix timestamp in milliseconds. TODO!
    pub expiring_time: i64,
    // Size of the file in bytes.
    pub file_size: i64,
    // The name of the file on the host machine, including the extension.
    pub file_name: String,
    // The SHA-256 checksum of the file.
    pub file_hash: String,
}

impl Media {
    // Returns a relative url to be attached to a base cdn url.
    // Format: "/fname.ext?id={id}&checksum={hash}"
    pub fn rel_endpoint(&self) -> String {
        format!(
            "/{}?&id={}&checksum={}", 
            self.file_name, 
            self.id, 
            self.file_hash  
        )
    }

    pub fn true_filename(&self) -> String {
        format!(
            "{}_{}", 
            self.id,
            self.file_hash
        )
    }

    pub async fn true_path(&self) -> PathBuf {
        let save_dir: &String = &cdn_settings.read().await.save_dir;
        let path: &Path = Path::new(save_dir);

        path.join(&self.true_filename())
    }
}

pub struct MediaUploadInfo {
    pub file_name: String,
    pub file_size: i64,
    pub file_hash: String,
    pub upload_start_time: i64
}           

pub struct MediaRequestInfo {
    pub id: i64,
    pub file_name: String,
    pub file_hash: String
}

// CDN settings
#[derive(Deserialize, Debug)]
pub struct CdnSettings {
    pub save_dir: String
}
