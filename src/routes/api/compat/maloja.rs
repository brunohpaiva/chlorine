use std::{sync::Arc, vec};

use anyhow::Context;
use askama_axum::{IntoResponse, Response};
use axum::{
    extract::{Query, State},
    http::StatusCode,
};

use crate::{
    db::{insert_scrobble, NewScrobble},
    extractor::JsonOrForm,
    AppState,
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
    time: Option<i64>,
    nofix: Option<bool>,
}

impl From<MalojaNewScrobbleReq> for NewScrobble {
    fn from(value: MalojaNewScrobbleReq) -> Self {
        let mut track_artists = value.artist.map_or_else(|| vec![], |str| vec![str]);

        if let Some(mut other_track_artists) = value.artists {
            track_artists.append(&mut other_track_artists);
        }

        Self {
            track_title: value.title,
            track_artists,
        }
    }
}

// TODO: better error handling (of course)
// Maloja accepts both query params or body with json/form data.
pub async fn maloja_new_scrobble(
    State(state): State<Arc<AppState>>,
    query: Option<Query<MalojaNewScrobbleReq>>,
    body: Option<JsonOrForm<MalojaNewScrobbleReq>>,
) -> Result<(), Response> {
    let maloja_scrobble: MalojaNewScrobbleReq = match (query, body) {
        (Some(query), None) => query.0,
        (None, Some(body)) => body.0,
        // Reject if either both query and body are missing, or if both are present.
        (None, None) | (Some(_), Some(_)) => return Err(StatusCode::BAD_REQUEST.into_response()),
    };

    let new_scrobble: NewScrobble = maloja_scrobble.into();

    if new_scrobble.track_artists.is_empty() {
        return Err(StatusCode::BAD_REQUEST.into_response());
    }

    let mut conn = state
        .pool
        .get()
        .await
        .with_context(|| "couldn't get a connection from the pool")
        .inspect_err(|err| eprintln!("{:?}", err))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    insert_scrobble(&mut conn, &new_scrobble)
        .await
        .inspect_err(|err| eprintln!("{:?}", err))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(())
}
