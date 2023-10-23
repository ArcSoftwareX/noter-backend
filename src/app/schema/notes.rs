use serde::Deserialize;

// use crate::repo::model::note::{NotePriority, NoteStatus};

#[derive(Deserialize)]
pub struct ReplaceNoteSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,

    // pub priority: Option<NotePriority>,
    // pub status: Option<NoteStatus>,
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct CreateNoteSchema {
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,

    // pub priority: Option<NotePriority>,
    // pub status: Option<NoteStatus>,
    pub tags: Option<Vec<String>>,

    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
