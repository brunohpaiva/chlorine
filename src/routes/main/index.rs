use crate::AppState;
use askama::Template;
use askama_web::WebTemplate;
use std::sync::Arc;

use crate::db::scrobble::{Scrobble, get_scrobbles};
use crate::db::track::{RankTrack, get_top_tracks};
use crate::routes::filters;
use axum::response::{IntoResponse, Response};
use axum::{extract::State, http::StatusCode};
use deadpool_postgres::GenericClient;

pub async fn get_index(State(state): State<Arc<AppState>>) -> Result<IndexTemplate, Response> {
    let conn = state
        .pool
        .get()
        .await
        .inspect_err(|err| eprintln!("{:?}", err))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(IndexTemplate {
        recent_scrobbles: get_scrobbles(&conn, 10)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?,
        top_artists: get_top_artists(&conn, Period::AllTime).await?,
        top_tracks: get_top_tracks(&conn, None)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?,
    })
}

enum Period {
    Today,
    ThisWeek,
    ThisMonth,
    ThisYear,
    AllTime,
}

struct RankArtist {
    id: i32,
    name: String,
    slug: String,
    scrobble_count: i64,
}

async fn get_top_artists<C: GenericClient>(
    conn: &C,
    period: Period,
) -> Result<Vec<RankArtist>, Response> {
    // TODO: implement period filtering

    let rows = conn
        .query(
            "
            SELECT a.id, a.name, a.slug, COUNT(s.utc_timestamp) AS scrobble_count FROM scrobble s
            INNER JOIN track t ON s.track_id = t.id
            INNER JOIN track_artist ta ON t.id = ta.track_id
            INNER JOIN artist a ON ta.artist_id = a.id
            GROUP BY a.id
            ORDER BY scrobble_count DESC
            LIMIT 10
            ",
            &[],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(rows
        .iter()
        .map(|row| RankArtist {
            id: row.get(0),
            name: row.get(1),
            slug: row.get(2),
            scrobble_count: row.get(3),
        })
        .collect())
}

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    recent_scrobbles: Vec<Scrobble>,
    top_artists: Vec<RankArtist>,
    top_tracks: Vec<RankTrack>,
}
