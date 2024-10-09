use std::{path::Path, sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}};

use crate::{error::Result, model::{Media, MediaUploadInfo}, Error};
use serde::{Serialize, Deserialize};
use sqlx::{postgres::PgRow, query, FromRow, PgPool, Row};

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
        let no_duplicates: bool = true;
        let media: Media = if no_duplicates {
            // Check for hash existing first
            // TODO! can i do this without 2 separate queries..
            let existing: Option<Media> = sqlx::query_as!(
                Media,
                "SELECT * FROM media
                WHERE file_hash=$1",
                info.file_hash
            )
                .fetch_optional(&self.db_pool).await
                .map_err(|_| Error::DatabaseQueryError)?;

            // Can't use unwrap_or, eagerly evaluated.
            if let Some(media) = existing {
                media
            } else {
                // Either returns existing or inserts a new one.
                sqlx::query_as!(
                    Media,
                    "INSERT INTO media (
                        uploaded_time,
                        accessed_time,
                        expiring_time,
                        file_size,
                        file_path,
                        file_hash
                    )
                    VALUES ($1, $2, $3, $4, $5, $6)
                    RETURNING *",
                    info.upload_start_time,
                    info.upload_start_time,
                    // TODO! expiring times maybe??
                    0,
                    info.file_size,
                    info.file_path,
                    info.file_hash
                )
                    .fetch_one(&self.db_pool).await
                    .map_err(|_| Error::DatabaseQueryError)?
            }
        } else {
            sqlx::query_as!(
                Media,
                "INSERT INTO media (
                    uploaded_time,
                    accessed_time,
                    expiring_time,
                    file_size,
                    file_path,
                    file_hash
                )
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING *",
                info.upload_start_time,
                info.upload_start_time,
                // TODO! expiring times maybe??
                0,
                info.file_size,
                info.file_path,
                info.file_hash
            )
                .fetch_one(&self.db_pool).await
                .map_err(|_| Error::DatabaseQueryError)?
        };
        

        Ok(media)
    }

    pub async fn get_media() {

    }

    pub async fn delete_media() {

    } 
}