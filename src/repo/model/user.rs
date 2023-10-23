use super::{super::utils::option_oid_to_string, date::Date};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(
        rename(deserialize = "_id"),
        skip_serializing_if = "Option::is_none",
        serialize_with = "option_oid_to_string"
    )]
    pub id: Option<ObjectId>,
    pub email: String,
    pub name: Option<String>,
    pub avatar_id: Option<ObjectId>,
    pub created_at: Date,
}

impl User {
    pub fn response(self) -> Self {
        Self {
            created_at: self.created_at.chrono(),
            ..self
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct WithPassword<T> {
    pub password: String,
    #[serde(flatten)]
    pub inner: T,
}

impl<T> WithPassword<T> {
    pub fn new(inner: T, password: String) -> Self {
        Self { password, inner }
    }
}
