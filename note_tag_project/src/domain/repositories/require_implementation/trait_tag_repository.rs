use sea_orm::DbErr;
use sea_orm_migration::async_trait;

#[async_trait::async_trait]
pub trait TagRepository {
    async fn get_all_user_tags(&self, user_id: i32) -> Result<Vec<String>, DbErr>;
    async fn create_user_tag(&self, user_id: i32, tag: String) -> Result<(), DbErr>;
    async fn update_user_tag(&self, user_id: i32, tag: String) -> Result<(), DbErr>;
    async fn delete_user_tag(&self, user_id: i32, tag: String) -> Result<(), DbErr>;
}