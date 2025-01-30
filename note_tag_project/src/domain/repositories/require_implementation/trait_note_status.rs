use sea_orm::{ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, QueryFilter};
use sea_orm_migration::async_trait;

use crate::domain::entities::note_status;

#[async_trait::async_trait]
pub trait NoteStatus {
    async fn set_note_status(&self, note_id: i32, status: &str) -> Result<(), DbErr>;
    
}

#[async_trait::async_trait]
pub trait NoteStatusFullyImplemented {
    async fn get_status_detail_by_status_id(
        &self,
        txn: &DatabaseTransaction,
        status_id: i32,
    ) -> Result<String, DbErr> {
        let status = note_status::Entity::find()
            .filter(note_status::Column::Id.eq(status_id))
            .one(txn)
            .await?
            .ok_or(DbErr::RecordNotFound("Failed to fetch status detail".to_string()))?.status_detail;
        
        Ok(status)
    }

    async fn get_status_id_by_status_detail(
        &self,
        txn: &DatabaseTransaction,
        status_detail: &str
    ) -> Result<i32, DbErr> {
        let status = note_status::Entity::find()
            .filter(note_status::Column::StatusDetail.eq(status_detail))
            .one(txn)
            .await?
            .ok_or(DbErr::RecordNotFound("Failed to fetch status id".to_string()))?.id;
        
        Ok(status)
    }
}