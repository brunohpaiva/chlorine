use crate::{AppState, db};
use askama::Template;
use askama_web::WebTemplate;
use std::sync::Arc;

use crate::db::scrobble::Scrobble;
use crate::routes::filters;
use axum::response::{IntoResponse, Response};
use axum::{extract::State, http::StatusCode};

// TODO: implement pagination
pub async fn get_scrobbles(
    State(state): State<Arc<AppState>>,
) -> Result<ScrobblesTemplate, Response> {
    let conn = state
        .pool
        .get()
        .await
        .inspect_err(|err| eprintln!("{:?}", err))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

    Ok(ScrobblesTemplate {
        scrobbles: db::scrobble::get_scrobbles(&conn, 100)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?,
    })
}

#[derive(Template, WebTemplate)]
#[template(path = "scrobbles.html")]
pub struct ScrobblesTemplate {
    scrobbles: Vec<Scrobble>,
}
