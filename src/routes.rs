use std::{io::Write, path::{Path, PathBuf}, time::{SystemTime, UNIX_EPOCH}};
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
        
        // Name should be the name of the file, including the extension.
        let name: String = field.name().unwrap().to_string();
        println!("Got file: {name}");

        let uploaded_time: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time itself is against you today, it seems..")
            .as_millis() as i64;

        let mut hasher = Sha256::new();

        let temp_path: PathBuf = PathBuf::from(format!("./tmp_{uploaded_time}_{name}"));

	    let mut file: File = File::create(&temp_path).await
            .map_err(|e| Error::IOError { why: e.to_string() } )?;
        // i64 type because postgres doesnt support unsigned gg
        let mut file_size: i64 = 0;
        while let Some(chunk) = field.chunk().await
            .map_err(|e| Error::AxumError { why: format!("Chunk error: {}", e.body_text()) })? {
            
            file.write_all(&chunk).await.map_err(|e| Error::IOError { why: e.to_string() } )?;
            file_size += chunk.len() as i64;
            hasher.write(&chunk).expect("Failed to hash shit");
        }

	    file.flush().await.expect("Bluh flushing file failed");

        let hash = hasher.finalize();
        let file_hash: String = format!("{:X}", hash);  
        println!("{file_hash}");    

        let info = MediaUploadInfo {
            file_path: temp_path.to_string_lossy().into_owned(),
            file_size,
            file_hash,
            upload_start_time: uploaded_time
        };
        
        // Check-in file to database
        let uploaded_media: Media = mc.checkin_media(info).await?;

        // Ensure the file handle is dropped before doing anything
        // ahem windows
        drop(file);

        // If the current upload_time is different 
        // from the returned media upload time, then the recieved file was
        // a duplicate of another and we can discard the new downloaded file.

        if uploaded_time != uploaded_media.uploaded_time {
            tokio::fs::remove_file(&temp_path).await
                .map_err(|e| Error::IOError { why: e.to_string() })?;
            println!("Removed duplicate file..")
        } else {
            // No duplicate!
            // Rename the file to have updated data
            let mut save_path: PathBuf = temp_path.clone();
            save_path.set_file_name(
                format!(
                    "{}_{}_{}", 
                    uploaded_media.id, 
                    uploaded_media.uploaded_time, 
                    name
                )
            );

            tokio::fs::rename(
                &temp_path.as_path(), 
                &save_path.as_path()
            ).await.map_err(|e| Error::IOError { why: e.to_string() })?;

            println!("Finalized filename..");  
        }

        Ok(Json(uploaded_media))
    } else {
        // There were no fields.
        Err(Error::Error { why: "No content uploaded".to_string() })
    }
}

async fn ping() -> &'static str {
    "pong...?"
}           