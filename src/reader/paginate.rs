use super::metrics::Metrics;
use super::settings::Settings;

pub struct Page {
    /// Desplazamiento en bytes, dentro del texto del libro, de la primera línea.
    pub start: usize,
    pub blocks: Vec<String>,
}

struct Line {
    start: usize,
    text: String,
}

/// Sangría de 1 em al inicio de cada párrafo.
const INDENT: char = '\u{2003}';
/// Margen de seguridad: medimos un poco más estrecho de lo real
/// para que el render nunca saque más líneas de las previstas.
const WIDTH_SAFETY: f32 = 0.95;

pub fn paginate(text: &str, settings: &Settings) -> Vec<Page> {
    let metrics = Metrics::new(settings.font_bytes());
    let size = f32::from(settings.size);
    let max_w = settings.text_width() * WIDTH_SAFETY;
    let rows = settings.rows();

    let mut pages: Vec<Page> = Vec::new();
    let mut page = Page { start: 0, blocks: Vec::new() };
    let mut used = 0;

    for paragraph in text.lines() {
        if paragraph.trim().is_empty() {
            continue;
        }
        let offset = paragraph.as_ptr() as usize - text.as_ptr() as usize;

        for (i, line) in wrap(paragraph, offset, &metrics, size, max_w)
            .into_iter()
            .enumerate()
        {
            if used == rows {
                pages.push(std::mem::replace(
                    &mut page,
                    Page { start: 0, blocks: Vec::new() },
                ));
                used = 0;
            }
            if page.blocks.is_empty() {
                page.start = line.start;
            }

            if i == 0 {
                page.blocks.push(format!("{INDENT}{}", line.text));
            } else if used == 0 {
                // Continuación de un párrafo en una página nueva.
                page.blocks.push(line.text);
            } else if let Some(last) = page.blocks.last_mut() {
                last.push(' ');
                last.push_str(&line.text);
            }
            used += 1;
        }
    }

    if !page.blocks.is_empty() {
        pages.push(page);
    }
    if pages.is_empty() {
        pages.push(Page { start: 0, blocks: vec!["(libro vacío)".to_string()] });
    }
    pages
}

/// Índice de la página que contiene `offset`.
pub fn page_for_offset(pages: &[Page], offset: usize) -> usize {
    pages.iter().rposition(|p| p.start <= offset).unwrap_or(0)
}

/// Ajuste de línea voraz midiendo el ancho real de cada palabra.
fn wrap(par: &str, par_off: usize, m: &Metrics, size: f32, max_w: f32) -> Vec<Line> {
    let space = m.advance(' ', size);
    let mut lines = Vec::new();
    let mut text = String::new();
    let mut start = par_off;
    let mut width = size; // la primera línea lleva sangría

    for word in par.split_whitespace() {
        let off = par_off + (word.as_ptr() as usize - par.as_ptr() as usize);
        let w = m.width(word, size);

        if !text.is_empty() && width + space + w > max_w {
            lines.push(Line { start, text: std::mem::take(&mut text) });
            width = 0.0;
        }

        if text.is_empty() {
            start = off;
        } else {
            text.push(' ');
            width += space;
        }

        if width + w <= max_w {
            text.push_str(word);
            width += w;
            continue;
        }

        // Palabra más ancha que la línea: partirla por caracteres.
        for c in word.chars() {
            let cw = m.advance(c, size);
            if width + cw > max_w && !text.is_empty() {
                lines.push(Line { start, text: std::mem::take(&mut text) });
                width = 0.0;
            }
            text.push(c);
            width += cw;
        }
    }

    if !text.is_empty() {
        lines.push(Line { start, text });
    }
    lines
}