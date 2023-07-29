use serde::*;

#[derive(Debug, Ord, PartialOrd, Clone,PartialEq, Eq, Deserialize, Serialize,)]
pub struct AirQuality {
    #[serde(default)]
    pub id: i32,
    // TODO: airquality struct
}
