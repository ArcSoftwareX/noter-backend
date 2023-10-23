use mongodb::bson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Date {
    Chrono(chrono::DateTime<chrono::Utc>),
    Mongo(bson::DateTime),
}

impl Date {
    pub fn chrono(self) -> Self {
        match self {
            Date::Mongo(dt) => Date::Chrono(dt.to_chrono()),
            chrono_dt => chrono_dt,
        }
    }

    pub fn mongo(self) -> Self {
        match self {
            Date::Chrono(dt) => Date::Mongo(bson::DateTime::from_chrono(dt)),
            dt => dt,
        }
    }
}
