use serde::*;

#[derive(Debug, Ord, PartialOrd, Clone,PartialEq, Eq, Deserialize, Serialize,)]
pub struct AirQuality {
    #[serde(default)]
    pub id: i32,
    // TODO: airquality struct
}


// CREATE TABLE `airqualtiy` (`id` int(11) NOT NULL AUTO_INCREMENT, `email` varchar(255) NOT NULL,`discord` varchar(255) NOT NULL,`additional_details` varchar(255) NOT NULL,PRIMARY KEY (`id`)) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
