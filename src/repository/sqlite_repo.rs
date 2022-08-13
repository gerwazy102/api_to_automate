use std::{str::FromStr, sync::Mutex};

use mongodb::bson::oid::ObjectId;
use sqlite::Connection;

use crate::models::{doc_model::Doc, states_models::DatabaseState};

use super::{
    crud_repo::{CrudError, CrudRepo},
    database_state::DatabaseStateChecker,
};

pub struct SQLiteRepo {
    db: Mutex<Connection>,
}

impl SQLiteRepo {
    pub fn init() -> Self {
        let db = sqlite::open(":memory:").unwrap();
        db.execute::<&str>("CREATE TABLE docs (id TEXT, info TEXT)")
            .expect("Error encountered while creating table");
        SQLiteRepo {
            db: Mutex::from(db),
        }
    }
}

impl DatabaseStateChecker for SQLiteRepo {
    fn get_state(&self) -> DatabaseState {
        let database_status = "UP(LOCAL TEST)";
        let col_name = String::from_str("docs").unwrap();
        let collections = Some(vec![col_name]);
        let documents_count = Some(100);

        DatabaseState {
            status: database_status.to_string(),
            documents_count: documents_count,
            collections: collections,
        }
    }
}

impl CrudRepo<Doc> for SQLiteRepo {
    fn create(&self, item: Doc) -> Result<String, CrudError> {
        let item = Doc {
            id: Some(ObjectId::new()),
            info: item.info,
        };
        let statement = format!(
            "INSERT INTO docs VALUES ('{}', '{}')",
            item.id.unwrap().to_string(),
            item.info
        );
        let id = item.id.clone().unwrap();
        let locked_db = self.db.lock().unwrap();
        locked_db.execute(statement).unwrap();
        Ok(id.to_string())
    }
    fn update(&self, _id: &String, _item: Doc) -> Result<Option<String>, CrudError> {
        // let obj_id = ObjectId::parse_str(id).unwrap();
        // for item_iter in self.docs {
        //     if item_iter.id.unwrap().to_string() == *id {
        //         item_iter.info = item.info;
        //         return Ok(Some(item_iter.id.unwrap().to_string()));
        //     }
        // }
        Err(CrudError::new(
            String::from_str("Not implemented yet!").unwrap(),
        ))
    }
    fn get(&self, id: &String) -> Result<Doc, CrudError> {
        let statement = format!("SELECT id, info FROM docs where id == '{}'", id);
        let locked_db = self.db.lock().unwrap();
        let mut docs: Vec<Doc> = Vec::new();
        locked_db
            .iterate(statement, |pairs| {
                let mut doc = Doc {
                    id: None,
                    info: format!(""),
                };
                for &(column, value) in pairs.iter() {
                    match column {
                        "id" => {
                            doc.id = Some(ObjectId::from_str(value.unwrap()).unwrap());
                        }
                        "info" => {
                            doc.info = format!("{}", value.unwrap());
                        }
                        _ => {}
                    }
                }
                docs.push(doc);
                true
            })
            .unwrap();
        if docs.len() == 1 {
            return Ok(docs[0].clone());
        }
        Err(CrudError::new(format!("Something went wrong")))
    }
    fn delete(&self, _id: &String) -> Result<u64, CrudError> {
        // let obj_id = ObjectId::parse_str(id).unwrap();
        // let deleted = false;
        // self.docs.retain(|doc| {
        //     if doc.id.unwrap() == obj_id {
        //         deleted = true;
        //         return false;
        //     }
        //     return true;
        // });
        // match deleted {
        //     true => Ok(1),
        //     false => Ok(0),
        // }
        Ok(0)
    }
    fn get_all(&self) -> Result<Vec<Doc>, CrudError> {
        let statement = format!("SELECT id, info FROM docs");
        let locked_db = self.db.lock().unwrap();
        let mut docs: Vec<Doc> = Vec::new();
        locked_db
            .iterate(statement, |pairs| {
                let mut doc = Doc {
                    id: None,
                    info: format!(""),
                };
                for &(column, value) in pairs.iter() {
                    match column {
                        "id" => {
                            doc.id = Some(ObjectId::from_str(value.unwrap()).unwrap());
                        }
                        "info" => {
                            doc.info = format!("{}", value.unwrap());
                        }
                        _ => {}
                    }
                }
                docs.push(doc);
                true
            })
            .unwrap();
        Ok(docs)
    }
}
