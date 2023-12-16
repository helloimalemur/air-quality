#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;
use std::{io, process, thread};

mod alerts;
mod entities;
mod fairings;
mod manage_airquality;
mod manage_sub;
use crate::entities::airquality::AirQuality;
use crate::entities::sub::Sub;
use crate::fairings::apikey_fairing::ApiKey;
use crate::manage_airquality::airquality_funcs::add_new_airquality_reading;
use config::Config;
use log::info;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config as LogConfig;
use manage_sub::sub_funcs::add_new_sub;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::NamedFile;
use rocket::http::{Header, Status};
use rocket::request::Request;
use rocket::tokio::time::{interval_at, Instant};
use rocket::Response;
use rocket::{custom, tokio};
use sqlx::MySqlPool;
use tokio::task::JoinHandle;

// // // // // // // // // // // // // // // // // // // // // // // //
// // // // // // // // // // // // // // // // // // // // // // // //

#[get("/")]
async fn index(socket_addr: SocketAddr, pool: &rocket::State<MySqlPool>) -> NamedFile {
    let cur_dir = process::Command::new("pwd").output().unwrap().stdout;
    let page_directory_path = format!(
        "{}/airquality-web/build/",
        String::from_utf8(cur_dir).unwrap().trim()
    );
    println!("{}", page_directory_path);
    NamedFile::open(Path::new(&page_directory_path).join("index.html"))
        .await
        .unwrap()
}

#[get("/<file..>")]
async fn files(file: PathBuf) -> NamedFile {
    let cur_dir = process::Command::new("pwd").output().unwrap().stdout;
    let page_directory_path = format!(
        "{}/airquality-web/build/",
        String::from_utf8(cur_dir).unwrap().trim()
    );
    println!("{}", page_directory_path);
    NamedFile::open(Path::new(&page_directory_path).join(file))
        .await
        .unwrap()
}

#[get("/static/<file..>")]
async fn static_files(file: PathBuf) -> NamedFile {
    let cur_dir = process::Command::new("pwd").output().unwrap().stdout;
    let page_directory_path = format!(
        "{}/airquality-web/build/",
        String::from_utf8(cur_dir).unwrap().trim()
    );
    println!("{}", page_directory_path);
    NamedFile::open(Path::new(&page_directory_path).join(file))
        .await
        .unwrap()
}

// // // // // // // // // // // // // // // // // // // // // // // //
// // // // // // // // // // // // // // // // // // // // // // // //

// Add subscriber
#[post("/api/addsub", data = "<data>")]
async fn addsub(
    socket_addr: SocketAddr,
    pool: &rocket::State<MySqlPool>,
    settings_map: &rocket::State<HashMap<String, String>>,
    data: Json<Sub>,
    key: ApiKey<'_>,
) -> &'static str {
    // let mail_data = data.clone();
    if key.0.to_string() == settings_map.get("api_key").unwrap().to_string() {
        add_new_sub(data, pool).await;
        info!(target:"app::requests", "ADD SUB - From: {}", socket_addr.ip().to_string());
        "sub added"
    } else {
        "invalid api-key"
    }
}

// Add air quality reading
#[post("/api/addaq", data = "<data>")]
async fn addaq(
    socket_addr: SocketAddr,
    pool: &rocket::State<MySqlPool>,
    settings_map: &rocket::State<HashMap<String, String>>,
    data: Json<AirQuality>,
    key: ApiKey<'_>,
) -> &'static str {
    if key.0.to_string() == settings_map.get("api_key").unwrap().to_string() {
        add_new_airquality_reading(data, pool).await;
        info!(target:"app::requests", "ADD AQ - From: {}", socket_addr.ip().to_string());
        "reading added"
    } else {
        "invalid api-key"
    }
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

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
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

    #[allow(unused_variables)]
    let config = LogConfig::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        // .logger(Logger::builder().build("app::backend::db", LevelFilter::Info))
        .logger(
            Logger::builder()
                .appender("requests")
                .additive(true)
                .build("app::requests", LevelFilter::Info),
        )
        .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
        .unwrap();

    info!(target: "app::requests","Starting");

    // debug
    // println!("{:#?}", settings_map);
    let _database_name = settings_map.get("database_name").unwrap().as_str();
    ////////////

    let _who = process::Command::new("hostname")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .is_ok();

    let database_url: &str;
    database_url = "";
    println!("{}", database_url.clone());
    let database_url = &settings_map.get("database_url").unwrap().as_str();

    // println!("{}", database_url);
    // launch rocket

    let config = rocket::Config {
        port: settings_map.clone().get("port").unwrap().parse().unwrap(),
        address: std::net::Ipv4Addr::new(0, 0, 0, 0).into(),
        ..rocket::Config::debug_default()
    };

    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("database connection");

    let moved_settings = settings_map.clone();
    tokio::spawn(async move {
        let start = Instant::now();
        let mut interval = interval_at(start, tokio::time::Duration::from_secs(21600));

        loop {
            manage_airquality::airquality_funcs::fetch_data_fire_alerts(moved_settings.clone())
                .await;
            interval.tick().await;
        }
    });

    let moved_settings_two = settings_map.clone();
    let alt_thread = tokio::spawn(async move {
        tokio::time::sleep(Duration::new(7, 0)).await;
        // println!("{:?}", moved_settings_two);
        let output = process::Command::new("bash")
            .arg("-e")
            .arg("./node.sh")
            .output()
            .unwrap()
            .stdout;
        println!("{:?}", String::from_utf8(output).unwrap());
    });

    custom(&config)
        .manage::<MySqlPool>(pool)
        .manage::<HashMap<String, String>>(settings_map.clone())
        .manage::<JoinHandle<_>>(alt_thread)
        .mount("/", routes![index, files, addaq, addsub])
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
