use mongodb::bson::{self, doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::app::schema::notes::CreateNoteSchema;

use super::{super::utils::option_oid_to_string, date::Date, oid::Oid};

#[derive(Serialize, Deserialize)]
pub struct Note {
    #[serde(
        rename(deserialize = "_id"),
        serialize_with = "option_oid_to_string",
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<ObjectId>,
    pub author_id: Oid,

    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,

    pub tags: Option<Vec<String>>,

    pub updated_at: Date,
    pub created_at: Date,
}

impl Note {
    // self is not passed by reference intentionally
    pub fn response(self) -> Self {
        Self {
            author_id: match self.author_id {
                Oid::ObjectId(oid) => Oid::String(oid.to_string()),
                s => s,
            },
            updated_at: self.updated_at.chrono(),
            created_at: self.created_at.chrono(),
            ..self
        }
    }

    pub fn from_create_schema(schema: CreateNoteSchema, uid: ObjectId) -> Self {
        Self {
            id: None,
            author_id: Oid::ObjectId(uid),
            title: schema.title,
            description: schema.description,
            content: schema.content,
            tags: schema.tags,
            updated_at: Date::Mongo(bson::DateTime::from_chrono(schema.updated_at)),
            created_at: Date::Mongo(bson::DateTime::from_chrono(schema.created_at)),
        }
    }
}
