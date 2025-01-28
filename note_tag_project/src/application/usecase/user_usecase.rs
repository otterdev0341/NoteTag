
use std::{sync::Arc, time::SystemTime};

use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use rocket::http::Status;

use crate::{ configuration::jwt_config::JwtSecret, domain::{dto::auth_dto::{Claims, ReqSignInDto, ReqSignUpDto, ResSignInDto}, repositories::require_implementation::{trait_user_helper_repository::UserHelperRepository, trait_user_repository::UserRepository}}, infrastructure::http::response_type::response_type::ErrorResponse};

pub struct UserUseCase<T>
where 
    T: UserRepository + UserHelperRepository + Send + Sync,
{
    user_repository: Arc<T>
}


pub enum UserOperation {
    Success,
    Failed,
    DatabaseError
}

impl<T> UserUseCase<T>
where 
    T : UserRepository + UserHelperRepository + Send + Sync,
{
    pub async fn new(user_repository: Arc<T>) -> Self {
        Self {
            user_repository: user_repository
        }
    }
    
    pub async fn sign_up(&self, user_data: ReqSignUpDto) -> UserOperation {
        // check is username and email unique
        let username_unique = self.user_repository.is_username_unique(&user_data.username).await;
        
        if !username_unique {
            return UserOperation::Failed;
        }

        let email_unique = self.user_repository.is_email_unique(&user_data.email).await;
        if !email_unique {
            return UserOperation::Failed;
        }

        // create user
        let result = self.user_repository.create_user(user_data).await;
        if result.is_err() {
            return UserOperation::DatabaseError;
        }else {
            return UserOperation::Success;
        }
        
    }
    pub async fn sign_in(&self, user_data: ReqSignInDto) -> Result<ResSignInDto, ErrorResponse>{
        // check if user exists
        let user = self.user_repository.is_user_data_valid(user_data.clone()).await;
        let u = match user {
            Some(u) => u,
            None => return Err(ErrorResponse((Status::BadRequest, "Invalid username or password".to_string())))
        };
        // verify password
        if !verify(&user_data.password, &u.password).unwrap() {
            return Err(ErrorResponse((Status::Unauthorized, "Invalid username or password".to_string())))
        }
        // generate token
        let claims = Claims {
            sub: u.id,
            role: u.role_id.to_string(),
            exp: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            + 4 * 60 * 60,
        };

        let header = Header {
            alg: jsonwebtoken::Algorithm::HS512,
            ..Default::default()
        };

        let token = encode(
            &header,
            &claims,
            &EncodingKey::from_secret(JwtSecret::default().jwt_secret.as_bytes())
        ).map_err(|e| ErrorResponse((
            Status::InternalServerError,
            format!("Token generation error: {}", e)
        )))?;
        
    
        Ok(ResSignInDto { token })
        
            
        // generate token
    
    }

    pub fn delete_account(&self) {
        todo!()
    }

    pub fn update_account(&self) {
        todo!()
    }

}