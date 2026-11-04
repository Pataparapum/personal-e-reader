use std::fs;
use std::path::Path;

use epub::doc::EpubDoc;

use super::Book;

/// Recorre 'dir' (y subcarpetas) buscando .txt y .epub.
pub fn scan(dir: &Path) -> Vec<Book> {
    let mut books = Vec::new();
    visit(dir, &mut books);
    books
}

fn visit(dir: &Path, books: &mut Vec<Book>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            visit(&path, books);
        } else if let Some(book) = read_book(&path) {
            books.push(book);
        }
    }
}

fn read_book(path: &Path) -> Option<Book> {
    let ext = path.extension()?.to_str()?.to_lowercase();
    let stem = path.file_stem()?.to_string_lossy().to_string();

    match ext.as_str() {
        "txt" => Some(Book {
            title: stem,
            author: None,
            path: path.to_path_buf(),
        }),
        "epub" => match EpubDoc::new(path) {
            Ok(doc) => Some(Book {
                title: doc.mdata("title").map(|m| m.value.clone()).unwrap_or(stem),
                author: doc.mdata("creator").map(|m| m.value.clone()),
                path: path.to_path_buf(),
            }),
            Err(e) => {
                eprintln!("epub ilegible {}: {e}", path.display());
                None
            }
        },
        _ => None,
    }
}