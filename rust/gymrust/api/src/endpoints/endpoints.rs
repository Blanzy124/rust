use actix_web::{HttpResponse, Responder, get, http::StatusCode, post};

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().status(StatusCode(200)).body("Hola que tal")
}

#[get("/hola")]
pub async fn hola() -> impl Responder {
    HttpResponse::Ok().body("Hello world! 2")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}