use actix_session::Session;
use actix_web::{delete, get, post, put, web, HttpResponse};
use mongodb::bson::{doc, oid::ObjectId};
use serde_json::json;

use crate::{
    app::{
        app_data::AppData,
        error::{AppError, AppResult},
        schema::notes::{CreateNoteSchema, ReplaceNoteSchema},
    },
    repo::model::{date::Date, note::Note, oid::Oid},
    utils::auth::get_uid,
};

#[derive(serde::Deserialize)]
pub struct GetNotesQuery {
    page: Option<u64>,
    per_page: Option<u64>,
}

#[get("")]
pub async fn get_notes(
    data: web::Data<AppData>,
    query: web::Query<GetNotesQuery>,
    session: Session,
) -> AppResult<HttpResponse> {
    let uid = get_uid(&session)?;

    let notes = data
        .mongo
        .notes
        .get_by_author(&uid, query.page.unwrap_or(0), query.per_page.unwrap_or(20))
        .await
        .or(Err(AppError::Internal))?;

    Ok(HttpResponse::Ok().json(json!({ "notes": notes })))
}

#[post("")]
pub async fn create_note(
    data: web::Data<AppData>,
    payload: web::Json<CreateNoteSchema>,
    session: Session,
) -> AppResult<HttpResponse> {
    let uid = get_uid(&session)?;

    let note = Note::from_create_schema(payload.0, uid);

    let note = data
        .mongo
        .notes
        .create(note)
        .await
        .or(Err(AppError::Internal))?;

    Ok(HttpResponse::Ok().json(note.response()))
}

#[put("/{note_id}")]
pub async fn replace_note(
    data: web::Data<AppData>,
    path: web::Path<String>,
    payload: web::Json<ReplaceNoteSchema>,
    session: Session,
) -> AppResult<HttpResponse> {
    let uid = get_uid(&session)?;
    let note_id = ObjectId::parse_str(&*path).or(Err(AppError::NoteDoesNotExist))?;

    let old_note = data
        .mongo
        .notes
        .get(&note_id, &uid)
        .await
        .or(Err(AppError::Internal))?
        .ok_or(AppError::NoteDoesNotExist)?;

    let new_note = Note {
        author_id: Oid::ObjectId(uid),
        content: payload.0.content,
        description: payload.0.description,
        title: payload.0.title,
        // priority: payload.0.priority,
        // status: payload.0.status,
        tags: payload.0.tags,
        updated_at: Date::Mongo(mongodb::bson::DateTime::now()),
        created_at: old_note.created_at.mongo(),
        ..old_note
    };

    let note = data
        .mongo
        .notes
        .replace(new_note, &uid)
        .await
        .or(Err(AppError::Internal))?;

    Ok(HttpResponse::Ok().json(note.response()))
}

#[delete("/{note_id}")]
pub async fn delete_note(
    data: web::Data<AppData>,
    path: web::Path<String>,
    session: Session,
) -> AppResult<HttpResponse> {
    let uid = get_uid(&session)?;
    let note_id = ObjectId::parse_str(&*path).or(Err(AppError::NoteDoesNotExist))?;

    if !data
        .mongo
        .notes
        .exists(&note_id, &uid)
        .await
        .or(Err(AppError::Internal))?
    {
        return Err(AppError::NoteDoesNotExist);
    }

    data.mongo
        .notes
        .delete_one(&note_id, &uid)
        .await
        .or(Err(AppError::Internal))?;

    Ok(HttpResponse::Ok().json(json!({ "message": "Note deleted successfully" })))
}
