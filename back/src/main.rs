#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
mod scanner;
use scanner::*;
pub mod instance;

#[get("/search/<query>")]
fn search(query: &str) -> String {
    "{\"address\":\"https://peoplemaking.games\",\"servertype\":\"Mastodon\",\"name\":\"peoplemaking.games\",\"description\":\"We're a community of folks who celebrate the craft of creating interactive experiences, whether they're working in games or around them! This is a space for games folks and creatives to call home!\",\"users\":0,\"favicon\":\"https://peoplemaking.games/favicon.ico\",\"invite_only\":false,\"last_indexed\":1668076736}\n{\"address\":\"https://peoplemaking.games\",\"servertype\":\"Mastodon\",\"name\":\"peoplemaking.games\",\"description\":\"We're a community of folks who celebrate the craft of creating interactive experiences, whether they're working in games or around them! This is a space for games folks and creatives to call home!\",\"users\":0,\"favicon\":\"https://peoplemaking.games/favicon.ico\",\"invite_only\":false,\"last_indexed\":1668076736}\n{\"address\":\"https://peoplemaking.games\",\"servertype\":\"Mastodon\",\"name\":\"peoplemaking.games\",\"description\":\"We're a community of folks who celebrate the craft of creating interactive experiences, whether they're working in games or around them! This is a space for games folks and creatives to call home!\",\"users\":0,\"favicon\":\"https://peoplemaking.games/favicon.ico\",\"invite_only\":false,\"last_indexed\":1668076736}".to_string()
    //let serialized = serde_json::to_string(&meta).unwrap();
}

#[get("/submit/<address>")]
fn submit(address: &str) -> String {
    match ping_web_service(address.to_string()) {
        Ok(meta) => serde_json::to_string(&meta).unwrap(),
        _ => "No service was found...".to_string()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
    .mount("/", routes![search, submit])
    .mount("/", FileServer::from(relative!("../front/dist")))
}