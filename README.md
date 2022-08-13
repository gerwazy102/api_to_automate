# Usage

## Local deployment

### Requirements

1. Rust
2. Mongo instance

### How to run

1. Setup mongodb - docker example:
```bash
docker run -d --name some-mongo -p 27017:27017\
        -e MONGO_INITDB_ROOT_USERNAME=mongoadmin \
        -e MONGO_INITDB_ROOT_PASSWORD=secret \
        mongo
```
2. Create `api_to_automate` database in mongo you just started

3. Export env variables:
```bash
export MONGODB_URI=mongodb://mongoadmin:secret@127.0.0.1
export MONGODB_DATABASE=api_to_automate
```
4. Start application with `cargo run`

## Unit testing

There are simple unit tests in this api written. Start them with `cargo test`

## SQLite mode

You can do:
```bash
export DATABASE_TYPE=MEMORY
```
before starting application which will start it in SQLite3 mode. This mode allows to run without any external database but data you save is available only until app restarts.

Any other value in `DATABASE_TYPE` variable will cause that app will start in mongo mode and will expect `MONGODB_URI` and `MONGODB_DATABASE` variables defined.


###Contributing

If you found any bugs please fell free to raise pull request! Contributions are welcome! Please be aware that by commiting code to this repository you agree to license it under MIT license!
