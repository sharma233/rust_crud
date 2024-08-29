use axum::{
        routing::{get, post},
        response::{Redirect, Html},
        Form,Router,
        
};
use serde::Deserialize;
use sqlite;
use minijinja::render;
use std::fs;

const TEMPLATE_PATH:&str = "templates";

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
    let home_template_path:String = String::from(TEMPLATE_PATH) + "/home.html";
    let home_template = fs::read_to_string(home_template_path).unwrap();
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

    let r = render!(&home_template, backlog_items => items);
    Html(r)
}
