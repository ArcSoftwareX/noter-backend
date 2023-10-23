use mongodb::bson::oid::ObjectId;
// use serde::ser::SerializeStruct;
use serde::Serializer;
// use serde::{Deserialize, Deserializer};

// pub fn from_oid<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
// where
//     D: Deserializer<'de>,
// {
//     let s: ObjectId = Deserialize::deserialize(deserializer)?;

//     Ok(Some(s.to_string()))
// }

// pub fn oid_to_string<S>(oid: &ObjectId, s: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     s.serialize_str(oid.to_string().as_str())
// }

// pub fn note_oid_helper<S>(oid: &ObjectId, s: S, id: Option<ObjectId>) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     if id.is_some() {
//         s.serialize_str(oid.to_string().as_str())
//     } else {
//         let mut state = s.serialize_struct("ObjectId", 1)?;
//         state.serialize_field("$oid", &oid.to_string())?;
//         state.end()
//     }
// }

pub fn option_oid_to_string<S>(oid: &Option<ObjectId>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(oid.unwrap().to_string().as_str())
}
