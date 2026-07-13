use actix_web::{ web, App, HttpServer};
use actix_files::Files;
mod endpoints;


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .service(Files::new("/", "landing_files").index_file("index.html"))
            .service(endpoints::endpoints::echo)
            .service(endpoints::endpoints::hola)
            //.route("/hey", web::get().to(endpoints::endpoints::manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
} 