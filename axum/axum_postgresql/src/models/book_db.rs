use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub genre: String,
}

pub struct BookDetail {
    pub id_num: String,
    pub title: String,
    pub author: String,
    pub genre: String,
}