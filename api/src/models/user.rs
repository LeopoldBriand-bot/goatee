use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", with = "mongodb::bson::serde_helpers::uuid_as_binary")]
    pub id: Uuid,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
    pub birth_date: String,
}
