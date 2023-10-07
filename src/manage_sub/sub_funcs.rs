use crate::entities::sub::Sub;
use crate::manage_sub::sub_funcs;
use rocket::serde::json::Json;
use rocket::State;
use sqlx::MySqlPool;

pub async fn new_sub(new_sub: Sub, pool: &rocket::State<MySqlPool>) {
    let _insert = sqlx::query(
        "INSERT INTO subs (email, discord, ip, location, additional_details, max_aqi)
        VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(new_sub.email)
    .bind(new_sub.discord)
    .bind(new_sub.ip)
    .bind(new_sub.location)
    .bind(new_sub.additional_details)
    .bind(new_sub.max_aqi)
    .execute(&**pool)
    .await
    .unwrap();
}

pub async fn add_new_sub(data: Json<Sub>, pool: &State<MySqlPool>) {
    let new_sub = Sub {
        id: rocket::serde::__private::Default::default(),
        email: data.email.to_string(),
        discord: data.discord.to_string(),
        ip: data.ip.to_string(),
        location: data.location.to_string(),
        additional_details: data.additional_details.to_string(),
        max_aqi: data.max_aqi,
    };
    sub_funcs::new_sub(new_sub, pool).await;
}
