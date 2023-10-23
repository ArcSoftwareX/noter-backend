use actix_session::Session;
use actix_web::{get, web, HttpResponse};

use crate::{
    app::{
        app_data::AppData,
        error::{AppError, AppResult},
    },
    utils::auth::get_uid,
};

#[get("/@me")]
pub async fn me(data: web::Data<AppData>, session: Session) -> AppResult<HttpResponse> {
    let user_id = get_uid(&session)?;

    let user = data
        .mongo
        .users
        .get(&user_id)
        .await
        .or(Err(AppError::Internal))?
        .ok_or(AppError::InvalidCredentials)?;

    Ok(HttpResponse::Ok().json(user.inner.response()))
}
