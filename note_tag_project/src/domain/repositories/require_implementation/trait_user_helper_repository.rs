use sea_orm::DbErr;
use sea_orm_migration::async_trait;

use crate::domain::{dto::auth_dto::ReqSignInDto, entities::user};

#[async_trait::async_trait]
pub trait UserHelperRepository {
    async fn is_email_unique(&self, email: &str) -> bool;
    async fn is_username_unique(&self, username: &str) -> bool;
    async fn get_id_by_gender(&self, gender: &str) -> Result<i32, DbErr>;
    async fn is_user_data_valid(&self, user_info: ReqSignInDto) -> Option<user::Model>;
}