use sea_orm::{ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;

use crate::domain::entities::tag;

#[async_trait::async_trait]
pub trait TagRepositoryFullyImplemented {
    // check is tag exist in tag table or not
    // if exist will return id of tag
    // if not exist will create new tag and return id of tag
    // it casesensitive that menn "hello" not same with "Hello"
    async fn is_this_tag_is_exist_in_tag_table_or_create(
        &self,
        txn: &DatabaseTransaction,
        user_tag: &str,
    ) -> Result<i32, DbErr> {
        let is_tag_exist = tag::Entity::find()
            .filter(tag::Column::TagName.eq(user_tag))
            .one(txn)
            .await?;
        let tag_id = match is_tag_exist {
            Some(tag) => tag.id,
            None => {
                let new_tag = tag::ActiveModel {
                    tag_name: Set(user_tag.to_string()),
                    ..Default::default()
                };
                let tag_id = tag::Entity::insert(new_tag)
                    .exec(txn)
                    .await?
                    .last_insert_id;
                tag_id
            }
        };
        Ok(tag_id)
    }

}