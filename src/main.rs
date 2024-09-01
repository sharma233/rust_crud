use axum::Router;
use std::sync::Arc;
use r2d2_sqlite::SqliteConnectionManager;

mod api;
mod routes;



#[tokio::main]
async fn main() { 
    //tracing_subscriber::fmt::init();
    let manager = SqliteConnectionManager::file("./backlog.db");
    let pool = r2d2::Pool::new(manager).unwrap();

    let shared_state = Arc::new(movie_backlog::AppState {
        conn_pool: pool
    });

    let app = Router::new()
        .merge(routes::frontend::frontend_routes(shared_state.clone()))
        .merge(routes::backend::backend_routes(shared_state.clone()));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


