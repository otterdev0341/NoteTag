use rocket::serde;
use ::serde::Deserialize;
use utoipa::ToSchema;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, ToSchema, Clone)]
#[serde(crate = "rocket::serde")]
pub struct QueryNoteDto{
    pub title: Option<String>,
    pub content: Option<String>,
    pub noteTags: Option<Vec<String>>,
}