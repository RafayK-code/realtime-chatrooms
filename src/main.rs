#[macro_use] extern crate rocket;
use realtime_chatrooms::message::{Message, post, events};
use rocket::{tokio::sync::broadcast::channel, fs::{relative, FileServer}};


#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![post, events])
        .mount("/", FileServer::from(relative!("static")))
}