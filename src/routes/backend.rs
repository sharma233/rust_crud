use crate::api;
use axum::Router;
use std::sync::Arc;

pub fn backend_routes(shared_state:Arc<movie_backlog::AppState>) -> Router {
    Router::new()
        .merge(api::movie::movie_api_route(shared_state))
}
