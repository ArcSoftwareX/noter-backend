use actix_session::Session;
use mongodb::bson::oid::ObjectId;

use crate::app::error::{AppError, AppResult};

pub fn get_uid(session: &Session) -> AppResult<ObjectId> {
    session
        .get::<ObjectId>("uid")
        .or(Err(AppError::LoggedOut))?
        .ok_or(AppError::InvalidCredentials)
}
