use std::collections::HashMap;
use std::fmt::format;
use reqwest::header::{CONTENT_TYPE, HeaderMap, HeaderValue};
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};
use sqlx::{MySql, MySqlPool, Pool, Row};
use crate::entities::airquality::*;
use crate::entities::sub::Sub;
use crate::manage_airquality::airquality_funcs;
use crate::manage_sub::sub_funcs;
use futures::{StreamExt, TryStreamExt};
use sqlx::mysql::MySqlRow;
use crate::alerts::alerts::send_discord;


pub async fn insert_new_airquality_reading_into_db(new_airquality: AirQuality, pool: &rocket::State<MySqlPool>) {
    let insert = sqlx::query(
        "INSERT INTO readings (city, state, temp, pressure, humidity, wind_speed, current_pollution_aqius, main_pollutant)
        VALUES (?,?,?,?,?,?,?,?)")
        .bind(new_airquality.data.city)
        .bind(new_airquality.data.state)
        .bind(new_airquality.data.current.weather.tp)
        .bind(new_airquality.data.current.weather.pr)
        .bind(new_airquality.data.current.weather.hu)
        .bind(new_airquality.data.current.weather.ws)
        .bind(new_airquality.data.current.pollution.aqius)
        .bind(new_airquality.data.current.pollution.mainus)
        .execute(&**pool)
        .await.unwrap();
    println!("{:?}", insert);
}


pub async fn add_new_airquality_reading(data: Json<AirQuality>, pool: &State<MySqlPool>) {
    let json = data.clone().0;

    check_threshold_for_subs(json.clone(), pool).await;

    insert_new_airquality_reading_into_db(json, pool).await;
}


pub async fn fetch_data_fire_alerts(
    settings_map: HashMap<String, String>,
    // data: Json<AirQuality>,
    // sub: Json<Sub>
) {
    println!("fetching..");
    let key = settings_map.get("iqair_key").unwrap();
    let url = format!("http://api.airvisual.com/v2/nearest_city?key={}", key);
    let req = reqwest::get(url).await.unwrap().text().await.unwrap();
    let json = serde_json::from_str::<AirQuality>(&*req).expect("UNABLE TO DESERIALIZE, CHECK iqAPI KEY");


    let client = reqwest::Client::new().post("http://127.0.0.1:8080/api/addaq")
        .header("Content-Type","application/json")
        .header("x-api-key",settings_map.get("api_key").unwrap())
        .body(req)
        .send().await;

    // debug
    // println!("{:?}", json);


    // fire_alerts(json).await;
    // add_new_airquality(json, pool);
}

pub async fn check_threshold_for_subs(new_airquality: AirQuality, pool: &rocket::State<MySqlPool>) {
    // TODO: loop on subs, compare current AQI with threshold and fire alert for those over threshold

    let mut subs = sqlx::query_as::<_, Sub>("SELECT * FROM subs")
        .fetch_all(&**pool).await;

    if subs.is_ok() {
        for (x,i) in subs.unwrap().iter().enumerate() {
            // iterate over subs

            if i.max_aqi as i64 > new_airquality.data.current.pollution.aqius {
            // if new_airquality.data.current.pollution.aqius > i.max_aqi as i64 {
                println!("{} || {} > {}", i.email, i.max_aqi, new_airquality.data.current.pollution.aqius);
                fire_alert(i.email.to_string(), i.discord.to_string(), i.max_aqi.to_string(), new_airquality.data.current.pollution.aqius.to_string(), pool).await;
            }
        }
    }
}

pub async fn fire_alert(email: String, discord: String, max_aqi: String, current_aqi: String, pool: &rocket::State<MySqlPool>) {
    send_discord(discord, email, max_aqi, current_aqi).await;
}
