use std::{path::Path, sync::{Arc, Mutex}, time::{SystemTime, UNIX_EPOCH}};

use crate::{error::Result, model::{Media, MediaRequestInfo, MediaUploadInfo}, Error};
use serde::{Serialize, Deserialize};
use sqlx::{postgres::PgRow, query, FromRow, PgPool, Row};

#[derive(Clone)]
pub struct MediaController {
    db_pool: PgPool,
}

impl MediaController {
    pub fn new(db_pool: PgPool) -> Self {
        Self {
            db_pool
        }
    }
}

async fn insert_info(db_pool: &PgPool, info: &MediaUploadInfo) -> Result<Media> {
    sqlx::query_as!(
        Media,
        "INSERT INTO media (
            uploaded_time,
            accessed_time,
            expiring_time,
            file_size,
            file_name,
            file_hash
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *",
        info.upload_start_time,
        info.upload_start_time,
        // TODO! expiring times maybe??
        0,
        info.file_size, 
        info.file_name,
        info.file_hash
    )
        .fetch_one(db_pool).await
        .map_err(|_| Error::DatabaseQueryError)
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
            // Either returns existing or inserts a new one.
            if let Some(media) = existing {
                media
            } else {
                insert_info(&self.db_pool, &info).await?
            }
        } else {
            insert_info(&self.db_pool, &info).await?
        };

        Ok(media)
    }

    pub async fn get_media(&self, info: &MediaRequestInfo) -> Result<Option<Media>> {
        Ok(
            sqlx::query_as!(
                Media,
                "SELECT * FROM media
                WHERE id = $1 AND file_hash = $2",
                info.id,
                info.file_hash
            )
            .fetch_optional(&self.db_pool).await
            .map_err(|_| Error::DatabaseQueryError)?
        )
    }

    pub async fn delete_media() {

    } 
}