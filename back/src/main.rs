#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::futures::StreamExt;
use rocket::http::Header;
use rocket::{Request, Response};
use serde_json::Value;
use tiberius::{Client, Config, AuthMethod, error::Error};
use tokio_util::compat::{TokioAsyncWriteCompatExt, Compat};
use tokio::net::TcpStream;
use async_std::sync::Mutex;
use once_cell::sync::Lazy;
use std::fs;

mod secret;
mod pinger;
use pinger::ping_web_service;

#[path = "../../shared/instance.rs"]
pub mod instance;

static DB_CLIENT: Lazy<Mutex<Option<Client<Compat<TcpStream>>>>> = Lazy::new(|| Mutex::new(None));

async fn open_sql_client() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = Config::new();
    config.host(secret::ADDRESS.clone());
    config.port(secret::PORT);
    config.trust_cert();
    config.authentication(AuthMethod::sql_server(secret::USER.clone(), secret::PASSWORD.clone()));
    config.database(secret::DATABASE.clone());
    let tcp = TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let client = match Client::connect(config, tcp.compat_write()).await {
        Ok(client) => client,
        Err(e) => Err(e)?,
    };
    *DB_CLIENT.lock().await = Some(client);
    // Housekeeping
    let contents = fs::read_to_string("../shared/blacklist.json")
        .expect("{}");
    let blacklist: Value = serde_json::from_str(&contents).unwrap_or(Value::Null);
    let blacklist = blacklist.get("list").unwrap().as_array().unwrap().iter().map(|x| {
        x.to_string()
    });
    for b in blacklist {
        let domain = b.as_str();
        let domain = domain[1..(domain.len()-1)].to_string();
        let sql = format!("
            DELETE FROM {}
            WHERE name LIKE '{}';", secret::TABLE, domain);
        let mut client = DB_CLIENT.lock().await;
        let client = client.as_mut().unwrap();
        let res = client.simple_query(sql).await;
    }
    Ok(())
}

#[get("/search/<query>/<page>")]
async fn search(query: &str, page: i32) -> String {
    let mut client = DB_CLIENT.lock().await;
    let client = client.as_mut().unwrap();
    let AMOUNT_PER_PAGE = 30;
    // TODO: Sanitize query
    let clean_query = if query.len() > 1 { query.replace("'", "&#39;")[1..].to_string() } else { ".".to_string() };
    let sql = format!("SELECT *
        FROM {}
        WHERE (CHARINDEX('{}', name) > 0 OR CHARINDEX('{}', description) > 0)
        ORDER BY users DESC
        OFFSET {} ROWS
        FETCH NEXT {} ROW ONLY;", secret::TABLE, clean_query, clean_query, page * AMOUNT_PER_PAGE, AMOUNT_PER_PAGE);
    let mut results = String::new();
    client.simple_query(&sql).await.unwrap().collect::<Vec<_>>().await.into_iter().for_each(|row| {
        let row = row.unwrap();
        let row = row.as_row();
        if row.is_none() { return; };
        let row = row.unwrap();
        let servertype = match row.get::<&str, &str>("servertype").unwrap() {
            "Mastodon" => instance::ServerType::Mastodon,
            "Pleroma" => instance::ServerType::Pleroma,
            "Akkoma" => instance::ServerType::Akkoma,
            _ => instance::ServerType::Mastodon
        };
        let instance = instance::ServiceMeta {
            address: row.get::<&str, &str>("address").unwrap().to_string(),
            servertype,
            name: row.get::<&str, &str>("name").unwrap().to_string(),
            description: row.get::<&str, &str>("description").unwrap().to_string().replace("&#39;", "'"),
            users: row.get::<i32, &str>("users").unwrap() as u32,
            favicon: row.get::<&str, &str>("favicon").unwrap().to_string(),
            invite_only: row.get::<&str, &str>("invite_only").unwrap() == "true",
            last_indexed: row.get::<i32, &str>("last_indexed").unwrap() as u64
        };
        results.push_str(&serde_json::to_string(&instance).unwrap());
        results.push_str("\n");
    });
    results
}

#[get("/submit/<address>")]
async fn submit(address: &str) -> String {
    let mut client = DB_CLIENT.lock().await;
    let client = client.as_mut().unwrap();
    match ping_web_service(address.to_string()) {
        Ok(meta) => {
            let sql = format!("
                SELECT *
                FROM {}
                WHERE address LIKE '{}';",secret::TABLE , meta.address);
            let known = client.simple_query(&sql).await.unwrap().into_row().await.unwrap().is_some();
            if known {
                let sql = format!("
                    DELETE FROM {}
                    WHERE address LIKE '{}';", secret::TABLE, meta.address);
                client.simple_query(sql).await;
            };
            let sql = format!("
                INSERT INTO {}
                VALUES ('{}', '{:#?}', '{}', '{}', {}, '{}', '{}', {}, 0);", secret::TABLE, meta.address, meta.servertype, meta.name, meta.description.replace("'", "&#39;"), meta.users, meta.favicon, meta.invite_only, meta.last_indexed);
            match client.simple_query(sql).await {
                Ok(_) => {
                    if known { "updated_known".to_string() } else { "added_unknown".to_string() }
                }
                Err(e) => {
                    format!("Error: {}", e)
                }
            }
        },
        _ => "no_instance_found".to_string()
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