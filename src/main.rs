use std::env;
use std::process;

use serde::Deserialize;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};
use isahc::prelude::*;

const API_URL: &'static str = "https://nominatim.openstreetmap.org/search/?format=json";

#[derive(Deserialize, Debug)]
struct Place {
    lat: String,
    lon: String,
    display_name: String,
}

type PlacesList = Vec<Place>;

fn query_url(query: &str) -> String {
    format!("{}&q={}", API_URL, utf8_percent_encode(query, NON_ALPHANUMERIC))
}

#[derive(PartialEq)]
enum Format {
    Xflux,
    Readable,
}

fn main() {
    let mut args = env::args().skip(1).peekable();
    let format = match args.peek() {
        Some(arg) if arg == "-x" => {
            Format::Xflux
        },
        _ => Format::Readable,
    };

    if format == Format::Xflux {
        let _ = args.next();
    }

    let query = match args.next() {
        Some(arg) => arg,
        None => {
            println!("Usage: geoplaces [-x] <query>");
            process::exit(1);
        }
    };

    let places: PlacesList = isahc::get(query_url(&query))
        .expect("Can't query API")
        .json()
        .expect("Can't parse API data");

    for place in places {
        match format {
            Format::Xflux => {
                println!("xflux -l {} -g {}", place.lat, place.lon);
                break;

            }
            Format::Readable => {
                println!("{}", place.display_name);
                println!("  Latitude:  {}", place.lat);
                println!("  Longitude: {}", place.lon);
                println!();
            }
        }
    }
}
