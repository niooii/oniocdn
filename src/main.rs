mod error;
mod model;
mod routes;
mod controller;

use std::env;
        
use axum::{middleware, response::{IntoResponse, Response}, serve::serve, Json, Router};
use error::Error;
use controller::MediaController;
use serde_json::json;                   
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
        .nest("/cdn", routes::routes(mc))
        .layer(middleware::map_response(main_response_mapper));

    let listener = TcpListener::bind("0.0.0.0:9100").await.unwrap();

    println!("Initialization complete..");
    serve(listener, routes.into_make_service()).await.expect("Failed to start listening");
}

async fn main_response_mapper(res: Response) -> Response {
    let error = res.extensions().get::<Error>();

    let sc_and_ce = error
        .map(|e| e.to_status_and_client_error());

    let error_response = sc_and_ce
        .as_ref()
        .map(|(status_code, client_err)| {
            let err_json = serde_json::to_value(client_err);
            let body = err_json.unwrap_or(json!("Failed to get error information."));
            println!("{:?}", client_err);

            (*status_code, Json(body)).into_response()

        });

    error_response.unwrap_or(res)
}                                                                     