extern crate chrono;
#[macro_use]
extern crate clap;
extern crate fern;
#[macro_use]
extern crate log;
extern crate os_info;
extern crate regex;
extern crate reqwest;
extern crate scraper;
extern crate select;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate vec_map;

use std::fs;
use std::io::prelude::*;
use std::path;

mod sites;

fn write_story(story: &sites::Story) {
    let story_json = match serde_json::to_string(story) {
        Ok(j) => j,
        Err(e) => {
            return error!("Could not convert struct to JSON: {}", e);
        }
    };

    let regex_lint = regex::Regex::new(r"[/]?[:]?[\?]?").unwrap();

    let story_path_name = format!("{}-by-{}.json",
                                  regex_lint.replace_all(&story.title.to_lowercase().replace(" ", "-"), ""),
                                  regex_lint.replace_all(&story.author.to_lowercase().replace(" ", "-"), "")
    );

    let story_path = path::Path::new(&story_path_name);

    let display = story_path.display();

    let mut file = match fs::File::create(&story_path) {
        Ok(f) => f,
        Err(e) => {
            error!("Could not create {}: {}", display, e);
            return;
        }
    };

    match file.write_all(story_json.as_bytes()) {
        Ok(_) => {
            return;
        }
        Err(e) => {
            error!("Could not write to {}: {}", display, e);
            return;
        },
    };
}

fn setup_headers(matches: &clap::ArgMatches) -> reqwest::header::Headers {
    let mut headers = reqwest::header::Headers::new();

    if matches.is_present("enable") {
        headers.set(reqwest::header::UserAgent::new(format!("ffscrape/{} reqwest/{} hyper/{} rust/{}", crate_version!(), "0.8.4", "0.11.9", "1.25.0-nightly")));
    } else {
        match os_info::get().os_type() {
            &os_info::Type::Windows => {
                headers.set(reqwest::header::UserAgent::new("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:60.0) Gecko/20100101 Firefox/60.0"));
            }
            &os_info::Type::Macos => {
                headers.set(reqwest::header::UserAgent::new("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.12; rv:60.0) Gecko/20100101 Firefox/60.0"));
            }
            &os_info::Type::Linux => {
                headers.set(reqwest::header::UserAgent::new("Mozilla/5.0 (X11; Linux x86_64; rv:60.0) Gecko/20100101 Firefox/60.0"));
            }
            _ => {
                headers.set(reqwest::header::UserAgent::new("Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:60.0) Gecko/20100101 Firefox/60.0"));
            }
        }
    }

    return headers;
}

fn download(client: &reqwest::Client, matches: &clap::ArgMatches, url: &str, site: sites::Sites) {
    debug!("Connecting to {}", url);

    let mut res = match client.get(url)
        .headers(setup_headers(matches))
        .send() {
        Ok(r) => r,
        Err(e) => {
            return error!("Couldn't connect to {}", e);
        }
    };

    let body = res.text().unwrap();

    debug!("Parsing document from: {}", url);

    let document = scraper::Html::parse_document(&body);

    match site {
        sites::Sites::FanFictionNet => {
            let (title, author, fandoms, rating,) = sites::fanfiction::get(document, client, url);

            let story = sites::Story {
                title: title,
                author: author,
                fandoms: fandoms,
                rating: rating,
            };

            write_story(&story);
        }
    }
}

fn main() {
    let app = clap::App::new("FFScrape")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Downloads stories from various sites")
        .arg(clap::Arg::with_name("url")
            .short("u")
            .long("url")
            .help("Story url(s)")
            .takes_value(true)
            .value_name("URL")
            .multiple(true))
        .arg(clap::Arg::with_name("debug")
            .short("d")
            .long("debug")
            .help("Enables debug logging"))
        .arg(clap::Arg::with_name("enable")
            .short("e")
            .long("enable")
            .help("Enables Reqwest useragent"))
        .get_matches();

    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(if app.is_present("debug") { log::LevelFilter::Debug } else { log::LevelFilter::Info })
        .chain(std::io::stdout())
        .chain(fern::log_file("ffscrape.log").expect("Fern could not create log file"))
        .apply().expect("Fern could not create logger");

    // https://regex101.com/r/tGp2wv/1/
    let regex_fanfiction_net = regex::Regex::new(r"(http[s]?://)?(www|m)?[.]?fanfiction.net/s/(\d{7})(/)?(\d{1,4})?(/)?(.*)?").unwrap();

    let client = reqwest::Client::new();

    let urls = app.values_of("url").map(|vals| vals.collect::<Vec<_>>());

    debug!("URLs: {:?}", &urls.clone().unwrap_or_else(Vec::new));

    for url in &urls.clone().unwrap_or_else(Vec::new) {
        debug!("URL: {}", url);
        if regex_fanfiction_net.is_match(&url) {
            debug!("URL: {}, matches Fanfiction.Net Regex", url);

            download(&client, &app, &url, sites::Sites::FanFictionNet);
        }
    }
}
