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

error_chain! {
    foreign_links {
        RegError(regex::Error);
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

mod sites;

fn download(matches: clap::ArgMatches, url: &str, site: &sites::Sites) -> Result<()> {

    let client = reqwest::Client::new();

    let mut res = client.get(url);

    if !matches.is_present("disable") {
        res.header(reqwest::header::UserAgent::new(format!("FFScrape/{:?} reqwest/{:?}", crate_version!(), "0.8.4")));
    }

    return Ok(())
}

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
        .apply().expect("Error: Fern unable to create logger");

    // https://regex101.com/r/tGp2wv/1/
    let regex_fanfiction_net = match regex::Regex::new(r"(http[s]?://)?(www|m)?[.]?fanfiction.net/s/(\d{7})(/)?(\d{1,4})?(/)?(.*)?") {
        Ok(r) => r,
        Err(e) => {
            return Err(format!("Could not compile Regex for FanFiction.Net: {}", e).into());
        }
    };

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
        .arg(clap::Arg::with_name("disable")
             .short("d")
             .long("disable")
             .help("Disables Reqwest useragent"))
        .get_matches();

    let urls = app.values_of("url").map(|vals| vals.collect::<Vec<_>>());

    info!("URLs: {:?}", &urls.clone().unwrap_or_else(Vec::new));

    for url in &urls.clone().unwrap_or_else(Vec::new) {
        info!("URL: {:?}", url);

        if regex_fanfiction_net.is_match(&url) {
            info!("URL: {:?}, matches Fanfiction.Net Regex", url);

            download(app, url, sites::Sites::FanFictionNet);
        }
    }

    return Ok(());
}

quick_main!(run);
