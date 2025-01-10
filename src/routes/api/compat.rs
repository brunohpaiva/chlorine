use std::sync::Arc;

use axum::{routing::post, Router};

use crate::AppState;

mod maloja;

pub fn build_router() -> Router<Arc<AppState>> {
    Router::new()
        // Maloja
        .route("/apis/mlj_1/newscrobble", post(maloja::maloja_new_scrobble))
}
