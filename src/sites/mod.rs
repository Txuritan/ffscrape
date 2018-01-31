use std::vec;
use std::string;

use chrono;
use vec_map;

pub mod fanfiction;
pub mod fanfiction_v2;

#[derive(Serialize, Deserialize, Debug)]
pub struct Chapter {
    content: string::String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Story {
    pub title: string::String,
    pub author: string::String,
    pub fandoms: vec::Vec<string::String>,
    pub rating: string::String,
    // pub language: string::String,
    // pub genres: vec_map::VecMap<string::String>,
    // pub characters: vec_map::VecMap<string::String>,
    // pub words: u32,
    // pub favorites: u32,
    // pub follows: u32,
    // pub published: chrono::DateTime<chrono::offset::Utc>,
    // pub updated: chrono::DateTime<chrono::offset::Utc>,
    // pub chapters: vec_map::VecMap<string::String>,
}

pub enum Sites {
    FanFictionNet,
}
