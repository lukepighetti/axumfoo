use std::env;

use axum::{routing::get, Router, Server};
use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "hello world" }));

    let addr = "0.0.0.0:8000";
    println!("Serving on {}", addr);
    Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("We need a database to connect to");
    SqliteConnection::establish(&database_url).unwrap_or_else(|_| panic!("Couldn't connect to DB"))
}

#[derive(Queryable)]
struct ClassyLuke {
    pub catchphrase: String,
    pub glasses: Glasses,
    pub hat: Hat,
    pub left_earring: Earring,
    pub right_earring: Earring,
    pub sexiness: u8,
    pub suit: Suit,
}

enum Suit {
    Birthday,
    ThreePiece,
}

enum Hat {
    Beanie,
    None,
    Tophat,
}

enum Glasses {
    Bottle,
    None,
    Sunglasses,
}

enum Earring {
    Feather,
    Hoop,
    None,
}
