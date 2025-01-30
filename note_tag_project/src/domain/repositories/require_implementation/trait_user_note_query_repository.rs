use sea_orm::DbErr;
use sea_orm_migration::async_trait;

use crate::domain::dto::{note_dto::ResNoteEntryDto, query::QueryNoteDto};

#[async_trait::async_trait]
pub trait UserNoteQueryRepository {
    async fn query_notes(&self, user_id: i32, query_info: QueryNoteDto) -> Result<Vec<ResNoteEntryDto>, DbErr>;
}

#[async_trait::async_trait]
pub trait UserNoteQueryRepositoryFullyImplemented {

}