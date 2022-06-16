use crate::db::DB;
use crate::models::{IRegisterRequest, RegisterRequest, User};
use async_trait::async_trait;
use bson::doc;
use uuid::Uuid;

#[async_trait]
pub trait IUserRepository {
    async fn find_by_email(&self, email: String) -> Option<User>;
    async fn register(&self, user: RegisterRequest) -> Option<User>;
    async fn delete_by_id(&self, user_id: String) -> Option<User>;
}

pub struct UserRepository {}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn find_by_email(&self, email: String) -> Option<User> {
        let collection = DB.get().unwrap().collection::<User>("User");
        return collection
            .find_one(doc! {"email": email}, None)
            .await
            .unwrap();
    }

    async fn register(&self, reg: RegisterRequest) -> Option<User> {
        let _exist = self.find_by_email(reg.email.clone()).await;
        match _exist {
            Some(_) => {
                // There is already a user with this email in DB
                None
            }
            None => {
                // Create user with this infos
                let collection = DB.get().unwrap().collection::<User>("User");
                let user = reg.to_user();
                match collection.insert_one(user.clone(), None).await {
                    Ok(_) => Some(user),
                    Err(_) => None,
                }
            }
        }
    }
    async fn delete_by_id(&self, user_id: String) -> Option<User> {
        let collection = DB.get().unwrap().collection::<User>("User");
        match collection
            .find_one_and_delete(
                doc! {
                    "_id": Uuid::parse_str(&user_id).unwrap()
                },
                None,
            )
            .await
        {
            Ok(user) => user,
            Err(_) => None,
        }
    }
}
