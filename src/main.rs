#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::process;
use std::process::Stdio;
mod entities;
mod manage_sub;
mod manage_airquality;
mod fairings;
use manage_sub::*;
use manage_sub::sub_funcs::add_new_sub;
use sqlx::{MySqlPool, Row};
use config::Config;
use rocket::tokio::time::{interval_at, Instant};
use rocket::{custom, tokio};
use rocket::http::{Cookie, Header, Status};
use rocket::request::{FromRequest, Request};
use rocket::{Response};
use rocket::fairing::{Fairing, Info, Kind};
use crate::fairings::apikey_fairing::ApiKey;
use rocket::http::{CookieJar};
use crate::entities::sub::Sub;
use log::{error, info, warn};
use log::{debug, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::Config as LogConfig;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Logger, Root};
use rocket::outcome::Outcome;
use crate::entities::airquality::AirQuality;
use crate::manage_airquality::airquality_funcs::add_new_airquality;


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


// curl -XPOST -H 'Content-Type:application/json' -H 'x-api-key:YOURAPIKEY' http://127.0.0.1:8030/api/addcontact -d '{"email": "test","discord": "test","additional_details": "test"}'
#[post("/api/addsub", data = "<data>")]
async fn addsub(
    socket_addr: SocketAddr,
    pool: &rocket::State<MySqlPool>,
    data: Json<Sub>,
    _key: ApiKey<'_>,
) -> Result<(), ErrorResponder> {
    // let mail_data = data.clone();
    info!(target:"app::requests", "ADD SUB - From: {}", socket_addr.ip().to_string());
    add_new_sub(data, pool).await;
    Ok(())
}

// curl -XPOST -H 'Content-Type:application/json' -H 'x-api-key:YOURAPIKEY' http://127.0.0.1:8080/api/addaq -d ''
#[post("/api/addaq", data = "<data>")]
async fn addaq(
    socket_addr: SocketAddr,
    pool: &rocket::State<MySqlPool>,
    settings_map: &rocket::State<HashMap<String, String>>,
    data: Json<AirQuality>,
    key: ApiKey<'_>,
) -> Result<(), ErrorResponder> {
    info!(target:"app::requests", "ADD AQ - From: {}", socket_addr.ip().to_string());
    if key.0.to_string() == settings_map.get("api_key").unwrap().to_string() {
        add_new_airquality(data, pool).await;
    }
    Ok(())
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
    let database_url = &settings_map.get("database_url").unwrap().as_str();


    println!("{}", database_url);
    // launch rocket


    let config = rocket::Config {
        port: settings_map.clone().get("port").unwrap().parse().unwrap(),
        address: std::net::Ipv4Addr::new(0, 0, 0, 0).into(),
        ..rocket::Config::debug_default()
    };

    let pool = MySqlPool::connect(&database_url).await.expect("database connection");


    let moved_settings = settings_map.clone();
    tokio::spawn(async move {
        let start = Instant::now();
        let mut interval = interval_at(start, tokio::time::Duration::from_secs(300));

        loop {
            interval.tick().await;
            manage_airquality::airquality_funcs::fetch_data_fire_alerts(moved_settings.clone()).await;
        }
    });

    // let moved_settings_two = settings_map.clone();
    // tokio::spawn(async move {

    // });

    custom(&config)
        .manage::<MySqlPool>(pool)
        .manage::<HashMap<String, String>>(settings_map.clone())
        .mount(
            "/",
            routes![
                index,
                addaq,
                addsub
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
