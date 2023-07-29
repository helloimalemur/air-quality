#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::process;
use std::process::Stdio;
mod entities;
mod manage_sessions;
use manage_sessions::*;
mod manage_users;
use manage_users::*;
mod manage_cookies;
mod fairings;
mod manage_contact;
mod emailer;
use emailer::*;

use manage_contact::*;
use manage_contact::contact_funcs::add_new_contact;
use sqlx::{MySqlPool, Row};
use config::Config;
use rocket::tokio::time::{interval_at, Instant};
use rocket::{custom, tokio};
use rocket::http::{Cookie, Header, Status};
use rocket::request::{Request};
use rocket::{Response};
use rocket::fairing::{Fairing, Info, Kind};
use crate::entities::login::Login;
use crate::entities::users::User;
use crate::fairings::cookie_fairing::{ApiCookie, FromCookie};
use crate::fairings::apikey_fairing::ApiKey;
use crate::fairings::sessionkey_fairing::SessionKey;
use crate::fairings::sessionkey_fairing::FromSessionKey;
use crate::manage_sessions::session_funcs::{check_user, create_session_key, does_a_session_exist, get_username_by_session, verify_session_by_session_id, verify_session_by_user_and_session_id, write_session_to_db};
use rocket::http::{CookieJar};
use crate::entities::contact::Contact;
use crate::manage_users::user_funcs::{create_new_user, login_user};
use log::{error, info, warn};
use log::{debug, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::Config as LogConfig;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Logger, Root};
use crate::emailer::emailer::send_email;


// // // // // // // // // // // // // // // // // // // // // // // //
// // // // // // // // // // // // // // // // // // // // // // // //

// curl -XGET --cookie "session_id=anythingrightnow" http://127.0.0.1:8000/
#[get("/")]
async fn index(
    socket_addr: SocketAddr,
    pool: &rocket::State<MySqlPool>
) -> &'static str {
    let is_pool_closed= pool.is_closed();
    info!(target:"app::requests", "ROOT PATH - From: {}", socket_addr.ip().to_string());
    if is_pool_closed {
        "No Swimming"
    } else {
        "Hello, Astronauts!"
    }
}


// curl -XGET -H 'x-api-key:YOURAPIKEY' https://.net/api/<session_id>
#[get("/api/<session_id>")]
async fn get_user(
    socket_addr: SocketAddr,
    session_id: String,
    pool: &rocket::State<MySqlPool>,
    _key: ApiKey<'_>
) -> String {
    let result = get_username_by_session(session_id, pool).await;
    info!(target:"app::requests", "GET USER via SESSION - From: {}, USER_RESULT: {}", socket_addr.ip().to_string(), result);
    result
}
// // // //


// curl -XPOST -H 'Content-Type:application/json' -H 'x-api-key:YOURAPIKEY' http://127.0.0.1:8000/adduser -d '{"username": "test","password": "test","email": "test","first_name": "test","last_name": "test"}'
#[post("/api/adduser", data = "<data>")]
async fn adduser(
    socket_addr: SocketAddr,
    pool: &rocket::State<MySqlPool>,
    data: Json<User>,
    // _key: ApiKey<'_>,
) -> Result<(), ErrorResponder> {
    let res = create_new_user(data.clone(), pool).await;
    info!(target:"app::requests", "ADD USER - From: {}, SUCCESS: {}, USER: {}", socket_addr.ip().to_string(), res, data.clone().username);
    Ok(())
}


// curl -XPOST -H 'Content-Type:application/json' http://127.0.0.1:8000/login -d '{"username": "user","password": "password"}'
#[post("/api/login", data = "<data>")]
async fn login(
    pool: &rocket::State<MySqlPool>,
    data: Json<Login>,
) -> Result<String, ErrorResponder> {
    let key = login_user(pool, data.clone()).await;
    let mut success = false;
    if key.len() >30 {
        success = true;
    }
    info!(target:"app::requests", "LOGIN REQUEST - From: {}, SUCCESS: {}, USER: {}", data.clone().ipaddress, success, data.clone().username);
    Ok(key)
}


// curl -XGET https://.net/api/logout/session_id/
#[get("/api/logout/<session_id>")]
async fn logout(
    socket_addr: SocketAddr,
    pool: &rocket::State<MySqlPool>,
    session_id: String
) -> Result<(), Status> {
    let mut success = false;
    let mut user = String::from("");
    if verify_session_by_session_id( session_id.clone(), pool).await {
        session_funcs::delete_session_from_db_by_session_id(session_id.clone(), pool).await;
        success = true;
        println!("deleting sessions for {}", session_id.clone());
    }
    info!(target:"app::requests", "LOGOUT USER - From: {}, SUCCESS: {}, USER: {}", socket_addr.ip().to_string(), success, user);
    Ok(())
}


// // curl -XGET -H 'x-api-key:YOURAPIKEY' http://127.0.0.1:8000/verify/user/sessionid
// #[get("/api/verify")]
// async fn verify(session: SessionKey<'_>, pool: &rocket::State<MySqlPool>) -> Result<(), Status> {
//     // let key: String = FromSessionKey::stringify(session);
//     // let result = verify_session_by_session_id(key, pool).await;
//
//     // println!("{}", result);
//     Ok(())
// }

// curl -XGET http://127.0.0.1:8000/verify/sessionid
#[get("/api/verify/<session_id>")]
async fn verify_by_session(
    socket_addr: SocketAddr,
    session_id: String,
    pool: &rocket::State<MySqlPool>
) -> &'static str {
    let mut verified = "false";
    let mut success = false;
    let mut user = String::from("");
    if verify_session_by_session_id(session_id.clone(), pool).await {
        verified = "true";
        success = true;
        user = get_username_by_session(session_id.clone(),pool).await
    }
    info!(target:"app::requests", "VERIFY USER SESSION - From: {}, SUCCESS: {}, USER: {}, SESSION: {}", socket_addr.ip().to_string(), success, user, session_id.clone());
    verified
}


// curl -XPOST -H 'Content-Type:application/json' -H 'x-api-key:YOURAPIKEY' http://127.0.0.1:8030/api/addcontact -d '{"first_name": "test","last_name": "test","email": "test","phone": "test","message": "test","additional_details": "test"}'
#[post("/api/addcontact", data = "<data>")]
async fn addcontact(
    socket_addr: SocketAddr,
    pool: &rocket::State<MySqlPool>,
    data: Json<Contact>,
    _key: ApiKey<'_>,
) -> Result<(), ErrorResponder> {
    let mail_data = data.clone();
    info!(target:"app::requests", "ADD CONTACT - From: {}", socket_addr.ip().to_string());
    add_new_contact(data, pool).await;
    let message = format!("Message from: {} {} \n
    Email: {} \n
    Message: {} \n
    Details: {} \n
    ", mail_data.first_name.as_str(), mail_data.last_name.as_str(), mail_data.email.as_str(), mail_data.message.as_str(), mail_data.additional_details);
    send_email("you@email.net", "noreply@.net", "Contact Form Message", message.as_str());
    Ok(())
}



// #[get("/cookiecheck")]
// async fn check_cookie(cookie: ApiCookie<'_>, pool: &rocket::State<MySqlPool>) -> &'static str {
//     let key = ApiCookie::stringify(&cookie, pool);
//     println!("{}", key);
//     if verify_session_by_session_id(key.to_string(), pool).await {
//         "verified"
//     } else { "invalid" }
// }

#[get("/api/cookiecheck")]
async fn check_cookie(cookie: ApiCookie<'_>, pool: &rocket::State<MySqlPool>) -> &'static str {
    let key = ApiCookie::stringify(&cookie, pool);
    println!("{}", key);
    if verify_session_by_session_id(key.to_string(), pool).await {
        "verified"
    } else { "invalid" }
}

// // // // // // // // // // // // // // // // // // // // // // // //
// // // // // // // // // // // // // // // // // // // // // // // //


pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        // response.set_header(Header::new("Access-Control-Allow-Origin", "https://yourlinuxadmin.com/"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_status(Status::new(200));
    }
}

#[rocket::main]
pub async fn main() {
    // load config
    let settings = Config::builder()
        .add_source(config::File::with_name("config/Settings"))
        .build()
        .unwrap();
    let settings_map = settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap();

    //logging

    let stdout = ConsoleAppender::builder().build();

    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build("log/requests.log")
        .unwrap();

    let config = LogConfig::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        // .logger(Logger::builder().build("app::backend::db", LevelFilter::Info))
        .logger(Logger::builder().appender("requests").additive(true).build("app::requests", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
        .unwrap();

    let handle = log4rs::init_config(config).unwrap();

    info!(target: "app::requests","Starting");


    // debug
    // println!("{:#?}", settings_map);
    let _database_name = settings_map.get("database_name").unwrap().as_str();
    ////////////

    let who = process::Command::new("hostname")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .is_ok();

    let mut database_url: &str;
    database_url = "";
    println!("{}", database_url.clone());
    // if std::str::from_utf8(&who.stdout).unwrap().contains("v1")
    if !who {
        database_url = settings_map.get("database_url").unwrap().as_str()
    } else {
        database_url = settings_map.get("remote_database_url").unwrap().as_str();
    }

    println!("{}", database_url);
    // launch rocket
    tokio::spawn(async {
        let start = Instant::now();
        let mut interval = interval_at(start, tokio::time::Duration::from_secs(5));

        loop {
            interval.tick().await;
        }
    });

    let config = rocket::Config {
        port: 8030,
        address: std::net::Ipv4Addr::new(0, 0, 0, 0).into(),
        ..rocket::Config::debug_default()
    };

    let pool = MySqlPool::connect(&database_url).await.expect("database connection");

    custom(&config)
        .manage::<MySqlPool>(pool)
        .mount(
            "/",
            routes![
                index,
                adduser,
                addcontact,
                login,
                logout,
                // verify,
                verify_by_session,
                get_user,
            ],
        )
        .attach(CORS)
        .launch()
        .await
        .unwrap();

}

// The following impl's are for easy conversion of error types.
#[derive(Responder)]
#[response(status = 500, content_type = "json")]
struct ErrorResponder {
    message: String,
}


impl From<anyhow::Error> for ErrorResponder {
    fn from(err: anyhow::Error) -> ErrorResponder {
        ErrorResponder {
            message: err.to_string(),
        }
    }
}


impl From<String> for ErrorResponder {
    fn from(string: String) -> ErrorResponder {
        ErrorResponder { message: string }
    }
}

impl From<&str> for ErrorResponder {
    fn from(str: &str) -> ErrorResponder {
        str.to_owned().into()
    }
}
