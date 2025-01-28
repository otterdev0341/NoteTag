use sea_orm::{
    ActiveModelTrait, DatabaseTransaction, DbErr, EntityTrait, IntoActiveModel, ModelTrait, QuerySelect, Select
};
use sea_orm_migration::async_trait;

#[async_trait::async_trait]
pub trait EntityHelper {
    async fn find_or_create<E>(
        txn: &DatabaseTransaction,
        finder: sea_orm::Select<E>,
        new_record: E::ActiveModel,
    ) -> Result<E::Model, DbErr>
    where
        E: EntityTrait + Send + Sync + 'static,
        E::Model: IntoActiveModel<E::ActiveModel> + Send + Sync,
        E::ActiveModel: ActiveModelTrait<Entity = E> + Send;

    async fn is_user_status_is_active(
        txn: &DatabaseTransaction,
        user_id: i32,
    ) -> Result<bool, DbErr>;
}