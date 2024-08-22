use axum::{
        routing::{get, post},
        response::{Redirect, Html},
        Form,Router,
        
};
use serde::Deserialize;
use sqlite;
use minijinja::render;

const HOME_TEMPLATE:&str = r#"
    <form action="/" method="POST">
        <label for="item">Item:</label>
        <input type="text" name="item">
        <br>
        <input type="submit" value="Add!">
    </form><br>
    <ul>
        {% for item in backlog_items %}
        <li>{{ item }}</li>
        {% endfor %}
    <ul>
"#;

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

    let r = render!(HOME_TEMPLATE, backlog_items => items);
    Html(r)
}
