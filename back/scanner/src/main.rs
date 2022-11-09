use error_chain::error_chain;
use regex::{Regex, Match};
use reqwest::blocking;
use std::f32::consts::E;
use std::io::Read;
use std::{fmt, string};
use std::option;
use fantoccini::{Client, Locator};


error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[derive(Debug)]
enum ServerType {
    Mastodon,
    Pleroma
}

struct ServiceMeta {
    address: String,
    servertype: ServerType,
    name: String,
    description: String,
    users: u32,
    favicon: String,
    invite_only: bool,
    last_indexed: i32
}

fn identify_from_body(body: &String) -> Option<ServerType> {
    if body.contains("Mastodon") { return Some(ServerType::Mastodon); }
    if body.contains("Pleroma") { return Some(ServerType::Pleroma); }
    None
}

fn ping_web_service(address: String) -> Result<()> {
    let url = format!("https://{}", address);
    let mut res = blocking::get(&url)?;
    if !res.status().is_success() {
        return Ok(());
    }

    let mut body = String::new();
    res.read_to_string(&mut body)?;
    initial_identify_ping(&url, &body);
    Ok(())
}

fn initial_identify_ping (address: &String, body: &String) -> Option<ServiceMeta> {
    let server_type = identify_from_body(&body);
    match server_type {
        Some(t) => get_service_metadata(address, body, t),
        None => None
    }
}

fn get_service_metadata(address: &String, body: &String, server_type: ServerType) -> Option<ServiceMeta> {
    println!("Found potential {:?} service at {}", server_type, address);
    match server_type {
        ServerType::Mastodon => get_mastodon_meta(address, body, server_type),
        ServerType::Pleroma => get_pleroma_meta(address, body, server_type)
    }
}

fn get_mastodon_meta(address: &String, body: &String, server_type: ServerType) -> Option<ServiceMeta> {
    let document = scraper::Html::parse_document(body);
    let head_selector = scraper::Selector::parse("head  > link, meta, title").unwrap();
    let titles = document.select(&head_selector).map(|x| x.html());
    titles
        .zip(1..)
        .for_each(|(item, number)| println!("{}. {}", number, item));
    None
}

fn get_pleroma_meta(address: &String, body: &String, server_type: ServerType) -> Option<ServiceMeta> {
    None
}

fn main() -> () {
    ping_web_service(String::from("activism.openworlds.info")).unwrap();
}