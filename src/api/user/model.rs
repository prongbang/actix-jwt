use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub firstname: String,
    pub lastname: String,
    pub phone: String,
    pub email: String,
    pub password: String,
}