use rocket::{serde, Responder};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize,ToSchema, Validate, Responder, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserTagDto {
    #[validate(length(min = 1, message = "Tag must be at least 1 characters"))]
    pub tagName: String
}

#[allow(non_snake_case)]
#[derive(Deserialize, Serialize, ToSchema, Clone)]
#[serde(crate = "rocket::serde")]
pub struct UserTagListDto {
    pub totalTag: usize,
    pub tagList: Vec<String>
}