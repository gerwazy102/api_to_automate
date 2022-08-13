use std::{env, str::FromStr};
extern crate dotenv;
use dotenv::dotenv;

use crate::models::{doc_model::Doc, states_models::DatabaseState};

use mongodb::{
    bson::{doc, oid::ObjectId}, //modify here
    sync::{Client, Collection, Database},
};

use super::{
    crud_repo::{CrudError, CrudRepo},
    database_state::DatabaseStateChecker,
};

pub struct MongoRepo {
    docs: Collection<Doc>,
    db: Database,
}

impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = match env::var("MONGODB_URI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Database uri not set via MONGODB_URI env variable"),
        };

        let db_name = match env::var("MONGODB_DATABASE") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Database name not set via MONGODB_DATABASE env variable"),
        };

        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database(db_name.as_str());
        let docs: Collection<Doc> = db.collection("docs");

        MongoRepo { docs, db }
    }
}

impl DatabaseStateChecker for MongoRepo {
    fn get_state(&self) -> DatabaseState {
        let mut database_status = "DOWN";
        let collection_names = self.db.list_collection_names(None);
        if collection_names.is_ok() {
            database_status = "UP";
        }
        let collections_optional = collection_names.ok();
        let documents_count = self.docs.count_documents(None, None).ok();

        DatabaseState {
            status: database_status.to_string(),
            documents_count: documents_count,
            collections: collections_optional,
        }
    }
}

impl CrudRepo<Doc> for MongoRepo {
    fn create(&self, item: Doc) -> Result<String, CrudError> {
        let item = Doc {
            id: None,
            info: item.info,
        };
        match self.docs.insert_one(item, None) {
            Ok(result) => {
                print!("{:?}", result.inserted_id);
                match result.inserted_id.as_object_id() {
                    Some(inserted_id_obj) => Ok(inserted_id_obj.to_string()),
                    None => Err(CrudError::new(String::from_str("Insert failed").unwrap())),
                }
            }
            Err(e) => Err(CrudError::new(e.to_string())),
        }
    }
    fn update(&self, id: &String, item: Doc) -> Result<Option<String>, CrudError> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "id": item.id,
                    "info": item.info,
                },
        };
        match self.docs.update_one(filter, new_doc, None) {
            Ok(result) => match result.matched_count {
                1 => Ok(Some(id.to_string())),
                _ => Ok(None),
            },
            Err(e) => Err(CrudError::new(e.to_string())),
        }
    }
    fn get(&self, id: &String) -> Result<Doc, CrudError> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        match self.docs.find_one(filter, None) {
            Ok(doc_details) => Ok(doc_details.unwrap()),
            Err(e) => Err(CrudError::new(e.to_string())),
        }
    }
    fn delete(&self, id: &String) -> Result<u64, CrudError> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        match self.docs.delete_one(filter, None) {
            Ok(result) => Ok(result.deleted_count),
            Err(e) => Err(CrudError::new(e.to_string())),
        }
    }
    fn get_all(&self) -> Result<Vec<Doc>, CrudError> {
        let cursors = self
            .docs
            .find(None, None)
            .ok()
            .expect("Error getting list of docs");
        let docs = cursors.map(|doc| doc.unwrap()).collect();
        Ok(docs)
    }
}
