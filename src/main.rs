use axum::{
        routing::{get, post},
        response::{Redirect, Html},
        Form,Router,
        
};
use serde::Deserialize;
use sqlite;


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
    let mut html_list = "<ul>".to_string();

    connection
        .iterate(query, |pairs| {
            for &(name, value) in pairs.iter() {
                let ul_element = format!("<li> {} </li>", value.unwrap());
                html_list = String::new() + &html_list + &ul_element;
                println!("{} = {}", name, value.unwrap());
            }
            true
        }).unwrap();
        html_list = String::new() + &html_list + "</ul>";

    Html(
        "<form action=\"/\" method=\"POST\">
            <label for=\"item\">Item:</label>
            <input type=\"text\" name=\"item\">
            <br>
            <input type=\"submit\" value=\"Add!\">
        </form><br>".to_string() + &html_list
    )
}
