use actix_web::{middleware, web, App, HttpServer};

mod middlewares;
mod repositories;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::scope("/test").configure(repositories::test::init_routes))
    })
    .bind("0.0.0.0:3001")?
    .run()
    .await
}
