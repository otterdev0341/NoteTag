use sea_orm::DbErr;
use sea_orm_migration::async_trait;

#[async_trait::async_trait]
pub trait NoteTagRepository {
    async fn add_note_tag() -> Result<(), DbErr>;
    async fn get_tag_by_note() -> Result<(), DbErr>;
    async fn get_note_by_tag() -> Result<(), DbErr>;
    async fn delete_note_tag() -> Result<(), DbErr>;
    async fn check_note_tag_exists() -> Result<(), DbErr>;
}