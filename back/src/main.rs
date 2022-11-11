#[macro_use] extern crate rocket;

use std::pin::Pin;
use std::task::Context;

use rocket::fs::{FileServer, relative};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::futures::StreamExt;
use rocket::http::Header;
use rocket::{Request, Response};
mod scanner;
use scanner::ping_web_service;
pub mod instance;
use tiberius::Row;
use tiberius::{Client, Config, AuthMethod, error::Error};
use tokio_util::compat::{TokioAsyncWriteCompatExt, Compat};
use tokio::net::TcpStream;
use async_std::sync::Mutex;
use once_cell::sync::Lazy;

mod secret;

static DB_CLIENT: Lazy<Mutex<Option<Client<Compat<TcpStream>>>>> = Lazy::new(|| Mutex::new(None));

async fn open_sql_client() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();

    let secret = secret::get();
    config.host(secret.address);
    config.port(secret.port);
    config.trust_cert();
    config.authentication(AuthMethod::sql_server(secret.user, secret.password));
    config.database(secret.database);
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let mut client = match Client::connect(config, tcp.compat_write()).await {
        Ok(client) => client,
        Err(e) => Err(e)?,
    };
    *DB_CLIENT.lock().await = Some(client);
    Ok(())
}

#[get("/search/<query>/<page>")]
async fn search(query: &str, page: i32) -> String {
    let mut client = DB_CLIENT.lock().await;
    let client = client.as_mut().unwrap();
    // TODO: Sanitize query
    let clean_query = query.replace("'", "&#39;")[1..].to_string();
    let SQL = format!("SELECT *
        FROM instances0
        WHERE (CHARINDEX('{}', name) > 0 OR CHARINDEX('{}', description) > 0)
        ORDER BY id ASC;", clean_query, clean_query);
        let mut results = String::new();

        client.simple_query(&SQL).await.unwrap().collect::<Vec<_>>().await.into_iter().for_each(|row| {
            let row = row.unwrap();
            let row = row.as_row();
            if row.is_none() { return; };
            let row = row.unwrap();
            let name = row.get::<&str, &str>("name").unwrap().to_string();
            let description = row.get::<&str, &str>("description").unwrap().to_string();
            let address = row.get::<&str, &str>("address").unwrap().to_string();
            //let users = row.get::<i32, &str>("users").unwrap();
            let favicon = row.get::<&str, &str>("favicon").unwrap().to_string();
            let invite_only = row.get::<&str, &str>("invite_only").unwrap() == "true";
            //let last_indexed = row.get::<i64, &str>("last_indexed").unwrap();
            let servertype = row.get::<&str, &str>("servertype").unwrap();
            let servertype = match servertype {
                "Mastodon" => instance::ServerType::Mastodon,
                "Pleroma" => instance::ServerType::Pleroma,
                _ => instance::ServerType::Mastodon
            };
            let instance = instance::ServiceMeta {
                address,
                servertype,
                name,
                description,
                //users: users as u32,
                users: 4,
                favicon,
                invite_only,
                //last_indexed: last_indexed as u64
                last_indexed: 2
            };
            results.push_str(&serde_json::to_string(&instance).unwrap());
            results.push_str("\n");
        });
        //println!("{:?}", results);
        results
    }

#[get("/submit/<address>")]
async fn submit(address: &str) -> String {
    let mut client = DB_CLIENT.lock().await;
    let client = client.as_mut().unwrap();
    match ping_web_service(address.to_string()) {
        Ok(meta) => {

            let SQL = format!("INSERT INTO instances0
                VALUES ('{}', '{:#?}', '{}', '{}', {}, '{}', '{}', {}, 0);", meta.address, meta.servertype, meta.name, meta.description.replace("'", "&#39;"), meta.users, meta.favicon, meta.invite_only, meta.last_indexed);
            println!("{}", SQL);
            match client.simple_query(SQL).await {
                Ok(_) => {
                    "Success".to_string()
                }
                Err(e) => {
                    format!("Error: {}", e)
                }
            }
            //serde_json::to_string(&meta).unwrap()
        },
        _ => "No service was found...".to_string()
    }
}

#[launch]
async fn rocket() -> _ {
    match open_sql_client().await {
        Ok(()) => {
            rocket::build()
            .attach(Cors)
            .mount("/", routes![search, submit])
            .mount("/", FileServer::from(relative!("../front/dist")))
        },
        Err(e) => {
            println!("Failed to connect to SQL Server: {}", e);
            rocket::build()
        }
    }
}

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}