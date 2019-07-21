use std::env;
use std::process;

use serde::Deserialize;
use url::Url;

const API_URL: &'static str = "http://nominatim.openstreetmap.org/search/?format=json";

#[derive(Deserialize, Debug)]
struct Place {
    lat: String,
    lon: String,
    display_name: String,
}

type PlacesList = Vec<Place>;

fn query_url(query: &str) -> Url {
    let mut url = Url::parse(API_URL).unwrap();
    url.query_pairs_mut().append_pair("q", query);
    url
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

    let places: PlacesList = reqwest::get(query_url(&query))
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
