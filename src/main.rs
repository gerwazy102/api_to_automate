mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

//add imports below
use api::doc_api::{create_doc, delete_doc, get_all_docs, get_doc, update_doc};
use api::health_api::get_health;
use models::doc_model::Doc;
use repository::crud_repo::CrudRepo;
use repository::database_state::DatabaseStateChecker;
use repository::mongodb_repo::MongoRepo;
use repository::sqlite_repo::SQLiteRepo;

use std::env;
extern crate dotenv;
use dotenv::dotenv;

enum DatabaseType {
    Memory,
    Mongo,
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let db_type = match env::var("DATABASE_TYPE") {
        Ok(v) => match v.to_uppercase().as_str() {
            "MEMORY" => DatabaseType::Memory,
            _ => DatabaseType::Mongo,
        },
        Err(_) => DatabaseType::Mongo,
    };

    let crud_repo: Option<Box<dyn CrudRepo<Doc>>>;
    let database_state_checker: Option<Box<dyn DatabaseStateChecker>>;
    match db_type {
        DatabaseType::Memory => {
            crud_repo = Some(Box::new(SQLiteRepo::init()) as Box<dyn CrudRepo<Doc>>);
            database_state_checker =
                Some(Box::new(SQLiteRepo::init()) as Box<dyn DatabaseStateChecker>);
        }
        DatabaseType::Mongo => {
            crud_repo = Some(Box::new(MongoRepo::init()) as Box<dyn CrudRepo<Doc>>);
            database_state_checker =
                Some(Box::new(MongoRepo::init()) as Box<dyn DatabaseStateChecker>);
        }
    };

    rocket::build()
        .manage(crud_repo.unwrap())
        .manage(database_state_checker.unwrap())
        .mount("/", routes![create_doc])
        .mount("/", routes![get_doc])
        .mount("/", routes![get_all_docs])
        .mount("/", routes![delete_doc])
        .mount("/", routes![update_doc])
        .mount("/", routes![get_health])
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use std::env;
    #[test]
    fn doc_create() {
        env::set_var("DATABASE_TYPE", "MEMORY");
        let client = Client::untracked(rocket()).expect("valid rocket instance");
        let resp = client
            .post("/doc")
            .body(r#"{"info": "test entry"}"#)
            .dispatch();
        assert!(resp.status() == Status::Ok, "status should be OK");
        assert!(
            resp.into_string().unwrap().contains(r#"{"id":"#),
            "should be equal"
        );
    }
}
