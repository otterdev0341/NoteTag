use rocket::{serde::{Deserialize, Serialize}, Responder};
use utoipa::ToSchema;




#[derive(Deserialize, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignInDto{
    pub email: String,
    pub password: String
}
#[derive(Serialize, Deserialize, Responder, ToSchema)]
#[serde(crate = "rocket::serde")]
pub struct ResSignInDto{
    pub token: String
}


#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ReqSignUpDto{
    pub username: String,
    pub password: String,
    pub email: String,
    pub first_name: String,
    pub middle_name: String,
    pub last_name: String,
    pub gender: u8,
}