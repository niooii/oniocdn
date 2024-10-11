mod error;
mod model;
mod routes;
mod controller;

use std::env;
use axum::{middleware, response::{IntoResponse, Response}, serve::serve, Json, Router};
use config::Config;
use error::Error;
use controller::MediaController;
use model::CdnSettings;
use serde_json::json;                   
use sqlx::PgPool;
use tokio::{net::TcpListener, sync::RwLock};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref cdn_settings: RwLock<CdnSettings> = {
        // Read configuration file
        let read_config = Config::builder()
            .add_source(config::File::with_name("./cdn_config"))
            .build()
            .expect("Unable to read config file data");

        RwLock::new(
            read_config.try_deserialize()
                .expect("Could not deserialize settings file.")
        )
    };
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to get env variables from .env");

    // Create file save directory if it doesn't exist already
    tokio::fs::create_dir_all(&cdn_settings.read().await.save_dir).await
        .expect("Failed to create save directory");

    let db_pool = PgPool::connect(
        env::var("DATABASE_URL").expect("Could not find DATABASE_URL in env").as_str()
    ).await.expect("Failed to connect to database");

    sqlx::migrate!("./migrations").run(&db_pool).await.expect("Failed to run migrations.");

    let mc = MediaController::new(db_pool);

    let routes = Router::new()
        .nest("", routes::routes(mc))
        .layer(middleware::map_response(main_response_mapper));

    let listener = TcpListener::bind("127.0.0.1:9100").await.unwrap();

    println!("Initialization complete..");
    serve(listener, routes.into_make_service()).await.expect("Failed to start listening");
}

async fn main_response_mapper(res: Response) -> Response {
    let error = res.extensions().get::<Error>();

    let sc_and_ce = error
        .map(|e| e.to_status_and_client_error());

    let error_response = sc_and_ce
        .as_ref()
        .map(
            |(status_code, client_err)| {
            let err_json = serde_json::to_value(client_err);
            let body = err_json.unwrap_or(json!("Failed to get error information."));
            println!("{:?}", client_err);

            (*status_code, Json(body)).into_response()
            }
        );

    error_response.unwrap_or(res)
}                                                                     