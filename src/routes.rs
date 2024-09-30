use axum::{extract::{DefaultBodyLimit, Multipart, State}, routing::{delete, get, post}, Json, Router};
use bytes::{Buf, Bytes};
use config::Config;
use futures::{FutureExt, StreamExt, TryFutureExt};
use crate::{error::{Error, Result}, model::{MediaUploadInfo, UploadedMediaInfo}};

use crate::model::MediaController;

pub fn routes(mc: MediaController) -> Router{
    Router::new()
        .route("/upload", post(upload).layer(
            // TODO! file size limit
            DefaultBodyLimit::disable()
        ))
        .route("/ping", get(ping))
        .with_state(mc)
}
use tokio_util::io::StreamReader;
async fn upload(
    State(mc): State<MediaController>,
    mut multipart: Multipart
) -> Result<Json<Vec<UploadedMediaInfo>>> {
    let mut uploaded_media: Vec<UploadedMediaInfo> = Vec::new();

    while let Some(field) = multipart.next_field().await
        .map_err(|e| Error::AxumError { why: format!("Multipart error: {}", e.body_text()) })? {
        let name = field.name().unwrap().to_string();
        println!("Field name: {name}");
        
        let stream = field.bytes().into_stream();
        futures::pin_mut!(stream);
        while let Some(chunk) = stream.next().await {
            println!("got some bytes: {:?}", chunk);
        }

        futures::pin_mut!();
        
        todo!();
        // mc.upload_media(info);
        // println!("Length of `{}` is {} bytes",   name, data.len());
        // uploaded_media.push(
        //     UploadedMediaInfo {
        //         filename: name,
        //         hash: String::new()
        //     }
        // );
    }

    Ok(
        Json(uploaded_media)
    )
}

async fn ping() -> &'static str {
    "pong...?"
}           