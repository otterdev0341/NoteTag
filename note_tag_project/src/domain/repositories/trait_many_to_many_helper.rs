use sea_orm::{DatabaseTransaction, DbErr};
use sea_orm_migration::async_trait;





#[async_trait::async_trait]
pub trait AssociationHelper {
    async fn is_this_tag_is_exist_in_tag_table_or_create(
        txn: &DatabaseTransaction,
        user_tag: &str,
    ) -> Result<Option<i32>, DbErr>;

    async fn is_tag_id_is_associate_with_this_user(
        txn: &DatabaseTransaction,
        user_id: i32,
        tag_id: i32,
    ) -> Result<bool, DbErr>;

    async fn is_tag_id_is_associate_with_note_id(
        txn: &DatabaseTransaction,
        note_id: i32,
        tag_id: i32,
    ) -> Result<bool, DbErr>;

    
}