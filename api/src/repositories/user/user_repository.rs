use crate::db::DB;
use crate::models::{IRegisterRequest, RegisterRequest, User};
use crate::repositories::user::UserError;
use async_trait::async_trait;
use bson::doc;
use uuid::Uuid;

#[async_trait]
pub trait IUserRepository {
    async fn find_by_email(&self, email: String) -> Result<User, UserError>;
    async fn register(&self, user: RegisterRequest) -> Result<User, UserError>;
    async fn delete_by_id(&self, user_id: String) -> Result<User, UserError>;
}

pub struct UserRepository {}

#[async_trait]
impl IUserRepository for UserRepository {
    async fn find_by_email(&self, email: String) -> Result<User, UserError> {
        let collection = DB.get().unwrap().collection::<User>("User");
        match collection.find_one(doc! {"email": email}, None).await {
            Ok(user) => match user {
                Some(user) => Ok(user),
                None => Err(UserError::UserNotFound),
            },
            Err(e) => Err(UserError::MongoError(e)),
        }
    }

    async fn register(&self, reg: RegisterRequest) -> Result<User, UserError> {
        let _exist = self.find_by_email(reg.email.clone()).await;
        match _exist {
            Ok(_) => Err(UserError::UserAlreadyExist),
            Err(_) => {
                // Create user with this infos
                let collection = DB.get().unwrap().collection::<User>("User");
                let user = reg.to_user();
                match collection.insert_one(user.clone(), None).await {
                    Ok(_) => Ok(user),
                    Err(e) => Err(UserError::MongoError(e)),
                }
            }
        }
    }
    async fn delete_by_id(&self, user_id: String) -> Result<User, UserError> {
        let collection = DB.get().unwrap().collection::<User>("User");
        return match Uuid::parse_str(&user_id) {
            Ok(uid) => match collection
                .find_one_and_delete(
                    doc! {
                        "_id": uid
                    },
                    None,
                )
                .await
            {
                Ok(resp) => match resp {
                    Some(user) => Ok(user),
                    None => Err(UserError::UserNotFound),
                },
                Err(e) => Err(UserError::MongoError(e)),
            },
            Err(e) => Err(UserError::IdError(e)),
        };
    }
}
