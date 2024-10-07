use std::{future::Future, pin::Pin};

use axum::extract::multipart::Field;
use futures::{Stream, StreamExt};
use serde::Serialize;
use sha2::{ Digest, Sha256};
use sqlx::PgPool;
use tokio::io::{AsyncRead, AsyncReadExt};
use bytes::Bytes;
use crate::error::{Error, Result};

// Contains data needed to get the file's url, as well as the url.
// This is to be returned as a repsonse when someone requests a file,
// or whe
#[derive(Serialize)]
pub struct Media {
    url: String
}

pub struct MediaUploadInfo {
    pub filename: String,
    pub file_path: String,
    pub file_size: u32
}

// Model controllers    
