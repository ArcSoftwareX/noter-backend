use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime},
    options::FindOptions,
    Client, Collection, IndexModel,
};

use super::model::note::Note;

pub struct Notes {
    client: Client,
}

impl Notes {
    pub async fn new(client: Client) -> anyhow::Result<Self> {
        client
            .database("primary")
            .collection::<Note>("notes")
            .create_index(
                IndexModel::builder()
                    .keys(doc! { "author_id": "hashed" })
                    .build(),
                None,
            )
            .await?;

        Ok(Self { client })
    }

    pub async fn get_by_author(
        &self,
        author_id: &ObjectId,
        page: u64,
        per_page: u64,
    ) -> anyhow::Result<Vec<Note>> {
        let coll = self.notes();

        let find_options = FindOptions::builder()
            .skip(page * per_page)
            .limit(per_page as i64)
            .sort(doc! { "updated_at": 1 })
            .build();
        let mut notes_c = coll
            .find(doc! { "author_id": author_id }, find_options)
            .await?;

        let mut notes = Vec::new();

        while let Some(note) = notes_c.try_next().await? {
            notes.push(note.response());
        }

        Ok(notes)
    }

    pub async fn get(&self, id: &ObjectId, uid: &ObjectId) -> anyhow::Result<Option<Note>> {
        let coll = self.notes();

        let note = coll
            .find_one(doc! { "_id": id, "author_id": uid }, None)
            .await?;

        Ok(note)
    }

    pub async fn create(&self, note: Note) -> anyhow::Result<Note> {
        let coll = self.notes();

        let id = coll
            .insert_one(&note, None)
            .await?
            .inserted_id
            .as_object_id();

        Ok(Note { id, ..note })
    }

    pub async fn replace(&self, note: Note, uid: &ObjectId) -> anyhow::Result<Note> {
        let coll = self.notes();

        coll.update_one(
            doc! { "_id": &note.id, "author_id": uid },
            doc! {
                "$set": {
                    "title": note.title.clone(),
                    "description": note.description.clone(),
                    "content": note.content.clone(),
                    "tags": note.tags.clone(),
                    "updated_at": DateTime::now()
                }
            },
            None,
        )
        .await?;

        Ok(note)
    }

    pub async fn delete_one(&self, id: &ObjectId, uid: &ObjectId) -> anyhow::Result<()> {
        let coll = self.notes();

        coll.delete_one(doc! { "_id": id, "author_id": uid }, None)
            .await?;

        Ok(())
    }

    pub async fn exists(&self, id: &ObjectId, uid: &ObjectId) -> anyhow::Result<bool> {
        Ok(self
            .notes()
            .count_documents(doc! { "_id": id, "author_id": uid }, None)
            .await?
            != 0)
    }

    fn notes(&self) -> Collection<Note> {
        self.client.database("primary").collection("notes")
    }
}
