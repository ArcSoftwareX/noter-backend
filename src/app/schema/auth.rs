use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct SignupSchema {
    #[validate(email(message = "Invalid email provided"))]
    pub email: String,

    #[validate(length(min = 8, max = 72, message = "Password must be 8-32 characters long"))]
    pub password: String,

    #[validate(length(min = 2, max = 64))]
    pub name: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct LoginSchema {
    #[validate(email(message = "Invalid email provided"))]
    pub email: String,

    #[validate(length(min = 8, max = 72, message = "Password must be 8-32 characters long"))]
    pub password: String,
}
