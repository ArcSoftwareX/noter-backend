use mongodb::{Client, GridFsBucket};

pub struct Avatars {
    pub bucket: GridFsBucket,
}

impl Avatars {
    pub fn new(client: Client) -> Self {
        let bucket = client.database("avatars").gridfs_bucket(None);

        Self { bucket }
    }
}
