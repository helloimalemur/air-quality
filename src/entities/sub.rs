use serde::*;
#[derive(sqlx::FromRow, Debug, Ord, PartialOrd, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Sub {
    #[serde(default)]
    pub id: i32,
    pub email: String,
    pub discord: String,
    pub ip: String,
    pub location: String,
    pub additional_details: String,
    pub max_aqi: i32,
}
