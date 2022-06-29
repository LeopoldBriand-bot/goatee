use crate::models::User;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub user: User,
}

impl Claims {
    pub fn new(exp: u64, user: User) -> Self {
        Claims {
            exp: exp as usize,
            user: user,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
    pub birth_date: String,
}

pub trait IRegisterRequest {
    fn to_user(&self) -> User;
}

impl IRegisterRequest for RegisterRequest {
    fn to_user(&self) -> User {
        User {
            id: Uuid::new_v4(),
            name: self.name.clone(),
            surname: self.surname.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
            birth_date: self.birth_date.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

impl LoginRequest {
    pub fn check_user(&self, user: User) -> bool {
        self.password == user.password
    }
}
