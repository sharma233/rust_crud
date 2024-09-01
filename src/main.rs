use axum::{
        routing::{get, post},
        response::{Redirect, Html},
        Form,Router,
        
};
use serde::{Serialize, Deserialize};
use rusqlite::{Connection, params};
use minijinja::{Environment, context};

fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_loader(minijinja::path_loader("templates"));
    env
}

#[derive(Deserialize)]
struct ItemPayload {
    item: String
}

#[derive(Serialize, Deserialize)]
struct Movie {
    id: i32,
    name: String
}

#[tokio::main]
async fn main() {
    //tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/", post(root_post));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_post(Form(item_payload): Form<ItemPayload>) -> Redirect {
    let conn = Connection::open("./backlog.db").unwrap();
    conn.execute(
        "INSERT INTO movies (name) VALUES (?1)",
        params![&item_payload.item],
    ).unwrap();

    Redirect::to("/")
}

async fn root() -> Html<String> {
    let env = make_env();
    let home_template = env.get_template("home.html").unwrap();

    let conn = Connection::open("./backlog.db").unwrap();
    let mut stmt = conn.prepare("SELECT rowid, name FROM movies").unwrap();
    let movies_iter = stmt.query_map([], |row| {
        Ok(Movie {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap()
        })
    }).unwrap();

    let mut movies:Vec<Movie> = Vec::new();
    for movie in movies_iter {
        movies.push(movie.unwrap());
    }

    let page = context! {
        backlog_items => movies 
    };

    Html(home_template.render(context!(page)).unwrap())

}
