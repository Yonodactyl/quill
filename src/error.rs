use thiserror::Error;

pub type Result<T> = std::result::Result<T, ScreenplayError>;

#[derive(Error, Debug)]
pub enum ScreenplayError {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Failed to parse JSON: {0}")]
    JsonParse(#[from] serde_json::Error),
    
    #[error("Invalid screenplay format: {0}")]
    InvalidFormat(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
}