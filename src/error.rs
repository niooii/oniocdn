#![allow(unreachable_patterns)]

use axum::{http::StatusCode, response::{IntoResponse, Response}};
use serde::Serialize;
use strum_macros::AsRefStr;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, AsRefStr)]
#[serde(tag = "type", content = "why")]
pub enum Error {
    AuthFailNoAuthToken,
    DatabaseConnectionError,
    DatabaseQueryError,
    AxumError { why: String }
}

#[derive(Clone, Debug, AsRefStr, Serialize)]
#[allow(non_camel_case_types)]
pub enum ClientError {
    NO_AUTH,
    BAD_REQUEST {why: String},
    INTERNAL_ERROR {why: String}
}

impl Error { 
    pub fn to_status_and_client_error(&self) -> (StatusCode, ClientError) {
        match self {    
            Self::AuthFailNoAuthToken => (
                StatusCode::FORBIDDEN,
                ClientError::NO_AUTH
            ),
            Self::DatabaseConnectionError | 
            Self::DatabaseQueryError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::INTERNAL_ERROR { why: "Internal database error.".to_string() }
            ),
            Self::AxumError { why } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::INTERNAL_ERROR { why: why.clone() }
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::INTERNAL_ERROR { why: "Something went wrong...".to_string() }
            )
        }
    }
}               

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        response.extensions_mut().insert(self);

        response
    }
}
