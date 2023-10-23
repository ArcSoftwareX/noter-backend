use std::sync::Arc;

#[derive(Clone)]
pub struct Config {
    pub cookie_secret: Arc<str>,
    pub mongo_uri: Arc<str>,
    pub app_uri: Arc<Option<String>>,
}

impl Config {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            cookie_secret: std::env::var("COOKIE_SECRET")?.into(),
            mongo_uri: std::env::var("MONGO_URI")?.into(),
            app_uri: Arc::new(std::env::var("APP_URI").ok()),
        })
    }
}
