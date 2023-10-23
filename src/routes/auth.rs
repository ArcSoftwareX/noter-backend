use actix_session::Session;
use actix_web::{post, web, HttpResponse};
use mongodb::bson::DateTime;
use serde_json::json;
use validator::Validate;

use crate::{
    app::{
        app_data::AppData,
        error::{AppError, AppResult},
        schema::auth::{LoginSchema, SignupSchema},
    },
    repo::model::{
        date::Date,
        user::{User, WithPassword},
    },
    utils::{
        hashing::{hash_password, validate_hash},
        validation::transform_errors,
    },
};

#[post("/signup")]
pub async fn signup(
    data: web::Data<AppData>,
    payload: web::Json<SignupSchema>,
    session: Session,
) -> AppResult<HttpResponse> {
    if let Err(errors) = payload.validate() {
        return Err(AppError::ValidationError(transform_errors(errors)));
    }

    if data
        .mongo
        .users
        .exists(&payload.email)
        .await
        .or(Err(AppError::Internal))?
    {
        return Err(AppError::UserAlreadyExists(payload.email.to_owned()));
    }

    let user = data
        .mongo
        .users
        .create(WithPassword::new(
            User {
                id: None,
                email: payload.email.to_owned(),
                name: payload.name.to_owned(),
                avatar_id: None,
                created_at: Date::Mongo(DateTime::now()),
            },
            hash_password(&payload.password).or(Err(AppError::Internal))?,
        ))
        .await
        .or(Err(AppError::Internal))?;

    session
        .insert("uid", user.inner.id.as_ref().unwrap())
        .or(Err(AppError::Internal))?;

    Ok(HttpResponse::Ok().json(user.inner.response()))
}

#[post("/login")]
pub async fn login(
    data: web::Data<AppData>,
    payload: web::Json<LoginSchema>,
    session: Session,
) -> AppResult<HttpResponse> {
    if let Err(errors) = payload.validate() {
        return Err(AppError::ValidationError(transform_errors(errors)));
    }

    let user = data
        .mongo
        .users
        .get_by_email(&payload.email)
        .await
        .or(Err(AppError::Internal))?
        .ok_or(AppError::InvalidCredentials)?;

    if validate_hash(&payload.password, &user.password).is_err() {
        return Err(AppError::InvalidCredentials);
    }

    session
        .insert("uid", user.inner.id.as_ref().unwrap())
        .or(Err(AppError::Internal))?;

    Ok(HttpResponse::Ok().json(user.inner.response()))
}

#[post("/logout")]
pub async fn logout(session: Session) -> AppResult<HttpResponse> {
    session.remove("uid").ok_or(AppError::LoggedOut)?;

    Ok(HttpResponse::Ok().json(json!({ "message": "Logged out successfully" })))
}
