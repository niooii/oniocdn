use axum::{routing::{delete, get, post}, Json, Router};

use crate::model::MediaController;

pub fn routes(mc: MediaController) -> Router{
    Router::new()
        .route("/upload", post(upload))
        .route("/ping", get(ping))
        .with_state(mc)
}

async fn upload() {

}

async fn ping() -> &'static str {
    "pong...?"
}