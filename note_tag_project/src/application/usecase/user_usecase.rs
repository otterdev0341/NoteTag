
use std::sync::Arc;

use rocket::serde::json::Json;
use sea_orm::DbErr;
use sea_orm_migration::async_trait;

use crate::{ domain::{dto::auth_dto::ReqSignUpDto, repositories::{trait_user_helper_repository::UserHelperRepository, trait_user_repository::UserRepository}}, infrastructure::mysql::repositories::impl_user_repository::ImplUserRepository};

pub struct UserUseCase<T>
where 
    T: UserRepository + UserHelperRepository + Send + Sync,
{
    user_repository: Arc<T>
}


pub enum UserOperation {
    Success,
    Failed,
    DatabaseError
}

impl<T> UserUseCase<T>
where 
    T : UserRepository + UserHelperRepository + Send + Sync,
{
    pub async fn new(user_repository: Arc<T>) -> Self {
        Self {
            user_repository: user_repository
        }
    }
    pub async fn sign_up(&self, user_data: ReqSignUpDto) -> UserOperation {
        // check is username and email unique
        let username_unique = self.user_repository.is_username_unique(&user_data.username).await;
        
        if !username_unique {
            return UserOperation::Failed;
        }

        let email_unique = self.user_repository.is_email_unique(&user_data.email).await;
        if !email_unique {
            return UserOperation::Failed;
        }

        // create user
        let result = self.user_repository.create_user(user_data).await;
        if result.is_err() {
            return UserOperation::DatabaseError;
        }else {
            return UserOperation::Success;
        }
        
    }
    pub fn sign_in(&self) {
        todo!()
    }

    pub fn delete_account(&self) {
        todo!()
    }

    pub fn update_account(&self) {
        todo!()
    }

}