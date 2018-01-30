use std::string;

use chrono;
use scraper;
use vec_map;

#[derive(Serialize, Deserialize, Debug)]
pub struct  Chapter {
    content: string::String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FanFiction {
    pub title: string::String,
    pub author: string::String,
    pub fandoms: vec_map::VecMap<string::String>,
    pub rating: string::String,
    pub language: string::String,
    pub genres: vec_map::VecMap<string::String>,
    pub characters: vec_map::VecMap<string::String>,
    pub words: u32,
    pub favorites: u32,
    pub follows: u32,
    pub published: chrono::DateTime<chrono::offset::Utc>,
    pub updated: chrono::DateTime<chrono::offset::Utc>,
    pub chapters: vec_map::VecMap<string::String>,
}

pub fn get_info(document: scraper::Html, url: &str) -> (
    string::String, // Title
    // string::String, // Author
    // vec_map::VecMap<string::String>, // Fandoms
    // string::String, // Rating
    // string::String, // Language
    // vec_map::VecMap<string::String>, // Genres
    // vec_map::VecMap<string::String>, // Characters
    // u32, // Words
    // u32, // Favorites
    // u32, // Follows
    // chrono::DateTime<chrono::offset::Utc>, // Published
    // chrono::DateTime<chrono::offset::Utc>, // Updated
) {
    let selector_title = scraper::Selector::parse("title").unwrap();

    let title = string::String::from("");

    for element in document.select(&selector_title) {
        info!("{:?}", element.text());
    }

    return (
        title,
    );
}
