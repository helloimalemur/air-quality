use crate::alerts::alerts::send_discord;
use crate::entities::airquality::*;
use crate::entities::sub::Sub;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::MySqlPool;
use std::collections::HashMap;

pub async fn insert_new_airquality_reading_into_db(
    new_airquality: AirQuality,
    pool: &rocket::State<MySqlPool>,
) {
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
    let _json =
        serde_json::from_str::<AirQuality>(&*req).expect("UNABLE TO DESERIALIZE, CHECK iqAPI KEY");

    let insert_url = format!(
        "http://127.0.0.1:{}/api/addaq",
        settings_map.get("port").unwrap().clone()
    );
    let _client = reqwest::Client::new()
        .post(insert_url)
        .header("Content-Type", "application/json")
        .header("x-api-key", settings_map.get("api_key").unwrap())
        .body(req)
        .send()
        .await;

    // fire_alerts(json).await;
    // add_new_airquality(json, pool);
}

pub async fn check_threshold_for_subs(new_airquality: AirQuality, pool: &rocket::State<MySqlPool>) {
    // TODO: loop on subs, compare current AQI with threshold and fire alert for those over threshold

    let subs = sqlx::query_as::<_, Sub>("SELECT * FROM subs")
        .fetch_all(&**pool)
        .await;

    if subs.is_ok() {
        for (_x, i) in subs.unwrap().iter().enumerate() {
            // iterate over subs
            if new_airquality.data.current.pollution.aqius > i.max_aqi as i64 {
                println!(
                    "{} {} || {} > {}",
                    new_airquality.data.city.clone(),
                    i.email,
                    i.max_aqi,
                    new_airquality.data.current.pollution.aqius
                );
                fire_alert(
                    i.email.to_string(),
                    i.discord.to_string(),
                    i.max_aqi.to_string(),
                    new_airquality.data.current.pollution.aqius.to_string(),
                    new_airquality.data.city.clone(),
                    pool,
                )
                .await;
            }
        }
    }
}

pub async fn fire_alert(
    email: String,
    discord: String,
    max_aqi: String,
    current_aqi: String,
    current_city: String,
    _pool: &rocket::State<MySqlPool>,
) {
    send_discord(discord, email, max_aqi, current_aqi, current_city).await;
}
