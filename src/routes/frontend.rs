use axum::{
        routing::{get, post},
        response::{Redirect, Html},
        Form,Router,
        extract::State
};
use serde::Deserialize;
use rusqlite::params;
use minijinja::{Environment, context};
use std::sync::Arc;
use crate::api;

#[derive(Deserialize)]
struct ItemPayload {
    item: String
}

pub fn frontend_routes(shared_state:Arc<movie_backlog::AppState>) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/", post(root_post))
        .with_state(shared_state)
}

fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_loader(minijinja::path_loader("templates"));
    env
}

async fn root_post(State(state): State<Arc<movie_backlog::AppState>>, Form(item_payload): Form<ItemPayload>) -> Redirect {
    state.conn_pool.get().unwrap().execute(
        "INSERT INTO movies (name) VALUES (?1)",
        params![&item_payload.item],
    ).unwrap();

    Redirect::to("/")
}

async fn root(State(state): State<Arc<movie_backlog::AppState>>) -> Html<String> {
    let env = make_env();
    let home_template = env.get_template("home.html").unwrap();

    let conn = state.conn_pool.get().unwrap();
    let mut stmt = conn.prepare("SELECT rowid, name FROM movies").unwrap();

    let movies_iter = stmt.query_map([], |row| {
        Ok(api::movie::Movie {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap()
        })
    }).unwrap();

    let mut movies:Vec<api::movie::Movie> = Vec::new();
    for movie in movies_iter {
        movies.push(movie.unwrap());
    }

    let page = context! {
        backlog_items => movies 
    };

    Html(home_template.render(context!(page)).unwrap())
}
