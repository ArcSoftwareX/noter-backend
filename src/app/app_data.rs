use crate::repo::MongoRepo;

use super::config::Config;

pub struct AppData {
    pub mongo: MongoRepo,
}

impl AppData {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        let mongo = MongoRepo::new(config).await?;

        Ok(Self { mongo })
    }
}
