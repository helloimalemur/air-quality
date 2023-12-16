use rocket::http::Status;
use rocket::outcome::Outcome;
use rocket::request::{FromRequest, Request};

#[derive(PartialEq)]
pub struct ApiKey<'r>(pub(crate) &'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    MissingError,
    InvalidError,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for ApiKey<'r> {
    type Error = ApiKeyError;

    async fn from_request(
        req: &'r Request<'_>,
    ) -> Outcome<ApiKey<'r>, (Status, ApiKeyError), Status> {
        /// Returns true if `key` is a valid API key string.
        fn is_valid(key: &str) -> bool {
            key.len() > 0
        }

        match req.headers().get_one("x-api-key") {
            None => Outcome::Success(ApiKey("")),
            Some(key) => Outcome::Success(ApiKey(key)),
        }
    }
}

// impl stringify for ApiKey<'_> {
//     fn stringify(key: ApiKey) -> String {
//         key.0.to_string()
//     }
// }
