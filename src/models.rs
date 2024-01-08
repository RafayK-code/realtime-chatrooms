use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use chrono::{DateTime, Utc};

/// A model for a conversation document in our database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Conversation {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub message: String,
    pub user_id: String,
    pub room_id: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
}

/// A model for a room document in our database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Room {
    #[serde(rename = "_id")]
    pub id: String,
    pub last_message: String,
    pub participant_ids: Vec<String>,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>, 
}

/// A model for a user document in our database
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: String,
    pub nickname: String,
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub created_at: DateTime<Utc>,
}

/// Collection of information required to make a User document
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewUser {
    pub username: String,
    pub nickname: String,
}

/// Collection of information required to make a Conversation document
#[derive(Serialize, Deserialize, Debug)]
pub struct NewConversation {
    pub user_id: String,
    pub room_id: String,
    pub message: String,
}

/// Represents a room and all the users associated with that room
#[derive(Serialize, Deserialize, Debug)]
pub struct RoomResponse {
    pub room: Room,
    pub users: Vec<User>,
}