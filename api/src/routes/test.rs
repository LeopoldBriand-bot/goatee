use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
pub async fn test() -> impl Responder {
    let test_env = std::env::var("TEST").expect("TEST not found");
    let mut message: String = String::from("Hello ");
    message.push_str(&test_env.to_string());
    message.push_str("!");
    HttpResponse::Ok().body(message)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
