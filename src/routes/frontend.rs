use axum::{
        routing::{get, post},
        response::{Redirect, Html},
        Form,Router,
        extract::{Query, State, Json}
};
use serde::{Serialize, Deserialize};
use rusqlite::params;
use minijinja::{Environment, context};
use std::sync::Arc;
use urlencoding::encode;
use crate::api;
use crate::thirdparty;

#[derive(Deserialize)]
struct ItemPayload {
    item: String
}

pub fn frontend_routes(shared_state:Arc<movie_backlog::AppState>) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/add_movie", post(add_movie))
        .route("/", post(root_post))
        .route("/search", get(search))
        .with_state(shared_state)
}

fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_loader(minijinja::path_loader("templates"));
    env
}

#[derive(Deserialize)]
struct AddMoviePayload {
    id: u32,
    name: String
}

async fn add_movie(State(state): State<Arc<movie_backlog::AppState>>, Json(payload): Json<AddMoviePayload>) -> Redirect {
    state.conn_pool.get().unwrap().execute(
        "INSERT INTO movies (tmdb_id, name) VALUES (?1, ?2)",
        params![payload.id, payload.name],
    ).unwrap();

    Redirect::to("/")
}

async fn root_post(State(_state): State<Arc<movie_backlog::AppState>>, Form(item_payload): Form<ItemPayload>) -> Redirect {
    let url = format!("/search?query={search_query}&page=1", search_query=item_payload.item);
    Redirect::to(&url)
}

#[derive(Deserialize)]
struct SearchQueryParams {
    page: u32,
    query: String
}

#[derive(Serialize)]
struct SearchResultWithContext {
    api_search_results: thirdparty::tmdb::SearchResultWrapper,
    current_page: u32,
    current_query: String
}

async fn search(State(_state): State<Arc<movie_backlog::AppState>>, query_params: Query<SearchQueryParams>) -> Html<String>{
    let query_params = query_params.0;
    let search_query = encode(&query_params.query);
    let search_results = crate::thirdparty::tmdb::search(&search_query.to_string(), query_params.page).await;

    let Ok(search_results) = search_results else {
        return render_error();
    };

    let results_with_context = SearchResultWithContext {
        api_search_results: search_results, 
        current_page: query_params.page,
        current_query: search_query.to_string()
    };

    let page = context! {
        results => results_with_context
    };

    let env = make_env();
    let search_template = env.get_template("search.html").unwrap();

    Html(search_template.render(context!(page)).unwrap())
}

async fn root(State(state): State<Arc<movie_backlog::AppState>>) -> Html<String> {
    let env = make_env();
    let home_template = env.get_template("home.html").unwrap();

    let conn = state.conn_pool.get();
    let Ok(conn) = conn else {
        return render_error();
    };

    let mut stmt = conn.prepare("SELECT rowid, name, tmdb_id FROM movies").unwrap();

    let movies_iter = stmt.query_map([], |row| {
        Ok(api::movie::Movie {
            id: row.get(0)?,
            name: row.get(1)?,
            tmdb_id: row.get(2)?
        })
    });

    let Ok(movies_iter) = movies_iter else {
        return render_error();
    };

    let mut movies:Vec<api::movie::Movie> = Vec::new();
    for movie in movies_iter {
        movies.push(movie.unwrap());
    }

    let page = context! {
        backlog_items => movies 
    };

    Html(home_template.render(context!(page)).unwrap())
}

fn render_error() -> Html<String> {
    let env = make_env();
    let error_template = env.get_template("error.html").unwrap();
    Html(error_template.render(()).unwrap())
}
