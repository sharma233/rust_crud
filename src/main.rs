use axum::{
        routing::{get, post},
        response::{Redirect, Html},
        Form,Router,
        
};
use serde::Deserialize;
use sqlite;
use minijinja::{Environment, context};

fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_loader(minijinja::path_loader("templates"));
    env
}

#[derive(Deserialize, Debug)]
struct ItemPayload {
    item: String,
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
    let connection:sqlite::Connection = sqlite::open("./backlog.db").unwrap();
    let query = format!("
        INSERT INTO movies VALUES (\'{}\');
    ", &item_payload.item);
    connection.execute(query).unwrap();

    Redirect::to("/")
}

async fn root() -> Html<String> {
    let env = make_env();
    let home_template = env.get_template("home.html").unwrap();

    let connection:sqlite::Connection = sqlite::open("./backlog.db").unwrap();
    let query = "SELECT name FROM movies";

    let mut items:Vec<String> = vec![];
    connection
        .iterate(query, |pairs| {
            for &(name, value) in pairs.iter() {
                items.push(String::from(value.unwrap()));
                println!("{} = {}", name, value.unwrap());
            }
            true
        }).unwrap();

    let page = context! {
        backlog_items => items
    };

    Html(home_template.render(context!(page)).unwrap())

}
