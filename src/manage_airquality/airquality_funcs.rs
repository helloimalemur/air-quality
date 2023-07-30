use rocket::serde::json::Json;
use rocket::State;
use sqlx::{MySqlPool};
use crate::entities::airquality::*;
use crate::entities::sub::Sub;
use crate::manage_airquality::airquality_funcs;
use crate::manage_sub::sub_funcs;

// pub async fn new_airquality(new_airquality: AirQuality, pool: &rocket::State<MySqlPool>) {
//     let _insert = sqlx::query(
//         "INSERT INTO sub (email, discord, additional_details)
//         VALUES (?, ?, ?)")
//
//         .execute(&**pool)
//         .await.unwrap();
// }


pub async fn add_new_airquality(data: Json<AirQuality>, pool: &State<MySqlPool>) {
    let json = data.clone();
    let new_airquality = AirQuality {
        id: Default::default(),
        status: json.status.to_string(),
        data: Data {
            city: json.data.city.to_string(),
            state: json.data.state.to_string(),
            country: json.data.country.to_string(),
            location: Location { type_field: json.data.location.type_field.to_string(), coordinates: vec![] },
            forecasts: vec![],
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
                    p2: P2 {
                        conc: json.data.current.pollution.p2.conc,
                        aqius: json.data.current.pollution.p2.aqius,
                        aqicn: json.data.current.pollution.p2.aqicn,
                    },
                }
            },
            history: History { weather: vec![], pollution: vec![] },
        },
    };

    println!("{new_airquality:?}")

    // airquality_funcs::new_airquality(new_airquality, pool).await;
}
