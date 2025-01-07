use std::sync::Arc;

use askama::Template;
use axum::{routing::get, Router};

use crate::AppState;

pub fn build_router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_index))
}

async fn get_index() -> IndexTemplate {
    IndexTemplate {}
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;
