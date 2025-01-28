use std::sync::Arc;
use bcrypt::{hash, DEFAULT_COST};
use sea_orm::{ColumnTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, Set};
use sea_orm_migration::async_trait;
use tracing::info;
use crate::domain::{dto::auth_dto::{ReqSignInDto, ReqSignUpDto}, entities::{gender, user}, repositories::require_implementation::{trait_user_helper_repository::UserHelperRepository, trait_user_repository::UserRepository}};

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
        
        // let gender_id = self.get_id_by_gender(&user_info.gender).await?;
        let hash_password = hash(user_info.password, DEFAULT_COST).unwrap();
        // Log the values before executing the query
        
        
        
        let new_user = user::ActiveModel {
            
            username: Set(user_info.username.to_owned()),
            password: Set(hash_password.to_owned()),
            email: Set(user_info.email.to_owned()),
            first_name: Set(user_info.first_name.to_owned()),
            middle_name: Set(user_info.middle_name.to_owned()),
            last_name: Set(user_info.last_name.to_owned()),
            gender: Set(1 as i32),
            status: Set(1 as i32),
            role_id: Set(1 as i32),
            // created_at: Set(Some(Utc::now())),  // Set create_at with the current timestamp
            // updated_at: Set(Some(Utc::now())), // Set updated_at with the current timestamp
            ..Default::default()
        };
        let result = user::Entity::insert(new_user).exec(&*self.db).await.map(|_| ());
        match result {
            Ok(_) => {
                info!("User created successfully");
                Ok(())
            },
            Err(err) => {
                info!("Error creating user: {:?}", err);
                Err(err)
            }
            
        }
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


#[async_trait::async_trait]
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
    
    async fn is_user_data_valid(&self, user_info: ReqSignInDto) -> Option<user::Model> {
        let result = user::Entity::find()
            .filter(user::Column::Email.eq(user_info.email))
            .one(&*self.db)
            .await;
        
        match result {
            Ok(user) => {
                if user.is_some() {
                    return user;
                }else {
                    return None;
                }
            },
            Err(_) => {
                return None;
            }
        }
    }

}

