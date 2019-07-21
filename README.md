# geoplaces

Query OpenStreetMap for geolocation information of any place.
This is based on [Nominatim](http://nominatim.openstreetmap.org/), the OpenStreetMap search engine.

**Requires Rust >=1.31**

## Install

First [install Rust](https://rustup.rs/).
Now install `geoplaces` using:

```
cargo install geoplaces
```

## Usage

```
$ geoplaces "New York"
NYC, New York, United States of America
  Latitude:  40.7127281
  Longitude: -74.0060152

New York, United States of America
  Latitude:  43.1561681
  Longitude: -75.8449946

New York, Tyne and Wear, North East England, England, NE29 8EP, UK
  Latitude:  55.0252998
  Longitude: -1.4869496
```

## License

MIT. See [LICENSE](LICENSE).
