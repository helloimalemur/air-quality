use rocket::http::Status;
use rocket::outcome::{Outcome};
use rocket::request::{Request, FromRequest};


// pub struct ApiCookie<'r>(&'r str);
//
// #[derive(Debug)]
// pub enum ApiCookieError {
//     MissingError,
//     InvalidError,
// }
//
// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for ApiCookie<'r> {
//     type Error = ApiCookieError;
//
//     async fn from_request(req: &'r Request<'_>) -> Outcome<ApiCookie<'r>, (Status, ApiCookieError), ()> {
//         let sessionid = req.cookies().get("session_id");
//
//         fn is_valid(key: &str) -> bool {
//             key.len() > 0
//         }
//
//         let key_string = sessionid.clone().unwrap().to_string();
//
//         println!("{}", key_string.clone());
//         match sessionid {
//             None => Outcome::Failure((Status::BadRequest, ApiCookieError::MissingError)),
//             Some(key) if is_valid(key.value()) => Outcome::Success(ApiCookie(key.value())),
//             Some(_) => Outcome::Failure((Status::BadRequest, ApiCookieError::InvalidError)),
//         }
//     }
// }


pub struct ApiKey<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    MissingError,
    InvalidError,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<ApiKey<'r>, (Status, ApiKeyError), ()> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key == ""
        } // set your api key here TODO: derive key from config file

        match req.headers().get_one("x-api-key") {
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::MissingError)),
            Some(key) if is_valid(key) => Outcome::Success(ApiKey(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, ApiKeyError::InvalidError)),
        }
    }
}
