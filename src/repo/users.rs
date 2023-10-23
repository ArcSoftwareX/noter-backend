use anyhow::Context;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Client, Collection,
};

use crate::repo::model::user::{User, WithPassword};

pub struct Users {
    client: Client,
}

impl Users {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, doc: WithPassword<User>) -> anyhow::Result<WithPassword<User>> {
        let coll = self.users();

        let res = coll.insert_one(&doc, None).await?;

        let id = res
            .inserted_id
            .as_object_id()
            .context("failed to retrieve inserted object id")?;

        Ok(WithPassword {
            inner: User {
                id: Some(id),
                ..doc.inner
            },
            ..doc
        })
    }

    pub async fn get(&self, id: &ObjectId) -> anyhow::Result<Option<WithPassword<User>>> {
        let coll = self.users();

        let res = coll.find_one(doc! { "_id": id }, None).await?;

        Ok(res)
    }

    pub async fn get_by_email(&self, email: &str) -> anyhow::Result<Option<WithPassword<User>>> {
        let coll = self.users();

        let res = coll.find_one(doc! { "email": email }, None).await?;

        Ok(res)
    }

    pub async fn update_avatar_id(
        &self,
        id: &ObjectId,
        avatar_id: &ObjectId,
    ) -> anyhow::Result<()> {
        let coll = self.users();

        coll.update_one(
            doc! { "_id": id },
            doc! { "$set": { "avatar_id": avatar_id } },
            None,
        )
        .await?;

        Ok(())
    }

    pub async fn exists(&self, email: &str) -> anyhow::Result<bool> {
        let coll = self.users();

        let exists = coll.count_documents(doc! { "email": email }, None).await? != 0;

        Ok(exists)
    }

    fn users(&self) -> Collection<WithPassword<User>> {
        self.client.database("primary").collection("users")
    }
}
