use sea_orm::DbErr;
use sea_orm_migration::async_trait;

#[async_trait::async_trait]
pub trait NoteStatus {
    async fn set_note_status(&self, note_id: i32, status: &str) -> Result<(), DbErr>;
    
}