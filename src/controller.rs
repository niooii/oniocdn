use sqlx::PgPool;

use crate::model::{Media, MediaUploadInfo};
use crate::error::Result;

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
    pub async fn checkin_media(&self, info: MediaUploadInfo) -> Result<Media> {
        // while let Some(chunk_result) = info.bytes.next().await {
        //     let chunk = chunk_result?;
        //     hasher.update(&chunk);
        //     total_bytes += chunk.len();
        //     println!("got some bytes: {total_bytes}");
        //     // Here you would typically write these bytes to storage
        //     // For example: storage.write_chunk(&chunk).await?;
        // }

        // self.complete_media_upload(&uploaded_info).await?;
        todo!();
    }

    pub async fn get_media( ) {

    }

    pub async fn delete_media() {

    } 
}