use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("database error: {0}")]
    DatabaseError(String),
}
