use std::collections::HashMap;
use std::fmt::format;
use rocket::serde::json::Json;
use rocket::State;
use serde_json::{json, Value};
use sqlx::{MySqlPool};
use crate::entities::airquality::*;
use crate::entities::sub::Sub;
use crate::manage_airquality::airquality_funcs;
use crate::manage_sub::sub_funcs;

pub async fn new_airquality(new_airquality: AirQuality, pool: &rocket::State<MySqlPool>) {
    let _insert = sqlx::query(
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
}


pub async fn add_new_airquality(data: Json<AirQuality>, pool: &State<MySqlPool>) {
    let json = data.clone();
    let new_airquality = AirQuality {
        status: json.status.to_string(),
        data: Data {
            city: json.data.city.to_string(),
            state: json.data.state.to_string(),
            country: json.data.country.to_string(),
            location: Location { type_field: json.data.location.type_field.to_string(), coordinates: vec![] },
            // forecasts: vec![],
            current: Current {
                weather: Weather {
                    ts: json.data.current.weather.ts.to_string(),
                    tp: json.data.current.weather.tp,
                    pr: json.data.current.weather.pr,
                    hu: json.data.current.weather.hu,
                    ws: json.data.current.weather.ws,
                    wd: json.data.current.weather.wd,
                    ic: json.data.current.weather.ic.to_string(),
                },
                pollution: Pollution {
                    ts: json.data.current.pollution.ts.to_string(),
                    aqius: json.data.current.pollution.aqius,
                    mainus: json.data.current.pollution.mainus.to_string(),
                    aqicn: json.data.current.pollution.aqicn,
                    maincn: json.data.current.pollution.maincn.to_string(),
                    // p2: P2 {
                    //     conc: json.data.current.pollution.p2.conc,
                    //     aqius: json.data.current.pollution.p2.aqius,
                    //     aqicn: json.data.current.pollution.p2.aqicn,
                    // },
                }
            },
            // history: History { weather: vec![], pollution: vec![] },
        },
    };

    println!("{new_airquality:?}");

    airquality_funcs::new_airquality(new_airquality, pool).await;
}


pub async fn fetch_data_fire_alerts(settings_map: HashMap<String, String>) {
    println!("fetching..");
    let key = settings_map.get("iqair_key").unwrap();
    let url = format!("http://api.airvisual.com/v2/nearest_city?key={}", key);
    let req = reqwest::get(url).await.unwrap().text().await.unwrap();
    println!("{:?}", req.clone().replace("\\",""));
    let json = serde_json::from_str::<AirQuality>(&*req).unwrap();
    
    // work around to deserializing issue for now
    // println!("{}", json.get("status").unwrap().to_string());

    // let entry: AirQuality = serde_json::from_str(json.as_str().unwrap()).unwrap();

    // let new_entry: AirQuality = AirQuality { status: "".to_string(), data: Data {
    //     city: "".to_string(),
    //     state: "".to_string(),
    //     country: "".to_string(),
    //     location: Location { type_field: "".to_string(), coordinates: vec![] },
    //     forecasts: vec![],
    //     current: Current {
    //         weather: Weather {
    //             ts: "".to_string(),
    //             tp: 0,
    //             pr: 0,
    //             hu: 0,
    //             ws: 0,
    //             wd: 0,
    //             ic: "".to_string(),
    //         },
    //         pollution: Pollution {
    //             ts: "".to_string(),
    //             aqius: 0,
    //             mainus: "".to_string(),
    //             aqicn: 0,
    //             maincn: "".to_string(),
    //             p2: P2 {
    //                 conc: 0.0,
    //                 aqius: 0,
    //                 aqicn: 0,
    //             },
    //         }
    //     },
    //     history: History { weather: vec![], pollution: vec![] },
    // } };


    // let iqair_data: AirQuality = serde_json::from_value(json).unwrap();
    // println!("{:?}", iqair_data.status);

    // fire_alerts(json);
    // add_new_airquality(json, pool);
}

pub async fn fire_alerts(json: Value) {
    todo!()
}
