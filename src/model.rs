use std::{future::Future, path::Path, pin::Pin};

use axum::extract::multipart::Field;
use futures::{Stream, StreamExt};
use serde::Serialize;
use sha2::{ Digest, Sha256};
use sqlx::PgPool;
use tokio::io::{AsyncRead, AsyncReadExt};
use bytes::Bytes;
use crate::error::{Error, Result};

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
    // The path of where the file is stored on the host machine.
    pub file_path: String,
    // The SHA-256 checksum of the file.
    pub file_hash: String,
}

impl Media {
    // Returns a relative url to be attached to a base cdn url.
    // Format: "/{id}_{uploaded_time}_fname.ext?checksum={hash}"
    pub fn to_rel_url(&self) -> String {
        let path = Path::new(&self.file_path);
        // format!(
        //     "/{}_{}_{}.{}", 
        //     self.file_hash, 
        //     self.id, 
        //     path.file_name().unwrap(), 
        //     path.extension()
        // )
        todo!()
    }
}

pub struct MediaUploadInfo {
    pub file_path: String,
    pub file_size: i64,
    pub file_hash: String,
    pub upload_start_time: i64
}

// Model controllers    
