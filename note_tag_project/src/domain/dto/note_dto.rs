use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;


#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ReqCreateNoteDto{
    pub title: Option<String>,
    pub content: Option<String>,
    pub color: Option<String>,
    pub status: Option<String>,
    pub noteTags: Option<Vec<String>>
}

// this is only on reposne, then dont need to validate this struct
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ResNoteEntryDto {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub colorCode: String,
    pub status: String,
    pub noteTags: Vec<String>,
    pub createdAt: String,
    pub updatedAt: String
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, Clone)]
#[serde(crate = "rocket::serde")]
pub struct ReqUpdateNoteDto{
    pub id: i32,
    pub title: Option<String>,
    pub content: Option<String>,
    pub color: Option<String>,
    pub status: Option<String>,
    pub noteTags: Option<Vec<String>>
}

// this is only on reposne, then dont need to validate this struct
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
#[serde(crate = "rocket::serde")]
pub struct ResNoteListDto{
    pub total: i32,
    pub notes: Vec<ResNoteEntryDto>
}