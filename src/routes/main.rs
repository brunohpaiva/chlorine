mod artist;
mod index;

use std::sync::Arc;

use axum::{Router, routing::get};

use crate::AppState;

pub fn build_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(index::get_index))
        .route("/artist/{artist_id}", get(artist::get_artist))
}
