use axum::{
    routing::get,
    Router,
};

async fn say_hello() -> String{

    let hello: String = String::from("hola que tal");

    hello
    
}

async fn say_goodbye() -> String {
    let bye: String = String::from("Good Bye");

    bye 
}


#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().
    route("/", get(say_hello().await))
    .route("/path", get(say_goodbye().await));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}