use std::io::Write;
use axum::{extract::{DefaultBodyLimit, Multipart, State}, routing::{delete, get, post}, Json, Router};
use bytes::{Buf, Bytes};
use tokio::{fs::File, io::AsyncWriteExt};
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
	    let mut file = File::create("./hichat.temp").await
            .map_err(|e| Error::IOError { why: e.to_string() } )?;
        let mut size = 0;
        while let Some(chunk) = field.chunk().await
            .map_err(|e| Error::AxumError { why: format!("Chunk error: {}", e.body_text()) })? {
            
            file.write_all(&chunk).await.map_err(|e| Error::IOError { why: e.to_string() } )?;
            size += chunk.len();
            hasher.write(&chunk).expect("Failed to hash shit");
        }

	    file.flush().await.expect("Bluh flushing file failed");

        let hash = hasher.finalize();
        println!("{hash:?}");

        let info = MediaUploadInfo {
            filename: String::new(),
            file_path: String::from("./hichat.temp"),
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