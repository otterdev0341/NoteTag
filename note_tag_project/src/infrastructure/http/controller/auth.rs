use rocket::{get, post};

#[post("/sign-in")]
pub async fn sign_in() -> &'static str {
    "This is sign in"
}

#[post("/sign-up")]
pub async fn sign_up() -> &'static str {
    "This is sign up page"
}

#[get("/me")]
pub async fn me() -> &'static str {
    "This is me"
}