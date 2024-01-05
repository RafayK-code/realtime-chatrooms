use std::time::Instant;

use actix::*;
use actix_files::NamedFile;
use actix_web::{Responder, HttpRequest, web, HttpResponse, Error, post, get};
use actix_web_actors::ws;
use serde_json::json;
use tokio::runtime::Runtime;

use crate::{database, server, session, models};

pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

pub async fn chat_server(req: HttpRequest, stream: web::Payload, db: web::Data<database::Database>, srv: web::Data<Addr<server::ChatServer>>) -> Result<HttpResponse, Error> {
    ws::start(
        session::WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "main".to_string(),
            name: None,
            addr: srv.get_ref().clone(),
            db,
        }, 
        &req, 
        stream
    )
}

#[post("/users/create")]
pub async fn create_user(db: web::Data<database::Database>, form: web::Json<models::NewUser>) -> Result<HttpResponse, Error> {
    let user = web::block(move || {
        Runtime::new().expect("Fatal error").block_on(db.add_user(form.username.clone(), form.nickname.clone()))
    })
    .await?
    .map_err(actix_web::error::ErrorUnprocessableEntity)?;

    Ok(HttpResponse::Ok().json(user))
}

#[get("/users/{username}")]
pub async fn get_user(db: web::Data<database::Database>, username: web::Path<String>) -> Result<HttpResponse, Error> {
    let id = username.to_owned();
    let user = web::block(move || {
        Runtime::new().expect("Fatal error").block_on(db.find_user(&id))
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
       return Ok(HttpResponse::Ok().json(user));
    }

    let res = HttpResponse::NotFound().body(
        json!({
            "error": 404,
            "message": format!("No user found with username: {username}")
        })
        .to_string(),
    );

    Ok(res)
}

#[get("/conversations/{room_id}")]
pub async fn get_conversation_by_id(db: web::Data<database::Database>, room_id: web::Path<String>) -> Result<HttpResponse, Error> {
    let id = room_id.to_owned();
    let conversations = web::block(move || {
        Runtime::new().expect("Fatal error").block_on(db.get_conversations_by_room_id(&id))
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if !conversations.is_empty() {
        return Ok(HttpResponse::Ok().json(conversations));
    }

    let res = HttpResponse::NotFound().body(
        json!({
            "error": 404,
            "message": format!("No conversation with room id: {room_id}")
        })
        .to_string(),
    );

    Ok(res)
}

#[get("/rooms")]
pub async fn get_rooms(db: web::Data<database::Database>) -> Result<HttpResponse, Error> {
    let rooms = web::block(move || {
        Runtime::new().expect("Fatal error").block_on(db.get_all_rooms())
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if !rooms.is_empty() {
        return Ok(HttpResponse::Ok().json(rooms));
    }

    let res = HttpResponse::NotFound().body(
        json!({
            "error": 404,
            "message": format!("No rooms available")
        })
        .to_string(),
    );

    Ok(res)
}