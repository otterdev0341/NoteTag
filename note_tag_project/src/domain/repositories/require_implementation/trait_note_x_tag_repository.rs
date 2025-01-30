use sea_orm::DbErr;
use sea_orm_migration::async_trait;

#[async_trait::async_trait]
pub trait NoteTagRepository {
    async fn add_tag_to_note(&self, note_id: i32, tag: &str) -> Result<(), DbErr>;
    async fn remove_tag_from_note(&self, note_id: i32, tag: &str) -> Result<(), DbErr>;
    async fn get_note_by_tags(&self, user_id: i32, tags: Vec<String>) -> Result<Vec<i32>, DbErr>;
}