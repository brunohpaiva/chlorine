use std::sync::Arc;

use axum::{routing::post, Router};

use crate::AppState;

mod compat;

pub fn build_router() -> Router<Arc<AppState>> {
    Router::new()
        // Maloja compat
        .route(
            "/apis/mlj_1/newscrobble",
            post(compat::maloja::maloja_new_scrobble),
        )
}
