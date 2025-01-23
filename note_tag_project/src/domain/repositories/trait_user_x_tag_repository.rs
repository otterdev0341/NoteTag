use sea_orm::DbErr;
use sea_orm_migration::async_trait;

#[async_trait::async_trait]
pub trait UserTagRepository {
    async fn add_user_tag() -> Result<(), DbErr>;
    async fn get_tag_by_user() -> Result<(), DbErr>;
    async fn get_user_by_tag() -> Result<(), DbErr>;
    async fn delete_user_tag() -> Result<(), DbErr>;
    async fn check_user_tag_exists() -> Result<(), DbErr>;
}