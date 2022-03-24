use actix_web::{App, HttpServer};

mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::test::test)
            .service(routes::test::echo)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
