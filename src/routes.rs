use axum::{extract::{DefaultBodyLimit, Multipart, State}, routing::{delete, get, post}, Json, Router};
use config::Config;
use crate::error::{Result, Error};

use crate::model::MediaController;

pub fn routes(mc: MediaController) -> Router{
    Router::new()
        .route("/upload", post(upload).layer(
            // TODO! file size limit
            DefaultBodyLimit::max(0)
        ))
        .route("/ping", get(ping))
        .with_state(mc)
}

async fn upload(
    State(_mc): State<MediaController>,
    mut multipart: Multipart
) -> Result<()> {
    while let Some(mut field) = multipart.next_field().await
        .map_err(|e| Error::AxumError { why: format!("Multipart error: {}", e.body_text()) })? {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }

    Ok(()) 
}

async fn ping() -> &'static str {
    "pong...?"
}           