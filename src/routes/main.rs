use std::sync::Arc;

use askama::Template;
use askama_web::WebTemplate;
use axum::response::{IntoResponse, Response};
use axum::{Router, extract::State, http::StatusCode, routing::get};
use jiff::Timestamp;

use crate::AppState;
use crate::routes::filters;

pub fn build_router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_index))
}

#[axum::debug_handler]
async fn get_index(State(state): State<Arc<AppState>>) -> Result<IndexTemplate, Response> {
    let conn = state
        .pool
        .get()
        .await
        .inspect_err(|err| eprintln!("{:?}", err))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    let rows = conn
        .query(
            "
            SELECT s.utc_date_time, t.title, ma.name AS main_artist_name, ma.slug AS main_artist_slug FROM scrobble s
            INNER JOIN track t ON s.track_id = t.id
            INNER JOIN artist ma ON t.main_artist_id = ma.id
            LEFT JOIN track_artist ta ON t.id = ta.track_id
            LEFT JOIN artist oa ON ta.artist_id = oa.id
            ORDER BY s.utc_date_time DESC
            LIMIT 10
            ",
            &[],
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    let scrobbles: Vec<Scrobble> = rows
        .iter()
        .map(|row| Scrobble {
            utc_date_time: row.get(0),
            title: row.get(1),
            main_artist_name: row.get(2),
            main_artist_slug: row.get(3),
        })
        .collect();

    Ok(IndexTemplate {
        recent_scrobbles: scrobbles,
    })
}

struct Scrobble {
    utc_date_time: Timestamp,
    title: String,
    main_artist_name: String,
    main_artist_slug: String,
}

#[derive(Template, WebTemplate)]
#[template(path = "index.html")]
struct IndexTemplate {
    recent_scrobbles: Vec<Scrobble>,
}
