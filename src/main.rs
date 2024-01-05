extern crate mongodb;
use mongodb::bson;
use mongodb::bson::oid::ObjectId;
use mongodb::options::ResolverConfig;
use mongodb::{Client, options::ClientOptions};

use actix::*;
use actix_cors::Cors;
use actix_files::Files;
use actix_web::{web, http, App, HttpServer};
use tokio;

use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::str::FromStr;
use std::time::SystemTime;

mod database;
mod models;
mod routes;
mod server;
mod session;

/* 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = server::ChatServer::new().start();
    let conn_spec = "chat.db";
    let server_addr = "127.0.0.1";
    let server_port = 8080;
    let app = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(web::Data::new(server.clone()))
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .service(web::resource("/").to(routes::index))
            .route("/ws", web::get().to(routes::chat_server))
            .service(routes::create_user)
            .service(routes::get_user_by_id)
            .service(routes::get_user_by_phone)
            .service(routes::get_conversation_by_id)
            .service(routes::get_rooms)
            .service(Files::new("/", "./static"))
    })
    .workers(2)
    .bind((server_addr, server_port))?
    .run();


    println!("Server running at http://{server_addr}:{server_port}/");
    app.await
}
*/
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = server::ChatServer::new().start();
    let server_addr = "127.0.0.1";
    let server_port = 8080;
    let db = database::Database::new().await;
    let app = HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        
        App::new()
            .app_data(web::Data::new(server.clone()))
            .app_data(web::Data::new(db.clone()))
            .wrap(cors)
            .service(web::resource("/").to(routes::index))
            .route("/ws", web::get().to(routes::chat_server))
            .service(routes::create_user)
            .service(routes::get_user)
            .service(routes::get_conversation_by_id)
            .service(routes::get_rooms)
            .service(Files::new("/", "./static"))
    })
    .workers(2)
    .bind((server_addr, server_port))?
    .run();

    Ok(())
}