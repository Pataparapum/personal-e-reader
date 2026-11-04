pub mod book;
pub mod scan;
pub mod sort;

pub use book::Book;

use std::path::PathBuf;

use crate::config::BOOKS_DIR;

fn book(title: &str, author: Option<&str>, file: &str) -> Book {
    Book {
        title: title.to_string(),
        author: author.map(str::to_string),
        path: PathBuf::from(BOOKS_DIR).join(file),
    }
}

