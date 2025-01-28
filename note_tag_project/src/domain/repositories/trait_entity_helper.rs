use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, IntoActiveModel, ModelTrait, QueryFilter, QuerySelect, Select
};
use sea_orm_migration::async_trait;

use crate::domain::entities::user;

#[async_trait::async_trait]
pub trait EntityHelperFullyImplemented {
    // async fn find_or_create<E>(
    //     txn: &DatabaseTransaction,
    //     finder: sea_orm::Select<E>,
    //     new_record: E::ActiveModel,
    // ) -> Result<E::Model, DbErr>
    // where
    //     E: EntityTrait + Send + Sync + 'static,
    //     E::Model: IntoActiveModel<E::ActiveModel> + Send + Sync,
    //     E::ActiveModel: ActiveModelTrait<Entity = E> + Send;

    async fn is_user_status_is_active(
        &self,
        txn: &DatabaseTransaction,
        user_id: i32,
    ) -> Result<bool, DbErr> {
        // 1 = active, 2 = inactive
        let user = user::Entity::find()
            .filter(user::Column::Id.eq(user_id))
            .one(txn)
            .await?;
        match user {
            Some(user) => {
                if user.status == 1 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => Ok(false),
        }
    }
}