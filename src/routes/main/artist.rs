use crate::AppState;
use crate::db::artist::get_artist_name;
use crate::db::track::{RankTrack, get_top_tracks};
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::sync::Arc;

pub async fn get_artist(
    State(state): State<Arc<AppState>>,
    Path(artist_id): Path<i32>,
) -> Result<ArtistTemplate, Response> {
    let conn = state
        .pool
        .get()
        .await
        .inspect_err(|err| eprintln!("{:?}", err))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    let artist_name = match get_artist_name(&conn, artist_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?
    {
        None => return Err(StatusCode::NOT_FOUND.into_response()),
        Some(artist_name) => artist_name,
    };

    Ok(ArtistTemplate {
        artist_name,
        top_tracks: get_top_tracks(&conn, Some(artist_id))
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?,
    })
}

#[derive(Template, WebTemplate)]
#[template(path = "artist.html")]
pub struct ArtistTemplate {
    artist_name: String,
    top_tracks: Vec<RankTrack>,
}
