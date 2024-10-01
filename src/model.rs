use std::{future::Future, pin::Pin};

use axum::extract::multipart::Field;
use futures::{Stream, StreamExt};
use serde::Serialize;
use sha2::{ Digest, Sha256};
use sqlx::PgPool;
use tokio::io::{AsyncRead, AsyncReadExt};
use bytes::Bytes;
use crate::error::{Error, Result};


// Types
pub struct Media {
    path: String,

}

pub struct MediaUploadInfo<'a> {
    pub filename: String,
    pub data: Field<'a>
}

// Contains data needed to get the file's url.
#[derive(Serialize)]
pub struct UploadedMediaInfo {
    pub filename: String,
    pub hash: String
}

// Model controllers    
#[derive(Clone)]
pub struct MediaController {
    db_pool: PgPool
}

impl MediaController {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            db_pool
        }
    }
}

impl MediaController {
    pub async fn upload_media(&self, mut info: MediaUploadInfo<'static>) -> Result<UploadedMediaInfo> {
        let mut hasher = Sha256::new();

        // while let Some(chunk_result) = info.bytes.next().await {
        //     let chunk = chunk_result?;
        //     hasher.update(&chunk);
        //     total_bytes += chunk.len();
        //     println!("got some bytes: {total_bytes}");
        //     // Here you would typically write these bytes to storage
        //     // For example: storage.write_chunk(&chunk).await?;
        // }

        let hash = format!("{:x}", hasher.finalize());

        let uploaded_info = UploadedMediaInfo {
            filename: info.filename,
            hash,
        };

        self.complete_media_upload(&uploaded_info).await?;

        Ok(uploaded_info)
    }

    pub async fn complete_media_upload(&self, info: &UploadedMediaInfo) -> Result<()> {
        
        todo!()
    }

    pub async fn get_media( ) {

    }

    pub async fn delete_media() {

    } 
}