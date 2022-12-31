# pws-collector
- polls a list of weather.com stations for data on interval, and stores in MongoDB 6 timeseries collection with geospatial index on location
## Requirements
* [Rust](https://www.rust-lang.org/tools/install)
* [MongoDB v6](https://www.mongodb.com/docs/manual/administration/install-enterprise) (local test instance)
* [MongoDB Atlas](https://www.mongodb.com/basics/mongodb-atlas-tutorial) (optional, production)
## Nice to have 
* [MongoDB Compass](https://www.mongodb.com/products/compass)

## Development
* provide a `.env` file based on `.env.example`
* run tests:
```sh
cargo test
```
## Production
* provide a `.env` file based on `.env.example`
* add a config/production.json file based with with desired settings for production (see config/development.json for example)
```sh
RUN_MODE=production cargo run
```
## Acknowledgments
* [rustapi](https://github.com/ndelvalle/rustapi)
* [geodata-rest](https://github.com/dclimate/geodata-rest)