use std::string;
use std::vec;

use chrono;
use vec_map;

#[derive(Serialize, Deserialize, Debug)]
pub struct FanFiction {
    pub title: string::String,
    pub author: string::String,
    pub fandoms: vec::Vec<string::String>,
    pub rating: string::String,
    pub language: string::String,
    pub genres: vec::Vec<string::String>,
    pub characters: vec::Vec<string::String>,
    pub words: u32,
    pub favorites: u32,
    pub follows: u32,
    pub published: chrono::DateTime<chrono::offset::Utc>,
    pub updated: chrono::DateTime<chrono::offset::Utc>,
    pub chapters: vec_map::VecMap<string::String>,
}
