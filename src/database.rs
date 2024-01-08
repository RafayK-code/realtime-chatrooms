use mongodb::Client;
use mongodb::Collection;
use mongodb::bson::doc;
use mongodb::options::{ClientOptions, ResolverConfig};
use futures::TryStreamExt;
use tokio::runtime::Runtime;

use std::collections::HashMap;
use std::collections::HashSet;
use std::time::SystemTime;
use std::env;

use dotenv::dotenv;

use crate::models::{RoomResponse, NewConversation, Conversation, User, Room};
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

const DB_NAME: &str = "chatroomdb";

/// A struct containing collections of Users, Conversations, and Rooms in our database
#[derive(Debug, Clone)]
pub struct Database {
    users: Collection<User>,
    conversations: Collection<Conversation>,
    rooms: Collection<Room>, 
}

impl Database {
    /// Returns an instance of our Database with designated handles to each collection.
    /// 
    /// # Arguments
    /// 
    /// * `key` - A string slice that holds the environment variable to conenct to the database
    /// 
    /// # Panics
    /// 
    /// If there is an error connecting to the client
    /// 
    /// # Examples 
    /// 
    /// ```
    /// let db = Database::new("MONGODB_URI");
    /// ```
    pub fn new(key: &str) -> Self {
        dotenv().ok();

        let client_uri = env::var(key).expect("You must set the MONGODB_URI environment var!");
        let options = Runtime::new().expect("Fatal error").block_on(
            ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare()))
            .unwrap();
        let client_conn = Client::with_options(options).unwrap();
        
        Database {
            users: client_conn.database(DB_NAME).collection("users"),
            conversations: client_conn.database(DB_NAME).collection("conversations"),
            rooms: client_conn.database(DB_NAME).collection("rooms"),
        }
    }

    /// Finds a user from the database with the given username
    /// 
    /// # Examples
    /// 
    /// ```
    /// let db = Database::new("MONGODB_URI");
    /// let user_result = db.find_user("user1").await.unwrap();
    /// 
    /// match user_result {
    ///     Some(user) => println!("User was found");
    ///     None => println!("User could not be found");
    /// }
    /// ```
    pub async fn find_user(&self, username: &str) -> Result<Option<User>, DbError> {
        let filter = doc! {"_id": username};
        let query = self.users.find_one(filter, None).await?;
        
        Ok(query)
    }

    /// Helper function to find a room from the database with the given id
    async fn find_room(&self, room_id: &str) -> Result<Option<Room>, DbError> {
        let filter = doc! {"_id": room_id};
        let query = self.rooms.find_one(filter, None).await?;

        Ok(query)
    }

    /// Creates and inserts a new user in the database with a given username and nickname
    /// 
    /// # Examples
    /// 
    /// ```
    /// let db = Database::new("MONGODB_URI");
    /// let new_user_result = db.add_user("user2".to_owned(), "jimmy".to_owned()).await;
    /// 
    /// match new_user_result {
    ///     Ok(_user) => println!("User inserted successfully!");
    ///     Err(error) => match error.kind() {
    ///         ErrorKind::AlreadyExists => println!("User with that username already exists!");
    ///         _ => println!("Some other error happened!");
    ///     }
    /// }
    /// ```
    pub async fn add_user(&self, username: String, nickname: String) -> Result<User, DbError> {
        let query_result = self.find_user(&username).await?;

        match query_result {
            Some(_user) => return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
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

    /// Creates and inserts a conversation into a database
    /// 
    /// # Paramters
    /// 
    /// * `new` - A struct containing the message contents, the username of the user that sent it, and the id of the room the message was sent in
    /// 
    /// # Examples
    /// 
    /// ```
    /// let db = Database::new("MONGODB_URI");
    /// let conversation_result = db.add_conversation(NewConversation {
    ///     message: "Hello World!".to_owned(),
    ///     user_id: "user1".to_owned(),
    ///     room_id: "main".to_owned(),
    /// }).await;
    /// 
    /// match conversation_result {
    ///     Ok(_convo) => println!("Conversation inserted successfully!");
    ///     Err(error) => match error.kind() {
    ///         ErrorKind::NotFound => println!("User or room does not exist!");
    ///         _ => println!("Some other error happened!");
    ///     }
    /// }
    /// ```
    pub async fn add_conversation(&self, new: NewConversation) -> Result<Conversation, DbError> {
        let try_user = self.find_user(new.user_id.as_str()).await?;
        match try_user {
            Some(_user) => {},

            None => return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "User does not exist",
            ))),
        }

        let try_room = self.find_room(new.room_id.as_str()).await?;
        match try_room {
            Some(_room) => {},

            None => return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Room does not exist",
            ))),
        }

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

    /// Retrives all conversations in a given room
    /// 
    /// # Examples
    /// 
    /// ```
    /// let db = Database::new("MONGODB_URI");
    /// let conversations_result = db.get_conversations_by_room_id("main")
    /// 
    /// match conversations_result {
    ///     Ok(_convos) => println!("Conversations retrieved!");
    ///     Err(error) => match error.kind() {
    ///         ErrorKind::NotFound => println!("Room does not exist!");
    ///         _ => println!("Some other error happened!");
    ///     }
    /// }
    /// ```
    pub async fn get_conversations_by_room_id(&self, room_id: &str) -> Result<Vec<Conversation>, DbError> {
        let try_room = self.find_room(room_id).await?;
        match try_room {
            Some(_room) => {},

            None => return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Room does not exist",
            ))),
        }

        let filter = doc! {"room_id": room_id};
        let query = self.conversations.find(filter, None).await?;

        let conversations: Vec<Conversation> = query.try_collect().await?;

        Ok(conversations)
    }

    /// Retrieves all rooms and their associated users
    /// 
    /// # Examples
    /// 
    /// ```
    /// let db = Database::new("MONGODB_URI");
    /// let rooms_result = db.get_all_rooms();
    /// 
    /// let rooms = match rooms_result {
    ///     Ok(rooms) => rooms,
    ///     Err(e) => panic!("Some error happened {:?}", e);
    /// }
    /// ```
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