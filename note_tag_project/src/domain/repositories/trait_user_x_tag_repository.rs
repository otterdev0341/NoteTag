use sea_orm::DbErr;
use sea_orm_migration::async_trait;

#[async_trait::async_trait]
pub trait UserTagRepository {
    async fn create_user_tag(&self, user_id:i32, tag_name: &str) -> Result<(), DbErr>;
    async fn get_user_tags(&self, user_id: i32) -> Result<Vec<String>, DbErr>;
    async fn update_user_tag(&self, user_id: i32, old_tag: &str, new_tag: &str) -> Result<(), DbErr>;
    async fn delete_tag_from_user(&self, user_id: i32, tag_name: &str) -> Result<(), DbErr>;
}