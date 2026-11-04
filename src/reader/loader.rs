use std::fs;
use std::path::Path;

use epub::doc::EpubDoc;

use super::{encoding, html};

/// Devuelve el libro como texto plano, un párrafo por linea.
pub fn load(path: &Path) -> Result<String, String> {
    match path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_lowercase)
        .as_deref()
    {
        Some("txt") => load_txt(path),
        Some("epub") => load_epub(path),
        _ => Err("Formato no soportado".to_string()),
    }
}

fn load_txt(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|e| e.to_string())?;
    Ok(normalize_txt(&encoding::decode(&bytes)))
}

/// Si el archivo usa lineas en blanco entre párrafos, une las líneas de cada
/// párrafo (arrega txt con saltos duros). Si no, cada línea es un párrafo
fn normalize_txt(s: &str) -> String {
    let has_blank = s.lines().any(|l| l.trim().is_empty());
    if !has_blank {
        return s.lines().map(str::trim).collect::<Vec<_>>().join("\n");
    }

    let mut out = String::new();
    let mut para = String::new();
    for line in s.lines() {
        if line.trim().is_empty() {
            if !para.trim().is_empty() {
                out.push_str(&para);
                out.push('\n');
                para.clear();
            }
        } else {
            if !para.is_empty() {
                para.push(' ');
            }
            para.push_str(line.trim());
        }
    }
    if !para.is_empty() {
        out.push_str(&para);
        out.push('\n');
    }
    out
}

fn load_epub(path: &Path) -> Result<String, String> {
    let mut doc = EpubDoc::new(path).map_err(|e| e.to_string())?;
    let mut text = String::new();

    loop {
        if let Some((content, _mime)) = doc.get_current_str() {
            text.push_str(&html::to_text(&content));
            text.push('\n');
        }
        if !doc.go_next() {
            break ;
        }
    }

    Ok(text)
}