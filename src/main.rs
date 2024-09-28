mod error;
mod model;
mod routes;

use std::env;
        
use axum::{serve::serve, Router};
use model::MediaController;
use sqlx::PgPool;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to get env variables from .env");

    let db_pool = PgPool::connect(
        env::var("DATABASE_URL").expect("Could not find DATABASE_URL in env").as_str()
    ).await.expect("Failed to connect to database");

    sqlx::migrate!("./migrations").run(&db_pool).await.expect("Failed to run migrations.");

    let mc = MediaController::new(db_pool);

    let routes = Router::new()
        .nest("/cdn", routes::routes(mc));

    let listener = TcpListener::bind("0.0.0.0:9100").await.unwrap();

    serve(listener, routes.into_make_service()).await.expect("Failed to start listening");
}
