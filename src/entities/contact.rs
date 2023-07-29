use serde::*;

#[derive(Debug, Ord, PartialOrd, Clone,PartialEq, Eq, Deserialize, Serialize,)]
pub struct Contact {
    #[serde(default)]
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: String,
    pub message: String,
    pub additional_details: String,
}

// CREATE TABLE `contact_me` (`id` int(11) NOT NULL AUTO_INCREMENT,`first_name` varchar(255) NOT NULL,`last_name` varchar(255) NOT NULL,`email` varchar(255) NOT NULL,`phone` varchar(255) NOT NULL,`message` varchar(255) NOT NULL,`additional_details` varchar(255) NOT NULL,PRIMARY KEY (`id`)) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
