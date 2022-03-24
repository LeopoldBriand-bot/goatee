use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
