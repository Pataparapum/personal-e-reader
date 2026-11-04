use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Book {
    pub title: String,
    pub author: Option<String>,
    pub path: PathBuf,
}

