use std::fs;
use std::path::Path;

use crate::config::DATA_DIR;

fn file() -> String {
    format!("{DATA_DIR}/progress.txt")
}

/// Posición guardada (desplazamiento en bytes dentro del texto del libro).
pub fn load_position(book: &Path) -> Option<usize> {
    let key = book.to_string_lossy().to_string();
    let content = fs::read_to_string(file()).ok()?;

    for line in content.lines() {
        if let Some((offset, path)) = line.split_once('\t') {
            if path == key {
                return offset.parse().ok();
            }
        }
    }
    None
}

pub fn save_position(book: &Path, offset: usize) {
    let _ = fs::create_dir_all(DATA_DIR);
    let key = book.to_string_lossy().to_string();

    let mut lines: Vec<String> = fs::read_to_string(file())
        .unwrap_or_default()
        .lines()
        .filter(|l| l.split_once('\t').is_some_and(|(_, p)| p != key))
        .map(str::to_string)
        .collect();

    lines.push(format!("{offset}\t{key}"));
    let _ = fs::write(file(), lines.join("\n") + "\n");
}