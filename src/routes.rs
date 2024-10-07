use std::io::Write;

use axum::{extract::{DefaultBodyLimit, Multipart, State}, routing::{delete, get, post}, Json, Router};
use bytes::{Buf, Bytes};
use config::Config;
use futures::{FutureExt, StreamExt, TryFutureExt};
use sha2::{Digest, Sha256};
use crate::{error::{Error, Result}, model::{Media, MediaUploadInfo}};

use crate::controller::MediaController;

pub fn routes(mc: MediaController) -> Router{
    Router::new()
        .route("/upload", post(upload).layer(
            // TODO! file size limit
            DefaultBodyLimit::disable()
        ))
        .route("/ping", get(ping))
        .with_state(mc)
}

async fn upload(
    State(mc): State<MediaController>,
    mut multipart: Multipart
) -> Result<Json<Media>> {

    // Write the first field to the disk, ignore other fields.
    if let Some(mut field) = multipart.next_field().await
        .map_err(|e| Error::AxumError { why: format!("Multipart error: {}", e.body_text()) })? {
        
        let name = field.name().unwrap().to_string();
        println!("Field name: {name}");

        let mut hasher = Sha256::new();

        while let Some(chunk) = field.chunk().await
            .map_err(|e| Error::AxumError { why: format!("Chunk error: {}", e.body_text()) })? {
            hasher.write(&chunk).expect("Failed to hash shit");
        }

        let hash = hasher.finalize();
        println!("{hash:?}");

        // mc.upload_media(info);
        // println          !("Length of `{}` is {} bytes",   name, data.len());
        // uploaded_media.push(
        //     UploadedMediaInfo {
        //         filename: name,
        //         hash: String::new()
        //     }
        // );

        let info = MediaUploadInfo {
            filename: String::new(),
            file_path: String::new(),
            file_size: 0
        };

        let uploaded_media = mc.checkin_media(info).await?;
        
        Ok(Json(uploaded_media))
    } else {
        // There were no fields.
        Err(Error::Error { why: "No content uploaded".to_string() })
    }
}

async fn ping() -> &'static str {
    "pong...?"
}           