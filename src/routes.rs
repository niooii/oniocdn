use axum::{routing::{post, get, delete}, Router};

use crate::model::MediaController;

pub fn routes(mc: MediaController) -> Router{
    Router::new()
    .route("/upload", post(upload))
    .with_state(mc      )
}

async fn upload() {

}