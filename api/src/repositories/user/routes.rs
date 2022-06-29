use crate::models::{Claims, LoginRequest, RegisterRequest};
use crate::repositories::user::UserError;
use crate::repositories::user::{IUserRepository, UserRepository};
use actix_web::{delete, post, web, HttpResponse};
use jsonwebtoken::{encode, EncodingKey, Header};
use std::time::{SystemTime, UNIX_EPOCH};

#[post("/register")] // This will reponse every call on API_ADDRESS/user/register
async fn register(user: web::Json<RegisterRequest>) -> HttpResponse {
    let _repository: UserRepository = UserRepository {};
    let resp = _repository.register(user.into_inner()).await;
    match resp {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::from_error(e),
    }
}

#[post("/login")] // This will reponse every call on API_ADDRESS/user/login
async fn login(log: web::Json<LoginRequest>) -> HttpResponse {
    let _repository: UserRepository = UserRepository {};
    let resp = _repository.login(log.into_inner()).await;
    match resp {
        Ok(user) => {
            let current_utc = SystemTime::now() // Get current timestamp
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let token_claims = Claims::new(current_utc + 3600, user.clone()); // Create token claims, expire in 3600 seconds (1H)
            let secret_key: &str =
                &std::env::var("SECRET_KEY").expect("SECRET_KEY env var not foud in .env file");
            let token = encode(
                // Encode token with claims and secretkey
                &Header::default(),
                &token_claims,
                &EncodingKey::from_secret(secret_key.as_bytes()),
            );
            match token {
                Ok(tok) => HttpResponse::Ok()
                    .append_header(("AUTHORIZATION", tok))
                    .json(user),
                Err(_) => HttpResponse::from_error(UserError::TokenError),
            }
        }
        Err(e) => HttpResponse::from_error(e),
    }
}

#[delete("/delete/{user_id}")] // This will reponse every call on API_ADDRESS/user/delete/{userid}
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
    cfg.service(login);
    cfg.service(delete_by_id);
}
