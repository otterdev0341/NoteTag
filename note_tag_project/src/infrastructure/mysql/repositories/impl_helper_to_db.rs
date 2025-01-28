use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, DatabaseTransaction, DbErr, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter as _, Select, Set};
use sea_orm_migration::async_trait;
use tracing::error;

use crate::domain::{entities::{note_tag, tag, user, user_tag}, repositories::{trait_entity_helper::EntityHelper, trait_association_helper::AssociationHelper}};

#[async_trait::async_trait]
impl EntityHelper for DatabaseConnection {
    
    async fn find_or_create<E>(
        txn: &DatabaseTransaction,
        finder: Select<E>,
        new_record: E::ActiveModel,
    ) -> Result<E::Model, DbErr>
    where
        E: EntityTrait + Send + Sync + 'static,
        E::Model: ModelTrait + Send + Sync + IntoActiveModel<E::ActiveModel>,
        E::ActiveModel: ActiveModelTrait<Entity = E> + Send,
    {
        // Attempt to find the record
        if let Some(record) = finder.one(txn).await? {
            return Ok(record);
        }

        // If the record does not exist, create a new one
        let inserted_record = new_record.insert(txn).await?;
        Ok(inserted_record)
    }

    async fn is_user_status_is_active(
        txn: &DatabaseTransaction,
        user_id: i32,
    ) -> Result<bool, DbErr> {
        let user = user::Entity::find()
            .filter(user::Column::Id.eq(user_id))
            .one(txn)
            .await?;
        Ok(user.is_some())
    }
}

#[async_trait::async_trait]
impl AssociationHelper for DatabaseConnection{
    async fn is_this_tag_is_exist_in_tag_table_or_create(
        txn: &DatabaseTransaction,
        user_tag: &str,
    ) -> Result<i32, DbErr> {
        let result = tag::Entity::find()
            .filter(tag::Column::TagName.eq(user_tag))
            .one(txn)
            .await?;
        match result {
            Some(tag) => Ok(tag.id),
            None => {
                let new_tag = tag::ActiveModel {
                    tag_name: Set(user_tag.to_string()),
                    ..Default::default()
                };
                let inserted_tag = new_tag.insert(txn).await?;
                Ok(inserted_tag.id)
            }
        }

    }

    async fn is_tag_id_is_associate_with_this_user(
        txn: &DatabaseTransaction,
        user_id: i32,
        tag_id: i32,
    ) -> Result<bool, DbErr>{
        let result = user_tag::Entity::find()
            .filter(user_tag::Column::UserId.eq(user_id))
            .filter(user_tag::Column::TagId.eq(tag_id))
            .one(txn)
            .await?;
        match result {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    async fn is_tag_id_is_associate_with_note_id(
        txn: &DatabaseTransaction,
        note_id: i32,
        tag_id: i32,
    ) -> Result<bool, DbErr>{
        let result = note_tag::Entity::find()
            .filter(note_tag::Column::NoteId.eq(note_id))
            .filter(note_tag::Column::TagId.eq(tag_id))
            .one(txn)
            .await?;
        match result {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
    
    
}