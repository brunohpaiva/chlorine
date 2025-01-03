use askama_axum::{IntoResponse, Response};
use axum::{
    async_trait,
    extract::{FromRequest, Query, Request},
    http::{header::CONTENT_TYPE, StatusCode},
    Form, Json, RequestExt,
};

#[derive(serde::Deserialize, Debug)]
pub struct MalojaNewScrobbleReq {
    artist: Option<String>,
    artists: Option<Vec<String>>,
    title: String,
    album: Option<String>,
    albumartists: Option<Vec<String>>,
    duration: Option<u16>,
    length: Option<u16>,
    time: Option<u32>,
    nofix: Option<bool>,
}

// Maloja accepts both query params or body with json/form data.
pub async fn maloja_new_scrobble(
    params: Option<Query<MalojaNewScrobbleReq>>,
    payload: Option<JsonOrForm<MalojaNewScrobbleReq>>,
) -> Result<(), Response> {
    let req: MalojaNewScrobbleReq = match (params, payload) {
        (Some(params), None) => params.0,
        (None, Some(payload)) => payload.0,
        (None, None) => return Err(StatusCode::BAD_REQUEST.into_response()),
        // Accept either query params or body, not both.
        (Some(_), Some(_)) => return Err(StatusCode::BAD_REQUEST.into_response()),
    };

    dbg!(req);

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
