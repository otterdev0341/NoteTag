use sea_orm::DbErr;
use sea_orm_migration::async_trait;

#[async_trait::async_trait]
pub trait NoteHexColorRepository {
    async fn set_note_color(&self, user_id:i32, note_id: i32, hex_color: &str) -> Result<(), DbErr>;
        
}