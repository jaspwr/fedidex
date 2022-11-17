use error_chain::error_chain;
use regex::Regex;
use reqwest::blocking;
use std::io::Read;
use scraper::Html;
use std::time::SystemTime;
use serde_json::Value;


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
    if body.contains("Akkoma") { return Some(ServerType::Akkoma); }
    if body.contains("<p>JavaScriptを有効にしてください<br>Please turn on your JavaScript</p>") { return Some(ServerType::Misskey); }
    if body.contains("GoToSocial") { return Some(ServerType::GoToSocial); }
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
    let mut server_type = identify_from_body(&body);
    if server_type.is_none() {
        let url = format!("{}/api/v1/instance", address);
        let instance_meta_res = web_req(url);
        if instance_meta_res.is_err() {
            return None;
        }
        let instance_meta = instance_meta_res.unwrap();
        if instance_meta.contains("Pleroma") {
            server_type = Some(ServerType::Pleroma);
        }
    }
    match server_type {
        Some(t) => get_service_metadata(address, body, t),
        None => None
    }
}

fn get_service_metadata(address: &String, body: &String, server_type: ServerType) -> Option<ServiceMeta> {
    println!("Found potential {:?} service at {}", server_type, address);
    //println!("{}", body);
    match server_type {
        ServerType::Mastodon => get_mastodon_meta(address, body, server_type),
        ServerType::Pleroma => get_pleroma_meta(address, body, server_type),
        ServerType::Akkoma => get_pleroma_meta(address, body, server_type),
        ServerType::Misskey => get_misskey_meta(address, body, server_type),
        ServerType::GoToSocial => get_mastodon_meta(address, body, server_type),
        _ => None
    }
}

fn grab_attribute_from_html(document: &Html, element_selector: &str, attribute_name: &str) -> Option<String> {
    let head_selector = scraper::Selector::parse(element_selector).unwrap();
    let elem = document.select(&head_selector).next()?;
    match elem.value().attr(attribute_name) {
        Some(s) => Some(String::from(s)),
        None => None
    }
}

fn grab_inner_from_html(document: &Html, element_selector: &str) -> Option<String> {
    let head_selector = scraper::Selector::parse(element_selector).unwrap();
    let elem = document.select(&head_selector).next()?;
    let inner = elem.inner_html();
    if inner.len() > 0 {
        Some(inner)
    } else {
        None
    }
}

fn parse_fancy_number(number: &String) -> Option<u32> {
    let re = Regex::new(r"(\d|\.)+").unwrap();
    let caps = re.captures(number)?;
    let cap = caps.get(0)?.as_str();
    match cap.to_string().parse::<f64>() {
        Ok(n) => {
            let mut multiplier = 1.0;
            if number.contains("K") { multiplier = 1.0e3; }
            else if number.contains("M") { multiplier = 1.0e6; }
            else if number.contains("B") { multiplier = 1.0e9; }
            let n = n * multiplier;
            Some(n as u32)
        },
        Err(_) => None
    }
}

fn extract_domain(url: &String) -> String {
    let mut name = url.clone();
    if name[0..7].eq("http://") {
        name = name[7..].to_string();
    } else if name[0..8].eq("https://") {
        name = name[8..].to_string();
    }
    let slash_position = name.chars().position(|c| c == '/');
    if slash_position.is_some() {
        name = name[..slash_position.unwrap()].to_string();
    }
    name
}

fn get_mastodon_meta(address: &String, body: &String, server_type: ServerType) -> Option<ServiceMeta> {
    let document = Html::parse_document(body);
    let mut favicon = grab_attribute_from_html(&document, "head > link[type=\"image/x-icon\"]", "href")?;
    let description = grab_attribute_from_html(&document, "head > meta[property=\"og:description\"]", "content")?;
    let url = format!("{}/api/v1/instance", address);
    let instance_meta_res = web_req(url).unwrap();
    let root: Value = serde_json::from_str(&instance_meta_res).unwrap_or(Value::Null);
    if root == Value::Null {
        return None;
    }
    let name = extract_domain(address);
    // Format strings
    if favicon[0..7].ne("http://") && favicon[0..8].ne("https://") {
        if favicon[0..1].eq("/") {
            favicon = format!("{}{}", address, favicon);
        } else {
            favicon = format!("{}/{}", address, favicon);
        }
    }
    Some(ServiceMeta {
        name: name,
        address: address.clone(),
        description: description,
        favicon,
        users: root.get("stats")?.get("user_count")?.as_u64()? as u32,
        invite_only: root.get("approval_required")?.as_bool()?,
        servertype: server_type,
        last_indexed: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    })
}

fn web_req (url: String) -> Result<String> {
    let mut res = blocking::get(&url)?;
    if !res.status().is_success() {
        return Err( Error {0: ErrorKind::__Nonexhaustive {}, 1: error_chain::State::default() });
    }
    let mut ret = String::new();
    res.read_to_string(&mut ret)?;
    Ok(ret)
}

fn get_pleroma_meta(address: &String, body: &String, server_type: ServerType) -> Option<ServiceMeta> {
    let url = format!("{}/api/v1/instance", address);
    let instance_meta_res = web_req(url).unwrap();
    let root: Value = serde_json::from_str(&instance_meta_res).unwrap_or(Value::Null);
    if root == Value::Null {
        return None;
    }
    let mut servertype = server_type;
    let description = root.get("description")?.as_str()?.to_string();
    if description.to_lowercase().contains("akkoma") || address.contains("akko") {
        servertype = ServerType::Akkoma;
    }
    Some(ServiceMeta {
        name: extract_domain(address),
        address: address.clone(),
        description,
        favicon: format!("{}/favicon.png", address),
        users: root.get("stats")?.get("user_count")?.as_u64()? as u32,
        invite_only: root.get("approval_required")?.as_bool()?,
        servertype,
        last_indexed: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    })
}

fn get_misskey_meta(address: &String, body: &String, server_type: ServerType) -> Option<ServiceMeta> {
    let url = format!("{}/api/meta?", address);
    let instance_meta_res = web_req(url).unwrap();
    let root: Value = serde_json::from_str(&instance_meta_res).unwrap_or(Value::Null);
    if root == Value::Null {
        return None;
    }
    Some(ServiceMeta {
        name: extract_domain(address),
        address: address.clone(),
        description: root.get("description")?.as_str()?.to_string(),
        favicon: format!("{}/favicon.png", address),
        users: root.get("stats")?.get("user_count")?.as_u64()? as u32,
        invite_only: root.get("approval_required")?.as_bool()?,
        servertype: ServerType::Akkoma,
        last_indexed: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()
    })
}