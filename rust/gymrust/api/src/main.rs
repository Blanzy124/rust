mod endpoints;



use actix_web::{ web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(endpoints::endpoints::hello)
            .service(endpoints::endpoints::echo)
            .service(endpoints::endpoints::hola)
            .route("/hey", web::get().to(endpoints::endpoints::manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
} 