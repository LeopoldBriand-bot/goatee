use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod middlewares;
mod repositories;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");
    let api_address: &str =
        &env::var("API_ADDRESS").expect("API_ADDRESS env var not foud in .env file");
    println!("API will run on: {}", api_address);
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::scope("/test").configure(repositories::test::init_routes))
    })
    .bind(api_address)?
    .run()
    .await
}
