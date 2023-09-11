use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub text: String,
    pub author: i64,
}

#[derive(Debug, Deserialize)]
pub struct NewArticle {
    pub title: String,
    pub text: String,
    pub author: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProfilePic {
    #[serde(rename = "profile_picture")]
    pub data: Vec<u8>,
}

impl From<Vec<u8>> for ProfilePic {
    fn from(data: Vec<u8>) -> Self {
        Self { data }
    }
}
