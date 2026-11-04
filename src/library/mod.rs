pub mod book;
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

/// Datos de prueba. Después: escanear BOOKS_DIR y lerr al metadata.
pub fn mock() -> Vec<Book> {
    vec![
        book("La casa de los espíritus", Some("Allende, Isabel"), "casa.epub"),
        book("Ficciones", Some("Borges, Jorge Luis"), "ficciones.epub"),
        book("El Aleph", Some("Borges, Jorge Luis"), "aleph.epub"),
        book("2666", Some("Bolaño, Roberto"), "2666.epub"),
        book("Los detectives salvajes", Some("Bolaño, Roberto"), "detectives.epub"),
        book("Don Quijote de la Mancha", Some("Cervantes, Miguel de"), "quijote.epub"),
        book("Rayuela", Some("Cortázar, Julio"), "rayuela.epub"),
        book("Cien años de soledad", Some("García Márquez, Gabriel"), "cien.epub"),
        book("Pedro Páramo", Some("Rulfo, Juan"), "pedro.epub"),
        book("Apuntes de Rust", None, "apuntes.pdf"),
        book("Borrador sin título", None, "borrador.epub"),
    ]
}