use serde::{Serialize, Deserialize};

use axum::{
    Json, Router, extract::rejection::JsonDataError, routing::get
};
use serde_json::json;

async fn say_hello(j: String) -> String{

    let hello: String = j;

    hello
    
}



async fn say_goodbye() -> String {
    let bye: String = String::from("Good Bye");

    bye 
}



#[tokio::main]
async fn main() {
    #[derive(Serialize, Deserialize)]
    struct yarn{
        name: String,
        size: i64
    }

    let mut y: yarn = yarn{name: String::from("cotton"), size: 30/1};

    let j: String = serde_json::to_string(&y).unwrap();

    // build our application with a single route
    let app = Router::new().
    route("/", get(say_hello(j).await))
    .route("/path", get(say_goodbye().await));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}