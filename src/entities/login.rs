use serde::*;

#[derive(Debug, Ord, PartialOrd, Clone,PartialEq, Eq, Deserialize, Serialize,)]
pub struct Login {
    pub username: String,
    pub password: String,
    pub ipaddress: String,
}


// impl User {
//     pub async fn find_by_username(username: String, pool: &State<MySqlConnection>) -> Result<User> {
//         let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = ?", username)
//             .fetch_one(&*pool)
//             .await?;
//
//         Ok(user)
//     }
// }
