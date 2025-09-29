use std::sync::Arc;

use askama::Template;
use askama_web::WebTemplate;
use axum::response::{IntoResponse, Response};
use axum::{Router, extract::State, http::StatusCode, routing::get};
use deadpool_postgres::GenericClient;
use jiff::Timestamp;

use crate::AppState;
use crate::routes::filters;

pub fn build_router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_index))
}

async fn get_index(State(state): State<Arc<AppState>>) -> Result<IndexTemplate, Response> {
    let conn = state
        .pool
        .get()
        .await
        .inspect_err(|err| eprintln!("{:?}", err))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(IndexTemplate {
        recent_scrobbles: get_recent_scrobbles(&conn).await?,
        top_artists: get_top_artists(&conn, Period::AllTime).await?,
        top_tracks: get_top_tracks(&conn, Period::AllTime).await?,
    })
}

struct Scrobble {
    utc_timestamp: Timestamp,
    title: String,
    artist_names: String,
}

async fn get_recent_scrobbles<C: GenericClient>(conn: &C) -> Result<Vec<Scrobble>, Response> {
    let rows = conn
        .query(
            "
            SELECT s.utc_timestamp, t.title, tan.artist_names FROM scrobble s
            INNER JOIN track t ON s.track_id = t.id
            INNER JOIN track_artist_names tan ON t.id = tan.track_id
            ORDER BY s.utc_timestamp DESC
            LIMIT 10
            ",
            &[],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(rows
        .iter()
        .map(|row| Scrobble {
            utc_timestamp: row.get(0),
            title: row.get(1),
            artist_names: row.get(2),
        })
        .collect())
}

enum Period {
    Today,
    ThisWeek,
    ThisMonth,
    ThisYear,
    AllTime,
}

struct RankTrack {
    id: i32,
    title: String,
    slug: String,
    artist_names: String,
    scrobble_count: i64,
}

async fn get_top_tracks<C: GenericClient>(
    conn: &C,
    period: Period,
) -> Result<Vec<RankTrack>, Response> {
    // TODO: implement period filtering

    let rows = conn
        .query(
            "
            SELECT t.id, t.title, t.slug, tan.artist_names, 
            COUNT(s.utc_timestamp) AS scrobble_count FROM scrobble s
            INNER JOIN track t ON s.track_id = t.id
            INNER JOIN track_artist_names tan ON t.id = tan.track_id
            GROUP BY t.id, tan.artist_names
            ORDER BY scrobble_count DESC
            LIMIT 10
            ",
            &[],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(rows
        .iter()
        .map(|row| RankTrack {
            id: row.get(0),
            title: row.get(1),
            slug: row.get(2),
            artist_names: row.get(3),
            scrobble_count: row.get(4),
        })
        .collect())
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
struct IndexTemplate {
    recent_scrobbles: Vec<Scrobble>,
    top_artists: Vec<RankArtist>,
    top_tracks: Vec<RankTrack>,
}
