use utoipa::OpenApi;

use crate::domain::dto::auth_dto::{ReqSignInDto, ResSignInDto};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::infrastructure::http::controller::auth::sign_in
    ),
    components(
        schemas(
            ReqSignInDto, ResSignInDto)
    )
)]
pub struct ApiDoc();
