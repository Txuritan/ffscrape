use std::default;
use std::vec;
use std::string;

use chrono;
use reqwest;
use regex;
use scraper;
use vec_map;

///
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

impl FanFiction {
    pub fn new() -> FanFiction {
        FanFiction {
            ..Default::default()
        }
    }

    pub fn get(&self, url: &str, client: &reqwest::Client, document: &scraper::Html) -> FanFiction {
        FanFiction {
            title: self.get_title(&document),
            author: self.get_author(&document),
            fandoms: self.get_fandoms(&document),
            // self.get_rating(&document),
            ..Default::default()
        }
    }

    fn get_title(&self, document: &scraper::Html) -> string::String {
        let selector_title = scraper::Selector::parse("title").unwrap();

        // https://regex101.com/r/WjCcRF/1
        let regex_title = regex::Regex::new(r"(.*)( Chapter .*)(, .*)|(.*)(, .*)").unwrap();

        let mut title = string::String::from("");

        for element_title in document.select(&selector_title) {
            let title_inner = element_title.inner_html();
            if regex_title.is_match(&title_inner) {
                let titles = regex_title.captures(&title_inner).unwrap();
                title = titles[1].to_string().to_owned();
            }
        }

        title
    }

    fn get_author(&self, document: &scraper::Html) -> string::String {
        let selector_author = scraper::Selector::parse("#profile_top > a:first-of-type").unwrap();

        let mut author = string::String::from("");

        for element_author in document.select(&selector_author) {
            author = element_author.inner_html();
        }

        author
    }

    fn get_fandoms(&self, document: &scraper::Html) -> vec::Vec<string::String> {
        let selector_fandoms = scraper::Selector::parse(".lc-left > a:last-of-type").unwrap();

        let mut fandoms: vec::Vec<string::String> = vec::Vec::new();

        for element_fandoms in document.select(&selector_fandoms) {
            let html = element_fandoms.inner_html();
            let vec_fandoms: vec::Vec<&str> = html.split(" + ").collect();

            for fandom in vec_fandoms {
                fandoms.push(fandom.to_owned());
            }
        }

        fandoms
    }
}

impl default::Default for FanFiction {
    fn default() -> FanFiction {
        FanFiction {
            title: string::String::from(""),
            author: string::String::from(""),
            fandoms: vec::Vec::new(),
            rating: string::String::from(""),
            language: string::String::from(""),
            genres: vec::Vec::new(),
            characters: vec::Vec::new(),
            words: 0,
            favorites: 0,
            follows: 0,
            published: chrono::offset::Utc::now(),
            updated: chrono::offset::Utc::now(),
            chapters: vec_map::VecMap::new(),
        }
    }
}
