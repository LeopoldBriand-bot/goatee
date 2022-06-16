use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", with = "mongodb::bson::serde_helpers::uuid_as_binary")]
    id: Uuid,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
    pub birth_date: String,
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
