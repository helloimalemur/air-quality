use rocket::http::Status;
use rocket::outcome::{Outcome};
use rocket::request::{Request, FromRequest};
use sqlx::MySqlPool;
use crate::manage_sessions::session_funcs::verify_session_by_session_id;


pub struct ApiCookie<'r>(&'r str);

#[derive(Debug)]
pub enum ApiCookieError {
    MissingError,
    InvalidError,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiCookie<'r> {
    type Error = ApiCookieError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<ApiCookie<'r>, (Status, ApiCookieError), ()> {
        let sessionid = req.cookies().get("session_id");
        // println!("{:?}", sessionid);

        fn is_valid(key: &str) -> bool {
            key.len() > 0
        }

        let key_string = sessionid.clone().unwrap().to_string();

        // println!("{}", key_string.clone());
        match sessionid {
            None => Outcome::Failure((Status::BadRequest, ApiCookieError::MissingError)),
            Some(key) if is_valid(key.value()) => Outcome::Success(ApiCookie(key.value())),
            Some(_) => Outcome::Failure((Status::BadRequest, ApiCookieError::InvalidError)),
        }
    }
}

pub trait FromCookie<'r> {
    fn stringify(cookie: &'r ApiCookie<'r>, pool: &rocket::State<MySqlPool>) -> String;
}

impl<'r> FromCookie<'r> for ApiCookie<'r> {
    fn stringify(cookie: &'r ApiCookie<'r>, pool: &rocket::State<MySqlPool>) -> String {
        // verify_session("test", cookie.0.to_string(), pool);
        cookie.0.to_string()
    }
}
