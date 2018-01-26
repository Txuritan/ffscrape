extern crate chrono;
#[macro_use] extern crate clap;
#[macro_use] extern crate error_chain;
extern crate fern;
#[macro_use] extern crate log;
extern crate regex;
extern crate reqwest;
extern crate scraper;
extern crate select;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate vec_map;

use clap::{Arg, App};

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

mod sites;

fn run() -> Result<()> {

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
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .apply().expect("Error: Fern unapble to create logger");

    let app = App::new("FFScraper")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Downloads stories from various sites")
        .arg(Arg::with_name("url")
             .short("u")
             .long("url")
             .help("Story url(s)")
             .takes_value(true)
             .value_name("URL")
             .multiple(true))
        .arg(Arg::with_name("disable")
             .short("d")
             .long("disable")
             .help("Disables Reqwest useragent"))
        .get_matches();

    let urls = app.values_of("url").map(|vals| vals.collect::<Vec<_>>());

    info!("{:?}", urls);

    Ok(())
}

quick_main!(run);
