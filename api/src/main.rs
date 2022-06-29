use actix_web::{middleware, web, App, HttpServer};
extern crate anyhow;
use dotenv::dotenv;

mod db;
mod middlewares;
mod models;
mod repositories;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Getting env variables
    dotenv().expect(".env file not found");
    let api_address: &str =
        &std::env::var("API_ADDRESS").expect("API_ADDRESS env var not foud in .env file");
    println!("API will run on: {}", api_address);

    // Init DB
    db::init().await;

    // Run API
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::scope("/test").configure(repositories::test::init_routes))
            .service(web::scope("/user").configure(repositories::user::init_routes))
    })
    .bind(api_address)?
    .run()
    .await?;
    Ok(())
}
