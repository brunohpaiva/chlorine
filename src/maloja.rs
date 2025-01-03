use std::sync::Arc;

use askama_axum::{IntoResponse, Response};
use axum::{
    async_trait,
    extract::{FromRequest, Query, Request, State},
    http::{header::CONTENT_TYPE, StatusCode},
    Form, Json, RequestExt,
};

use crate::AppState;

#[derive(serde::Deserialize, Debug)]
pub struct MalojaNewScrobbleReq {
    artist: Option<String>,
    artists: Option<Vec<String>>,
    title: String,
    album: Option<String>,
    albumartists: Option<Vec<String>>,
    duration: Option<u16>,
    length: Option<u16>,
    time: Option<i64>,
    nofix: Option<bool>,
}

// Maloja accepts both query params or body with json/form data.
pub async fn maloja_new_scrobble(
    State(state): State<Arc<AppState>>,
    query: Option<Query<MalojaNewScrobbleReq>>,
    body: Option<JsonOrForm<MalojaNewScrobbleReq>>,
) -> Result<(), Response> {
    let scrobble_data: MalojaNewScrobbleReq = match (query, body) {
        (Some(query), None) => query.0,
        (None, Some(body)) => body.0,
        // Reject if either both query and body are missing, or if both are present.
        (None, None) | (Some(_), Some(_)) => return Err(StatusCode::BAD_REQUEST.into_response()),
    };

    let _conn = state
        .pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    dbg!(scrobble_data);

    Ok(())
}

#[derive(Debug)]
pub struct JsonOrForm<T>(T);

#[async_trait]
impl<S, T> FromRequest<S> for JsonOrForm<T>
where
    S: Send + Sync,
    Json<T>: FromRequest<()>,
    Form<T>: FromRequest<()>,
    T: 'static,
{
    type Rejection = Response;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let content_type_header = req.headers().get(CONTENT_TYPE);
        let content_type = content_type_header.and_then(|value| value.to_str().ok());

        if let Some(content_type) = content_type {
            if content_type.starts_with("application/json") {
                let Json(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
                return Ok(Self(payload));
            }

            if content_type.starts_with("application/x-www-form-urlencoded") {
                let Form(payload) = req.extract().await.map_err(IntoResponse::into_response)?;
                return Ok(Self(payload));
            }
        }

        Err(StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response())
    }
}
