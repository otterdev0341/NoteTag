use std::sync::Arc;

use sea_orm::DbErr;

use crate::domain::{dto::{note_dto::ResNoteListDto, query::QueryNoteDto}, repositories::require_implementation::trait_user_note_query_repository::UserNoteQueryRepository};

pub struct UserQueryUsecase<T>
where 
    T: UserNoteQueryRepository + Send + Sync,
{
    user_note_query_repository: Arc<T>
}

impl<T> UserQueryUsecase<T>
where 
    T: UserNoteQueryRepository + Send + Sync,
{
    pub async fn new(user_note_query_repository: Arc<T>) -> Self {
        Self {
            user_note_query_repository: user_note_query_repository
        }
    }

    pub async fn query_notes(&self, user_id: i32, query_info: QueryNoteDto) -> Result<ResNoteListDto, DbErr> {
        let result = self.user_note_query_repository.query_notes(user_id, query_info).await?;
        Ok(ResNoteListDto {
            total: result.len() as i32,
            notes: result
        })
    }
}