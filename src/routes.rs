use std::{io::Write, path::PathBuf, time::{SystemTime, UNIX_EPOCH}};
use axum::{body::Body, extract::{DefaultBodyLimit, Multipart, Path, Query, State}, http::{header, HeaderValue}, response::Response, routing::{get, post}, Router};
use serde::Deserialize;
use tokio::{fs::File, io::AsyncWriteExt};
use sha2::{Digest, Sha256};
use tokio_util::io::ReaderStream;
use crate::{cdn_settings, error::{Error, Result}, model::{Media, MediaRequestInfo, MediaUploadInfo}};

use crate::controller::MediaController;

pub fn routes(mc: MediaController) -> Router{
    Router::new()
        .route(
            "/media", post(upload_media).delete(delete_media).layer(
            // TODO! file size limit
            DefaultBodyLimit::disable()
        ))
        .route("/media/:file_name", get(get_media))
        .route("/ping", get(ping))
        .with_state(mc)
}

async fn upload_media(
    State(mc): State<MediaController>,
    mut multipart: Multipart
) -> Result<String> {
    
    // Write the first field to the disk, ignore other fields.
    if let Some(mut field) = multipart.next_field().await
    .map_err(|e| Error::AxumError { why: format!("Multipart error: {}", e.body_text()) })? {
        
        let save_dir = PathBuf::from(&cdn_settings.read().await.save_dir);
        
        // Name should be the name of the file, including the extension.
        let name: String = field.name().expect("File has no name??").to_string();
        println!("Got file: {name}");

        let uploaded_time: i64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time itself is against you today, it seems..")
            .as_millis() as i64;

        let mut hasher = Sha256::new();

        let temp_path: PathBuf = save_dir.join(
            format!("./tmp_{uploaded_time}_{name}")
        );

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

        let info = MediaUploadInfo {
            file_name: name,
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
            // No duplicate
            // Rename the file to have the proper filename
            let true_path: PathBuf = save_dir.join(
                uploaded_media.true_filename()
            );

            tokio::fs::rename(
                temp_path.as_path(), 
                true_path.as_path()
            ).await.map_err(|e| Error::IOError { why: e.to_string() })?;

            println!("Finalized filename..");  
        }

        Ok(uploaded_media.rel_endpoint())
    } else {
        // There were no fields.
        Err(Error::Error { why: "No content uploaded".to_string() })
    }
}

#[derive(Deserialize)]
struct GetMediaQueryParams {
    id: i64,
    checksum: String
}

async fn get_media(
    State(mc): State<MediaController>,
    Path(file_name): Path<String>,
    Query(q_params): Query<GetMediaQueryParams>
) -> Result<Response> {

    let req_info = MediaRequestInfo {
        id: q_params.id,
        file_hash: q_params.checksum,
        file_name: file_name.clone()
    };

    let media: Media = if let Some(m) = mc.get_media(&req_info).await? {
        m
    } else {
        return Err(Error::NoMediaFound)
    };

    let file = File::open(&media.true_path().await)
        .await.expect("Could not open file");
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let mut res = Response::new(body);
    
    let mime_type = mime_guess::from_path(&file_name).first_raw().unwrap_or("application/octet-stream");
    res.headers_mut().append(
        header::CONTENT_TYPE, 
        HeaderValue::from_static(
            mime_type
        )
    );
    
    // Get that built-in media player thingy
    if !mime_type.starts_with("video/") && 
       !mime_type.starts_with("audio/") && 
       !mime_type.starts_with("image/") {
        res.headers_mut().append(
            header::CONTENT_DISPOSITION,
            HeaderValue::from_str(
                &format!("attachment; filename=\"{}\"", file_name)
            ).map_err(|_e| Error::Error { why: "Parse error".to_string() })?
        );
    }

    Ok(res)
}

async fn delete_media() {

}

async fn ping() -> &'static str {
    "pong...?"
}           
