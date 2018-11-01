# geoplaces

Query OpenStreetMap for geolocation information of any place.
This is based on [Nominatim](http://nominatim.openstreetmap.org/), the OpenStreetMap search engine.

**Requires Rust >=1.15**

## Install

First [install Rust](https://rustup.rs/).
Now install `geoplaces` using:

```
cargo install --git https://github.com/badboy/geoplaces
```

## Usage

```
$ geoplaces "New York"
NYC, New York, United States of America
  Latitude:  40.7305991
  Longitude: -73.9865811

New York, NYC, New York, United States of America
  Latitude:  40.7647714
  Longitude: -73.9807639

New York, Frederiksberggade, Kødbyen, Vesterbro, København, Københavns Kommune, Region Hovedstaden, 1459, Danmark
  Latitude:  55.677379
  Longitude: 12.571152

```

## License

MIT. See [LICENSE](LICENSE).
