use mongodb::Client;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, ResolverConfig};
use futures::TryStreamExt;

use std::collections::HashMap;
use std::collections::HashSet;
use std::time::SystemTime;
use std::env;

use dotenv::dotenv;

use crate::models::{RoomResponse, NewConversation, Conversation, User, Room};
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

const DB_NAME: &str = "chatroomdb";

///
/// (Database)
#[derive(Debug, Clone)]
pub struct Database {
    users: Collection<User>,
    conversations: Collection<Conversation>,
    rooms: Collection<Room>, 
}

impl Database {
    pub async fn new() -> Self {
        dotenv().ok();

        let client_uri = env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");
        let options = ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()).await.unwrap();
        let client_conn = Client::with_options(options).unwrap();
        
        Database {
            users: client_conn.database(DB_NAME).collection("users"),
            conversations: client_conn.database(DB_NAME).collection("conversations"),
            rooms: client_conn.database(DB_NAME).collection("rooms"),
        }
    }

    pub async fn find_user(&self, username: &String) -> Result<Option<User>, DbError> {
        let filter = doc! {"_id": username};
        let query = self.users.find_one(filter, None).await?;
        
        Ok(query)
    }

    pub async fn add_user(&self, username: String, nickname: String) -> Result<User, DbError> {
        let query_result = self.find_user(&username).await?;

        match query_result {
            Some(_user) => return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Duplicate found",
            ))),

            None => {},
        };

        let user = User {
            id: username,
            nickname,
            created_at: SystemTime::now().into(),
        };

        let _insert_result = self.users.insert_one(user.clone(), None).await?;

        Ok(user)
    }

    pub async fn add_conversation(&self, new: NewConversation) -> Result<Conversation, DbError> {
        let message = Conversation {
            id: None,
            message: new.message,
            user_id: new.user_id,
            room_id: new.room_id,
            created_at: SystemTime::now().into(),
        };

        let _insert_result = self.conversations.insert_one(message.clone(), None).await?;

        Ok(message)
    }

    pub async fn get_conversations_by_room_id(&self, room_id: &String) -> Result<Vec<Conversation>, DbError> {
        let filter = doc! {"room_id": room_id};
        let query = self.conversations.find(filter, None).await?;

        let conversations = query.try_collect().await?;

        Ok(conversations)
    }

    pub async fn get_all_rooms(&self) -> Result<Vec<RoomResponse>, DbError> {
        let query = self.rooms.find(None, None).await?;
        let rooms_data: Vec<Room> = query.try_collect().await?;

        let mut ids = HashSet::new();
        let mut rooms_map = HashMap::new();
        let data = rooms_data.to_vec();

        for room in &data {
            let user_ids = &room.participant_ids;
            for id in user_ids.to_vec() {
                ids.insert(id);
            }
            rooms_map.insert(&room.id, user_ids.to_vec());
        }

        let query = self.users.find(None, None).await?;
        let users_data: Vec<User> = query.try_collect().await?;

        let users_map: HashMap<String, User> = HashMap::from_iter(
            users_data
                .into_iter()
                .map(|item| (item.id.clone(), item)),
        );
    
        let response_rooms = rooms_data.into_iter().map(|room| {
            let users = rooms_map
            .get(&room.id)
            .unwrap()
            .into_iter()
            .map(|id| users_map.get(id).unwrap().clone())
            .collect::<Vec<_>>();
    
            return RoomResponse{ room, users };
        }).collect::<Vec<_>>();
    
        Ok(response_rooms)
    }
}

/* 
pub async fn find_user(client: &Client, username: &String) -> Result<User, DbError> {
    let users = client.database(DB_NAME).collection("users");

    let query = users.find_one(Some(doc! { "_id": username }), None).await?;

    let loaded_user = match query {
        Some(doc) => doc,
        None => return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Document not found",
        ))),
    };

    let user: User = bson::from_bson(Bson::Document(loaded_user))?;

    Ok(user)
}

pub async fn insert_new_user(client: &Client, username: String, nickname: String) -> Result<User, DbError> {
    let users = client.database(DB_NAME).collection("users");

    let query_result = find_user(client, &username).await;

    match query_result {
        Ok(_user) => return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Duplicate found",
        ))),
        Err(_e) => {},
    };

    let user = User {
        id: Some(username),
        nickname: nickname,
        created_at: SystemTime::now().into(),
    };

    let serialized_user = bson::to_bson(&user)?;
    let document = serialized_user.as_document().unwrap();

    let _insert_result = users.insert_one(document.to_owned(), None).await?;

    Ok(user)
}

pub async fn insert_new_message(client: &Client, new: NewMessage) -> Result<Message, DbError> {
    let messages = client.database(DB_NAME).collection("messages");

    let message = Message {
        id: None,
        message: new.message,
        user_id: new.user_id,
        room_id: new.room_id,
        created_at: SystemTime::now().into(),
    };

    let serialized_message = bson::to_bson(&message)?;
    let document = serialized_message.as_document().unwrap();

    let _insert_result = messages.insert_one(document.to_owned(), None).await?;

    Ok(message)
}

pub async fn get_all_rooms(client: &Client) -> Result<Vec<RoomResponse>, DbError> {
    let rooms = client.database(DB_NAME).collection("rooms");
    let users = client.database(DB_NAME).collection("users");

    let mut query = rooms.find(None, None).await?;
    let mut rooms_data = Vec::new();

    while let Some(doc) = query.try_next().await? {
        let room: Room = bson::from_bson(Bson::Document(doc))?;
        rooms_data.push(room);
    }

    let mut ids = HashSet::new();
    let mut rooms_map = HashMap::new();
    let data = rooms_data.to_vec();

    for room in &data {
        let user_ids = &room.participant_ids;
        for id in user_ids.to_vec() {
            ids.insert(id);
        }
        rooms_map.insert(room.id.unwrap(), user_ids.to_vec());
    }

    let mut query = users.find(None, None).await?;
    let mut users_data = Vec::new();

    while let Some(doc) = query.try_next().await? {
        let user: User = bson::from_bson(Bson::Document(doc))?;
        if  ids.contains(user.id.as_ref().unwrap()){
            users_data.push(user);
        }
    }

    let users_map: HashMap<String, User> = HashMap::from_iter(
        users_data
            .into_iter()
            .map(|item| (item.id.clone().unwrap(), item)),
    );

    let response_rooms = rooms_data.into_iter().map(|room| {
        let users = rooms_map
        .get(room.id.as_ref().unwrap())
        .unwrap()
        .into_iter()
        .map(|id| users_map.get(id).unwrap().clone())
        .collect::<Vec<_>>();

        return RoomResponse{ room, users };
    }).collect::<Vec<_>>();

    Ok(response_rooms)
}
*/