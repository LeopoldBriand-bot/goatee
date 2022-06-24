use crate::models::RegisterRequest;
use crate::repositories::user::{IUserRepository, UserRepository};
use actix_web::{post, web, HttpResponse};

#[post("/register")] // This will reponse every call on API_ADDRESS/user/register
async fn register(user: web::Json<RegisterRequest>) -> HttpResponse {
    let _repository: UserRepository = UserRepository {};
    let resp = _repository.register(user.into_inner()).await;
    match resp {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::from_error(e),
    }
}

#[post("/delete/{user_id}")] // This will reponse every call on API_ADDRESS/user/delete/{userid}
async fn delete_by_id(path: web::Path<String>) -> HttpResponse {
    let user_id = path.into_inner();
    let _repository: UserRepository = UserRepository {};
    let resp = _repository.delete_by_id(user_id).await;
    match resp {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::from_error(e),
    }
}

// Init all user routes
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register);
    cfg.service(delete_by_id);
}
