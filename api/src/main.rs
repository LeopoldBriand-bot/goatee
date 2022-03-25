use actix_web::{App, HttpServer};
extern crate dotenv;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().expect("Failed to read .env file");
    HttpServer::new(|| {
        App::new()
            .service(routes::test::test)
            .service(routes::test::echo)
    })
    .bind("0.0.0.0:3000")?
    .run()
    .await
}
