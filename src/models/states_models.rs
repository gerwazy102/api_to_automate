use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseState {
    pub documents_count: Option<u64>,
    pub status: String,
    pub collections: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct APIState {
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationState {
    pub api_state: APIState,
    pub database_state: DatabaseState,
}
