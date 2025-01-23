use rocket::serde::json::Json;
use sea_orm::{DatabaseConnection, DbErr};
use sea_orm_migration::async_trait;

use crate::domain::{dto::auth_dto::{ReqSignUpDto, ResSignInDto}, entities::user, repositories::trait_user_repository::UserRepository};

pub struct ImplUserRepository{
    db: DatabaseConnection
}

impl ImplUserRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        ImplUserRepository {
            db
        }
    }
    pub fn is_email_unique(&self, email: &str) -> bool {
        todo!()
    }

    pub fn is_username_unique(&self, username: &str) -> bool {
        todo!()
    }

    pub fn is_user_data_valid(&self, user_info: &ReqSignUpDto) -> Json<ResSignInDto> {
        todo!()
    }

}
#[async_trait::async_trait]
impl UserRepository for ImplUserRepository {
    async fn create_user(&self, user_info: ReqSignUpDto) -> Result<(), DbErr> {
        todo!()
    }

    async fn get_user_by_id(&self, user_id: i32) -> Result<Option<user::Entity>, DbErr> {
        todo!()
    }

    async fn get_all_user(&self) -> Result<Vec<user::Entity>, DbErr> {
        todo!()
    }

    async fn update_user_by_id(&self, user_id: i32, user_info: ReqSignUpDto) -> Result<(), DbErr> {
        todo!()
    }

    async fn delete_user_by_id(&self, user_id: i32) -> Result<(), DbErr> {
        todo!()
    }
}

