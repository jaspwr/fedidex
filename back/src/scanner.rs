use error_chain::error_chain;
use regex::{Regex, Match};
use reqwest::blocking;
use std::f32::consts::E;
use std::io::Read;
use std::ops::Index;
use std::{fmt, string};
use std::option;
use fantoccini::{Client, Locator};
use scraper::Html;
use std::time::SystemTime;


use crate::instance::*;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
    skip_msg_variant
}

fn identify_from_body(body: &String) -> Option<ServerType> {
    if body.contains("Mastodon") { return Some(ServerType::Mastodon); }
    if body.contains("Pleroma") { return Some(ServerType::Pleroma); }
    None
}

pub fn ping_web_service(address: String) -> Result<ServiceMeta> {
    let url = format!("https://{}", address);
    let mut res = blocking::get(&url)?;
    if !res.status().is_success() {
        return Err( Error {0: ErrorKind::__Nonexhaustive {}, 1: error_chain::State::default() });
    }

    //TODO: bail if the stream is extreemly long
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    let meta = initial_identify_ping(&url, &body);
    match meta {
        Some(m) => Ok(m),
        None => Err( Error {0: ErrorKind::__Nonexhaustive {}, 1: error_chain::State::default() })
    }
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

fn grap_attribute_from_html(document: &Html, element_selector: &str, attribute_name: &str) -> Option<String> {
    let head_selector = scraper::Selector::parse(element_selector).unwrap();
    let elem = document.select(&head_selector).next()?;
    match elem.value().attr(attribute_name) {
        Some(s) => Some(String::from(s)),
        None => None
    }
}

fn get_mastodon_meta(address: &String, body: &String, server_type: ServerType) -> Option<ServiceMeta> {
    let document = Html::parse_document(body);

    // Do all html selectors to see if valid page before doing anything else
    let mut favicon = grap_attribute_from_html(&document, "head > link[type=\"image/x-icon\"]", "href")?;
    let mut name = grap_attribute_from_html(&document, "head > meta[property=\"og:url\"]", "content")?;
    let description = grap_attribute_from_html(&document, "head > meta[property=\"og:description\"]", "content")?;

    // Format strings
    if favicon[0..7].ne("http://") && favicon[0..8].ne("https://") {
        if favicon[0..1].eq("/") {
            favicon = format!("{}{}", address, favicon);
        } else {
            favicon = format!("{}/{}", address, favicon);
        }
    }

    if name[0..7].eq("http://") {
        name = name[7..].to_string();
    } else if name[0..8].eq("https://") {
        name = name[8..].to_string();
    }
    let slash_position = name.chars().position(|c| c == '/');
    if slash_position.is_some() {
        name = name[..slash_position.unwrap()].to_string();
    }

    Some(ServiceMeta {
        name: name,
        address: address.clone(),
        description: description,
        favicon: favicon,
        users: 0,
        invite_only: false,
        servertype: ServerType::Mastodon,
        last_indexed: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    })
}

fn get_pleroma_meta(address: &String, body: &String, server_type: ServerType) -> Option<ServiceMeta> {
    None
}
