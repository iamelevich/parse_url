extern crate clap;
use clap::{Arg, App, crate_version};
use url::{Url};
use std::error::Error;

fn main() {
    let matches = App::new("Comand Line URL Parser")
        .version(crate_version!())
        .author("Ilya Amelevich <ilya.amelevich@ya.ru>")
        .about("Parse URLs")
        .arg(Arg::with_name("URL")
            .help("Sets URL to parse")
            .required(true)
            .index(1))
        .get_matches();

    let url = matches.value_of("URL").unwrap();
    match get_queries_from_url(&url) {
        Ok(_) => ::std::process::exit(0),
        Err(err) => {
            eprintln!("Error: {:?}", err);
            ::std::process::exit(1);
        }
    }
}

fn get_queries_from_url(url: &str) -> Result<bool, Box<dyn Error>> {
    let parsed_url = Url::parse(url)?;
    let parsed_url = match parsed_url.query() {
        Some(val) => val.split('&').collect(),
        None => vec![],
    };
    for q in parsed_url.iter() {
        println!("{:?}", q);
    }
    Ok(true)
}
