use crate::{
    models::{
        doc_model::Doc,
        response_models::{IDResponse, InfoResponse},
    },
    repository::crud_repo::CrudRepo,
};
use mongodb::bson::oid::ObjectId; //modify here
use rocket::{http::Status, serde::json::Json, State};

#[post("/doc", data = "<new_doc>")]
pub fn create_doc(
    docs_crud: &State<Box<dyn CrudRepo<Doc>>>,
    new_doc: Json<Doc>,
) -> Result<Json<IDResponse>, Status> {
    let data = Doc {
        id: None,
        info: new_doc.info.to_owned(),
    };
    let doc_detail = docs_crud.create(data);
    match doc_detail {
        Ok(doc) => Ok(Json(IDResponse::from_str(doc.as_str()))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/doc/<path>")]
pub fn get_doc(
    docs_crud: &State<Box<dyn CrudRepo<Doc>>>,
    path: String,
) -> Result<Json<Doc>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let doc_detail = docs_crud.get(&id);
    match doc_detail {
        Ok(doc) => Ok(Json(doc)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/doc")]
pub fn get_all_docs(docs_crud: &State<Box<dyn CrudRepo<Doc>>>) -> Result<Json<Vec<Doc>>, Status> {
    let docs = docs_crud.get_all();
    match docs {
        Ok(docs) => Ok(Json(docs)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/doc/<path>")]
pub fn delete_doc(
    docs_crud: &State<Box<dyn CrudRepo<Doc>>>,
    path: String,
) -> Result<Json<InfoResponse>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let result = docs_crud.delete(&id);
    match result {
        Ok(res) => {
            if res == 1 {
                return Ok(Json(InfoResponse::from_str("Doc successfully deleted!")));
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}

#[put("/doc/<path>", data = "<new_doc>")]
pub fn update_doc(
    docs_crud: &State<Box<dyn CrudRepo<Doc>>>,
    path: String,
    new_doc: Json<Doc>,
) -> Result<Json<IDResponse>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = Doc {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        info: new_doc.info.to_owned(),
    };
    let update_result = docs_crud.update(&id, data);
    match update_result {
        Ok(update) => match update {
            Some(id) => Ok(Json(IDResponse::from_str(id.as_str()))),
            None => Err(Status::NotFound),
        },
        Err(_) => Err(Status::InternalServerError),
    }
}
