use crate::models::states_models::DatabaseState;

pub trait DatabaseStateChecker: Send + Sync {
    fn get_state(&self) -> DatabaseState;
}
