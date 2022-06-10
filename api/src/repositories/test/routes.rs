use crate::repositories::test::{ITestRepository, TestRepository};
use actix_web::{get, web, HttpResponse};

#[get("/")] // This will reponse every call on API_ADDRESS/test/
async fn hello() -> HttpResponse {
    let _repository: TestRepository = TestRepository {};
    let resp = _repository.test();
    return HttpResponse::Ok().json(resp);
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(hello);
}
