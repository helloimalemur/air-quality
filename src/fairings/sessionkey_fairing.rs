use rocket::http::Status;
use rocket::outcome::{Outcome};
use rocket::request::{Request, FromRequest};
use sqlx::MySqlPool;


pub struct SessionKey<'r>(&'r str);

#[derive(Debug)]
pub enum SessionKeyError {
    MissingError,
    InvalidError,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for SessionKey<'r> {
    type Error = SessionKeyError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<SessionKey<'r>, (Status, SessionKeyError), ()> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key.len() > 0
        }

        match req.headers().get_one("x-session-key") {
            None => Outcome::Failure((Status::BadRequest, SessionKeyError::MissingError)),
            Some(key) if is_valid(key) => Outcome::Success(SessionKey(key)),
            Some(_) => Outcome::Failure((Status::BadRequest, SessionKeyError::InvalidError)),
        }
    }
}


pub trait FromSessionKey<'r> {
    fn stringify(cookie: SessionKey) -> String;
}

impl<'r> FromSessionKey<'r> for SessionKey<'r> {
    fn stringify(cookie: SessionKey) -> String {
        // verify_session("test", cookie.0.to_string(), pool);
        cookie.0.to_string()
    }
}
