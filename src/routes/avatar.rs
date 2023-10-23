use std::io::Read;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_session::Session;
use actix_web::{get, http::header::ContentType, put, web, HttpResponse};
use futures::{AsyncReadExt, AsyncWriteExt};
use serde_json::json;

use crate::{
    app::{
        app_data::AppData,
        error::{AppError, AppResult},
    },
    constants::MAX_AVATAR_SIZE,
    utils::auth::get_uid,
};

#[derive(MultipartForm)]
pub struct Upload {
    file: TempFile,
}

#[put("")]
pub async fn upload_avatar(
    data: web::Data<AppData>,
    session: Session,
    form: MultipartForm<Upload>,
) -> AppResult<HttpResponse> {
    let uid = get_uid(&session)?;
    let user = data
        .mongo
        .users
        .get(&uid)
        .await
        .or(Err(AppError::Internal))?
        .ok_or(AppError::InvalidCredentials)?;

    if form.file.content_type.is_none() {
        return Ok(
            HttpResponse::BadRequest().json(json!({ "error": "Content type is not provided" }))
        );
    }

    let content_type = form.file.content_type.clone().unwrap().clone();

    if content_type != mime::IMAGE_PNG && content_type != mime::IMAGE_JPEG {
        return Err(AppError::InvalidFileType(content_type.to_string()));
    }

    match form.file.size as u64 {
        0 => return Err(crate::app::error::AppError::InvalidAvatarSize(0)),
        length if length > MAX_AVATAR_SIZE => {
            return Err(crate::app::error::AppError::InvalidAvatarSize(length))
        }
        _ => (),
    }

    let bucket = &data.mongo.avatars.bucket;

    let buf = web::block(move || {
        let mut file = form.file.file.as_file();
        let mut buf = Vec::with_capacity(file.metadata().unwrap().len() as usize);

        println!("buffer length is: {}", buf.len());

        file.read_to_end(&mut buf).unwrap();

        buf
    })
    .await
    .or(Err(AppError::Internal))?;

    let mut upload_stream =
        bucket.open_upload_stream(format!("avatar.{}", content_type.subtype()), None);

    upload_stream
        .write_all(&buf)
        .await
        .or(Err(AppError::Internal))?;

    if let Some(avatar_id) = user.inner.avatar_id {
        bucket
            .delete(mongodb::bson::Bson::ObjectId(avatar_id))
            .await
            .or(Err(AppError::Internal))?;
    }

    upload_stream.close().await.or(Err(AppError::Internal))?;

    let file_id = upload_stream
        .id()
        .as_object_id()
        .ok_or(AppError::Internal)?;

    data.mongo
        .users
        .update_avatar_id(&uid, &file_id)
        .await
        .or(Err(AppError::Internal))?;

    Ok(HttpResponse::Created()
        .json(json!({ "message": "Avatar changed successfully", "data": file_id })))
}

#[get("")]
pub async fn get_avatar(data: web::Data<AppData>, session: Session) -> AppResult<HttpResponse> {
    let user_id = get_uid(&session)?;
    let user = data
        .mongo
        .users
        .get(&user_id)
        .await
        .or(Err(AppError::Internal))?
        .ok_or(AppError::InvalidCredentials)?;

    if user.inner.avatar_id.is_none() {
        return Ok(HttpResponse::NoContent().finish());
    }

    let bucket = &data.mongo.avatars.bucket;
    let mut download_stream = bucket
        .open_download_stream(user.inner.avatar_id.into())
        .await
        .or(Err(AppError::Internal))?;

    let mut buf = Vec::new();
    download_stream
        .read_to_end(&mut buf)
        .await
        .or(Err(AppError::Internal))?;

    // println!("respond with avatar");
    Ok(HttpResponse::Ok()
        .content_type(ContentType::jpeg())
        .body(buf))
}
