use rocket::serde::json::Json;
use rocket::State;
use sqlx::{MySqlPool};
use crate::entities::contact::Contact;
use crate::manage_contact::contact_funcs;

pub async fn new_contact(new_contact: Contact, pool: &rocket::State<MySqlPool>) {
    let _insert = sqlx::query(
        "INSERT INTO contact_me (first_name, last_name, email, phone, message, additional_details)
        VALUES (?, ?, ?, ?, ?, ?)")
        .bind(new_contact.first_name)
        .bind(new_contact.last_name)
        .bind(new_contact.email)
        .bind(new_contact.phone)
        .bind(new_contact.message)
        .bind(new_contact.additional_details)
        .execute(&**pool)
        .await.unwrap();
}


pub async fn add_new_contact(data: Json<Contact>, pool: &State<MySqlPool>) {

    let new_contact = Contact {
        id: rocket::serde::__private::Default::default(),
        first_name: data.first_name.to_string(),
        last_name: data.last_name.to_string(),
        email: data.email.to_string(),
        phone: data.phone.to_string(),
        message: data.message.to_string(),
        additional_details: data.additional_details.to_string(),
    };
    contact_funcs::new_contact(new_contact, pool).await;
}
