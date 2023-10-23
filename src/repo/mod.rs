use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};

use crate::{app::config::Config, repo::avatar::Avatars};

use self::{notes::Notes, users::Users};

pub struct MongoRepo {
    pub users: Users,
    pub notes: Notes,
    pub avatars: Avatars,
}

impl MongoRepo {
    pub async fn new(config: &Config) -> anyhow::Result<Self> {
        log::info!("Establishing database connection...");

        let client = if cfg!(debug_assertions) {
            Client::with_uri_str(&config.mongo_uri).await?
        } else {
            let mut client_options = ClientOptions::parse(&config.mongo_uri).await?;
            let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();

            client_options.server_api = Some(server_api);

            Client::with_options(client_options)?
        };

        log::info!("Database connection established");

        Ok(Self {
            users: Users::new(client.clone()),
            notes: Notes::new(client.clone()).await?,
            avatars: Avatars::new(client.clone()),
        })
    }
}

pub mod avatar;
pub mod model;
pub mod notes;
pub mod users;
pub mod utils;
