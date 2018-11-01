#[macro_use]
extern crate serde_derive;
extern crate url;
extern crate reqwest;

use std::env;
use std::process;
use url::Url;

const API_URL : &'static str = "http://nominatim.openstreetmap.org/search/?format=json";

#[derive(Serialize, Deserialize,Debug)]
struct Place {
    place_id: String,
    osm_type: String,
    osm_id: String,
    lat: String,
    lon: String,
    display_name: String,
    class: String,
    #[serde(rename = "type")]
    typ: String,
}

type PlacesList = Vec<Place>;

fn query_url(query: &str) -> Url {
    let mut url = Url::parse(API_URL).unwrap();
    url.query_pairs_mut().append_pair("q", query);
    url
}

fn main() {
    let query = match env::args().skip(1).next() {
        Some(arg) => arg,
        None => {
            println!("Usage: geoplaces <query>");
            process::exit(1);
        }
    };

    let client = reqwest::Client::new().expect("Can't create client");
    let mut res = client.get(query_url(&query)).send().expect("Can't query API");
    let places : PlacesList = res.json().expect("Can't parse API data");

    for place in places {
        println!("{}", place.display_name);
        println!("  Latitude:  {}", place.lat);
        println!("  Longitude: {}", place.lon);
        println!();
    }
}
