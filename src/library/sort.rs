use std::collections::BTreeMap;

use super::Book;

pub const OTHERS: &str = "Otros";

pub struct Group {
    pub name: String,
    pub books: Vec<Book>,
}

fn sort_by_title(books: &mut [Book]) {
    books.sort_by_key(|b| b.title.to_lowercase());
}

/// Agrupa por autor tal como viene en la metadata, ordena alfabéticamente.
/// Los libros sin autor va al final, en "Otros".
pub fn by_author(books: &[Book]) -> Vec<Group> {
    let mut map: BTreeMap<String, Vec<Book>> = BTreeMap::new();
    let mut others: Vec<Book> = Vec::new();

    for book in books {
        match book.author.as_deref().map(str::trim).filter(|a| !a.is_empty()) {
            Some(author) => map.entry(author.to_string()).or_default().push(book.clone()),
            None => others.push(book.clone()),
        }
    }

    let mut groups: Vec<Group> = map
        .into_iter()
        .map(|(name, mut books)| {
            sort_by_title(&mut books);
            Group { name, books }
        })
        .collect();

    if !others.is_empty() {
        sort_by_title(&mut others);
        groups.push(Group { name: OTHERS.to_string(), books: others });
    }

    groups
}

