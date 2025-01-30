use sea_orm::{ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;
use tracing::info;

use crate::domain::entities::user_tag;

#[async_trait::async_trait]
pub trait UserTagRepository {
    async fn create_user_tag(&self, user_id:i32, tag_name: &str) -> Result<(), DbErr>;
    async fn get_user_tags(&self, user_id: i32) -> Result<Vec<String>, DbErr>;
    async fn update_user_tag(&self, user_id: i32, old_tag: &str, new_tag: &str) -> Result<(), DbErr>;
    async fn delete_tag_from_user(&self, user_id: i32, tag_name: &str) -> Result<(), DbErr>;
}


#[async_trait::async_trait]
pub trait UserTagRepositoryFullyImplementd {
    // check is { tag_id } is associate with this { user } or not
    // if not associate will create new association
    // if associate do nothing
    async fn is_tag_id_is_associate_with_this_user_or_create(
        &self,
        txn: &DatabaseTransaction,
        user_id: i32,
        tag_id: i32,
    ) -> Result<user_tag::Model, DbErr> 
    {
        let is_tag_associate = user_tag::Entity::find()
            .filter(user_tag::Column::UserId.eq(user_id))
            .filter(user_tag::Column::TagId.eq(tag_id))
            .one(txn)
            .await?;
        match is_tag_associate {
            Some(model) => {
                info!("tag is already associate with user");
                Ok(model)
            },
            None => {
                let new_user_tag = user_tag::ActiveModel {
                    user_id: Set(user_id),
                    tag_id: Set(tag_id),
                    ..Default::default()
                };
                let insert_result = user_tag::Entity::insert(new_user_tag)
                    .exec(txn)
                    .await?;
                let inserted_model = user_tag::Entity::find_by_id(insert_result.last_insert_id)
                    .one(txn)
                    .await?
                    .ok_or(DbErr::RecordNotFound("Failed to fetch inserted note_tag".to_string()))?;
                info!("tag is associate with user");
                Ok(inserted_model)
            }
        }
    
    }
}