use std::sync::Arc;

use axum::Router;

use crate::AppState;

mod compat;

pub fn build_router() -> Router<Arc<AppState>> {
    Router::new().merge(compat::build_router())
}
