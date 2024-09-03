use axum::{
    routing::delete,
    Router,
    extract::Path,
    extract::State,
};
use std::sync::Arc;
use serde::{Serialize, Deserialize};
use rusqlite::params;

#[derive(Serialize, Deserialize)]
pub struct Movie {
    pub id: i32,
    pub name: String,
    pub tmdb_id: i32
}

pub fn movie_api_route(shared_state:Arc<movie_backlog::AppState>) -> Router {
    Router::new()
        .route("/movie/:movie_id", delete(delete_movie))
        .with_state(shared_state)
}


async fn delete_movie(State(state): State<Arc<movie_backlog::AppState>>, Path(movie_id): Path<String>) -> String {
    state.conn_pool.get().unwrap().execute(
        "DELETE FROM movies WHERE rowid=(?1)",
        params![movie_id],
    ).unwrap();

    return movie_id;
}
