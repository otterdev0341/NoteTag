use sea_orm::{ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, QueryFilter};
use sea_orm_migration::async_trait;

use crate::domain::entities::note_hex_color;

#[async_trait::async_trait]
pub trait NoteHexColorRepository {
    async fn set_note_color(&self, user_id:i32, note_id: i32, hex_color: &str) -> Result<(), DbErr>;
        
}

#[async_trait::async_trait]
pub trait NoteHexColorRepositoryFullyImplemented{
    async fn get_color_detail_by_color_id(
        &self,
        txn: &DatabaseTransaction,
        color_id: i32,
    ) -> Result<String, DbErr> {
        let color = note_hex_color::Entity::find()
            .filter(note_hex_color::Column::Id.eq(color_id))
            .one(txn)
            .await?
            .ok_or(DbErr::RecordNotFound("Failed to fetch color detail".to_string()))?.hex_color;
        
        Ok(color)
    }

    async fn get_color_id_by_color_detail(
        &self,
        txn: &DatabaseTransaction,
        color_detail: &str
    ) -> Result<i32, DbErr> {
        let color = note_hex_color::Entity::find()
            .filter(note_hex_color::Column::HexColor.eq(color_detail))
            .one(txn)
            .await?
            .ok_or(DbErr::RecordNotFound("Failed to fetch color id".to_string()))?.id;
            
        Ok(color)
    }
}