use std::fmt;

#[derive(Debug, Clone)]
pub struct CrudError {
    info: String,
}

impl CrudError {
    pub fn new(message: String) -> CrudError {
        CrudError { info: message }
    }
}

impl fmt::Display for CrudError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})", self.info)
    }
}

pub trait CrudRepo<T>: Send + Sync {
    fn create(&self, item: T) -> Result<String, CrudError>;
    fn update(&self, id: &String, item: T) -> Result<Option<String>, CrudError>;
    fn get(&self, id: &String) -> Result<T, CrudError>;
    fn delete(&self, id: &String) -> Result<u64, CrudError>;
    fn get_all(&self) -> Result<Vec<T>, CrudError>;
}
