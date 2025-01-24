

use std::sync::Arc;

use bcrypt::{hash, DEFAULT_COST};
use rocket::serde::json::Json;
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;


use crate::domain::{dto::auth_dto::{ReqSignInDto, ReqSignUpDto, ResSignInDto}, entities::{gender, user}, repositories::{trait_user_helper_repository::UserHelperRepository, trait_user_repository::UserRepository}};

pub struct ImplUserRepository
{
    pub db: Arc<DatabaseConnection>
}

impl ImplUserRepository {
    pub fn new(db: Arc<DatabaseConnection>) -> Self {
        ImplUserRepository {
            db
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for ImplUserRepository {
    
    async fn create_user(&self, user_info: ReqSignUpDto) -> Result<(), DbErr> {
        
        let gender_id = self.get_id_by_gender(&user_info.gender).await?;
        let hash_password = hash(user_info.password, DEFAULT_COST).unwrap();

        let new_user = user::ActiveModel {
            email: Set(user_info.email),
            username: Set(user_info.username),
            password: Set(hash_password),
            first_name: Set(user_info.first_name),
            last_name: Set(user_info.last_name),
            middle_name: Set(user_info.middle_name),
            gender: Set(gender_id),
            role_id: Set(0),
            status: Set(0),
            ..Default::default()
        };
        user::Entity::insert(new_user).exec(&*self.db).await.map(|_| ())
    }

    async fn get_user_by_id(&self, user_id: i32) -> Result<Option<user::Entity>, DbErr> {
        todo!()
    }

    async fn get_all_user(&self) -> Result<Vec<user::Entity>, DbErr> {
        todo!()
    }

    async fn update_user_by_id(&self, user_id: i32, user_info: ReqSignUpDto) -> Result<(), DbErr> {
        todo!()
    }

    async fn delete_user_by_id(&self, user_id: i32) -> Result<(), DbErr> {
        todo!()
    }
}

impl UserHelperRepository for ImplUserRepository {
    async fn is_email_unique(&self, email: &str) -> bool {
        let result = user::Entity::find()
            .filter(user::Column::Email.eq(email))
            .one(&*self.db)
            .await;
        
        match result {
            Ok(user) => {
                if user.is_some() {
                    return false;
                }else {
                    return true;
                }
            },
            Err(_) => {
                return false;
            }
        }
    }

    async fn is_username_unique(&self, username: &str) -> bool {
        let result = user::Entity::find()
            .filter(user::Column::Username.eq(username))
            .one(&*self.db)
            .await;
        
        match result {
            Ok(user) => {
                if user.is_some() {
                    return false;
                }else {
                    return true;
                }
            },
            Err(_) => {
                return false;
            }
        }
    }
    
    async fn get_id_by_gender(&self, gender: &str) -> Result<i32, DbErr> {
        let result = gender::Entity::find()
            .filter(gender::Column::Detail.eq(gender))
            .one(&*self.db)
            .await?;

        match result {
            Some(gender) => Ok(gender.id),
            None => Err(DbErr::Custom("Gender not found".to_string())),
        }
    }
    
    async fn is_user_data_valid(&self, user_info: &ReqSignInDto) -> Option<user::Entity> {
        todo!()
    }

}

