use serde::*;

// #[derive(Debug, Ord, PartialOrd, Clone,PartialEq, Eq, Deserialize, Serialize,)]
// pub struct AirQuality {
//     #[serde(default)]
//     pub id: i32,
//     // TODO: airquality struct
// }


// CREATE TABLE `airqualtiy` (`id` int(11) NOT NULL AUTO_INCREMENT, `email` varchar(255) NOT NULL,`discord` varchar(255) NOT NULL,`additional_details` varchar(255) NOT NULL,PRIMARY KEY (`id`)) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;



#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AirQuality {
    pub status: String,
    pub data: Data,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub city: String,
    pub state: String,
    pub country: String,
    pub location: Location,
    pub forecasts: Vec<Forecast>,
    pub current: Current,
    pub history: History,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(rename = "type")]
    pub type_field: String,
    pub coordinates: Vec<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Forecast {
    pub ts: String,
    pub aqius: i64,
    pub aqicn: i64,
    pub tp: i64,
    #[serde(rename = "tp_min")]
    pub tp_min: i64,
    pub pr: i64,
    pub hu: i64,
    pub ws: i64,
    pub wd: i64,
    pub ic: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    pub weather: Weather,
    pub pollution: Pollution,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub ts: String,
    pub tp: i64,
    pub pr: i64,
    pub hu: i64,
    pub ws: i64,
    pub wd: i64,
    pub ic: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pollution {
    pub ts: String,
    pub aqius: i64,
    pub mainus: String,
    pub aqicn: i64,
    pub maincn: String,
    pub p2: P2,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct P2 {
    pub conc: f64,
    pub aqius: i64,
    pub aqicn: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub weather: Vec<Weather2>,
    pub pollution: Vec<Pollution2>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather2 {
    pub ts: String,
    pub tp: i64,
    pub pr: i64,
    pub hu: i64,
    pub ws: i64,
    pub wd: Option<i64>,
    pub ic: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Pollution2 {
    pub ts: String,
    pub aqius: i64,
    pub mainus: String,
    pub aqicn: i64,
    pub maincn: String,
    pub p2: P22,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct P22 {
    pub conc: f64,
    pub aqius: i64,
    pub aqicn: i64,
}
