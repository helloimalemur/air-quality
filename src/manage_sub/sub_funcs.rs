use rocket::serde::json::Json;
use rocket::State;
use sqlx::{MySqlPool};
use crate::entities::sub::Sub;
use crate::manage_sub::sub_funcs;

pub async fn new_sub(new_sub: Sub, pool: &rocket::State<MySqlPool>) {
    let _insert = sqlx::query(
        "INSERT INTO sub (email, discord, additional_details)
        VALUES (?, ?, ?)")
        .bind(new_sub.email)
        .bind(new_sub.discord)
        .bind(new_sub.additional_details)
        .execute(&**pool)
        .await.unwrap();
}


pub async fn add_new_sub(data: Json<Sub>, pool: &State<MySqlPool>) {

    let new_sub = Sub {
        id: rocket::serde::__private::Default::default(),
        email: data.email.to_string(),
        discord: "".to_string(),
        additional_details: data.additional_details.to_string(),
        max_aqi: data.max_aqi,
    };
    sub_funcs::new_sub(new_sub, pool).await;
}
