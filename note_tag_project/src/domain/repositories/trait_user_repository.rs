use sea_orm::DbErr;
use sea_orm_migration::async_trait;

use crate::domain::{dto::auth_dto::ReqSignUpDto, entities::user};

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create_user(&self, user_info: ReqSignUpDto) -> Result<(), DbErr>;
    async fn get_user_by_id(&self, user_id: i32) -> Result<Option<user::Entity>, DbErr>;
    async fn get_all_user(&self)-> Result<Vec<user::Entity>, DbErr>;
    async fn update_user_by_id(&self, user_id: i32, user_info: ReqSignUpDto) -> Result<(), DbErr>;
    async fn delete_user_by_id(&self, user_id: i32) -> Result<(), DbErr>; 

    
}