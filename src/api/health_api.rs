use std::str::FromStr;

use crate::{
    models::states_models::{APIState, ApplicationState},
    repository::database_state::DatabaseStateChecker,
};
use rocket::{http::Status, serde::json::Json, State};

#[get("/health")]
pub fn get_health(
    db: &State<Box<dyn DatabaseStateChecker>>,
) -> Result<Json<ApplicationState>, Status> {
    Ok(Json(ApplicationState {
        api_state: APIState {
            status: String::from_str("UP").unwrap(),
        },
        database_state: db.get_state(),
    }))
}
