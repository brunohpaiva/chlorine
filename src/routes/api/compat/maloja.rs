use std::{sync::Arc, vec};

use crate::parser::parse_artists;
use crate::{
    AppState,
    db::scrobble::{NewScrobble, insert_scrobble},
    extractor::JsonOrForm,
};
use anyhow::Context;
use axum::{
    Json,
    extract::{
        Query, State,
        rejection::{JsonRejection, QueryRejection},
    },
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;

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

impl TryFrom<MalojaNewScrobbleReq> for NewScrobble {
    type Error = anyhow::Error;
    fn try_from(value: MalojaNewScrobbleReq) -> Result<Self, Self::Error> {
        // FIXME: oh god this is a mess

        let mut track_artists: Vec<String> = vec![];

        if let Some(artist) = value.artist {
            // FIXME: this looks really stupid
            let mut parsed_artists = parse_artists(&artist)?
                .into_iter()
                .map(|s| s.to_string())
                .collect();
            track_artists.append(&mut parsed_artists);
        }

        if let Some(other_track_artists) = value.artists {
            for artist in other_track_artists {
                // FIXME: this looks really stupid x2
                let mut parsed_artists = parse_artists(&artist)?
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect();
                track_artists.append(&mut parsed_artists);
            }
        }

        let album_artists: Option<Vec<String>> = if let Some(unparsed_album_artists) = value.albumartists {
            let mut album_artists = vec![];

            for artist in unparsed_album_artists {
                // FIXME: this looks really stupid x3
                let mut parsed_artists = parse_artists(&artist)?
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect();
                album_artists.append(&mut parsed_artists);
            }

            Some(album_artists)
        } else {
            None
        };

        let utc_timestamp = if let Some(time) = value.time {
            Some(jiff::Timestamp::new(time, 0)?)
        } else {
            None
        };

        Ok(Self {
            utc_timestamp,
            track_title: value.title,
            track_artists,
            album_title: value.album,
            album_artists,
        })
    }
}

// TODO: auth
// TODO: better error handling (of course)
// Maloja accepts both query params or body with json/form data.
pub async fn maloja_new_scrobble(
    State(state): State<Arc<AppState>>,
    query: Result<Query<MalojaNewScrobbleReq>, QueryRejection>,
    body: Option<JsonOrForm<MalojaNewScrobbleReq>>,
) -> Result<(), Response> {
    let maloja_scrobble: MalojaNewScrobbleReq = match (query, body) {
        (Ok(query), None) => query.0,
        (Err(_), Some(body)) => body.0,
        // Reject if either both query and body are missing, or if both are present.
        (Err(_), None) | (Ok(_), Some(_)) => return Err(StatusCode::BAD_REQUEST.into_response()),
    };

    let new_scrobble: NewScrobble = maloja_scrobble
        .try_into()
        .map_err(|_| StatusCode::BAD_REQUEST.into_response())?;

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

#[derive(serde::Deserialize, Debug)]
pub struct MalojaTestParamsReq {
    key: String,
}

pub async fn maloja_test(
    State(_state): State<Arc<AppState>>,
    query: Result<Query<MalojaTestParamsReq>, QueryRejection>,
    // Maloja also accepts form urlencoded data for this GET endpoint,
    // but that is not supported by the Form axum extractor and kinda invalid HTTP, so we ignore it.
    body: Result<Json<MalojaTestParamsReq>, JsonRejection>,
) -> impl IntoResponse {
    let params = match (query, body) {
        (Ok(query), Err(_)) => Some(query.0),
        (Err(_), Ok(body)) => Some(body.0),
        // Reject if either both query and body are missing, or if both are present.
        (Err(_), Err(_)) | (Ok(_), Ok(_)) => None,
    };

    if let Some(_key) = params.and_then(|p| Some(p.key)) {
        // TODO: check if key is valid
        // return (StatusCode::FORBIDDEN, Json(json!({"status": "error", "error": "Wrong API key"})))
    }

    Json(json!({"status": "ok"}))
}
