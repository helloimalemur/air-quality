use serde::*;

#[derive(Debug, Ord, PartialOrd, Clone,PartialEq, Eq, Deserialize, Serialize,)]
pub struct Websession {
    // pub id: i32,
    pub user_name: String,
    pub session_id: String,
}

// CREATE TABLE `web_sessions` (`id` int(11) NOT NULL AUTO_INCREMENT,`user_name` varchar(255) NOT NULL,`session_id` varchar(255) NOT NULL,PRIMARY KEY (`id`)) ENGINE=InnoDB AUTO_INCREMENT=48 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
