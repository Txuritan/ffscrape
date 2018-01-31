use std::vec;
use std::string;

use chrono;
use regex;
use reqwest;
use scraper;
use vec_map;

pub fn get(document: scraper::Html, client: &reqwest::Client, url: &str) -> (
    string::String, // Title
    string::String, // Author
    vec::Vec<string::String>, // Fandoms
    string::String, // Rating
    // string::String, // Language
    // vec::Ve<string::String>, // Genres
    // vec::Ve<string::String>, // Characters
    // u32, // Words
    // u32, // Favorites
    // u32, // Follows
    // chrono::DateTime<chrono::offset::Utc>, // Published
    // chrono::DateTime<chrono::offset::Utc>, // Updated
) {
    return (
        get_title(&document),
        get_author(&document),
        get_fandoms(&document),
        get_rating(&document),
    );
}

fn get_title(document: &scraper::Html) -> string::String {
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

    return title;
}

fn get_author(document: &scraper::Html) -> string::String {
    let selector_author = scraper::Selector::parse("#profile_top > a:first-of-type").unwrap();

    let mut author = string::String::from("");

    for element_author in document.select(&selector_author) {
        author = element_author.inner_html();
    }

    return author;
}

fn get_fandoms(document: &scraper::Html) -> vec::Vec<string::String> {
    let selector_fandoms = scraper::Selector::parse(".lc-left > a:last-of-type").unwrap();

    let mut fandoms: vec::Vec<string::String> = vec::Vec::new();

    for element_fandoms in document.select(&selector_fandoms) {
        let html = element_fandoms.inner_html();
        let vec_fandoms: vec::Vec<&str> = html.split(" + ").collect();

        for fandom in vec_fandoms {
            fandoms.push(fandom.to_owned());
        }
    }

    return fandoms;
}

fn get_rating(document: &scraper::Html) -> string::String {
    let selector_rating = scraper::Selector::parse("#profile_top > .xgray.xcontrast_txt > a.xcontrast_txt:first-of-type").unwrap();

    let mut rating = string::String::from("");

    for element_rating in document.select(&selector_rating) {
        rating = element_rating.inner_html();
    }

    return rating;
}
