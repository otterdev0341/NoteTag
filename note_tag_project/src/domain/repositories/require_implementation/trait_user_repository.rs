use sea_orm::{ColumnTrait, DatabaseTransaction, DbErr, EntityTrait, QueryFilter};
use sea_orm_migration::async_trait;

use crate::domain::{dto::auth_dto::ReqSignUpDto, entities::user};

#[async_trait::async_trait]
pub trait UserRepository {
    async fn create_user(&self, user_info: ReqSignUpDto) -> Result<(), DbErr>;
    async fn get_user_by_id(&self, user_id: i32) -> Result<Option<user::Entity>, DbErr>;
    async fn get_all_user(&self)-> Result<Vec<user::Entity>, DbErr>;
    async fn update_user_by_id(&self, user_id: i32, user_info: ReqSignUpDto) -> Result<(), DbErr>;
    async fn delete_user_by_id(&self, user_id: i32) -> Result<(), DbErr>; 
}

#[async_trait::async_trait]
pub trait UserRepositoryFullyImplemented {
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