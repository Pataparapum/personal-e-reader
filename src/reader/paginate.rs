const INDENT: usize = 2;

/// Parte el texto en páginas de 'rows' líneas de hasta 'cols' caracteres.
/// Cada página es un String con saltos de línea.
pub fn paginate(text: &str, cols: usize, rows: usize) -> Vec<String> {
    let lines: Vec<String> = text.lines().flat_map(|p| wrap(p, cols)).collect();
    lines.chunks(rows).map(|chunk| chunk.join("\n")).collect()
}

fn wrap(paragraph: &str, cols: usize) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut line = " ".repeat(INDENT);
    let mut len = INDENT;
    let mut has_word = false;

    for word in paragraph.split_whitespace() {
        let mut chars: Vec<char> = word.chars().collect();

        loop {
            let sep = usize::from(has_word);
            if len + sep + chars.len() <= cols {
                if has_word {
                    line.push(' ');
                }
                line.extend(chars.iter());
                len += sep + chars.len();
                has_word = true;
                break;
            }

            if has_word {
                // No cabe: cierra la línea y reintenta en la siguiente.
                lines.push(std::mem::take(&mut line));
                len = 0;
                has_word = false;
            } else {
                // Palabra más larga que la línea: partirla.
                let take = cols - len;
                line.extend(chars.drain(..take));
                lines.push(std::mem::take(&mut line));
                len = 0;
            }
        }
    }

    if has_word {
        lines.push(line);
    }
    lines
}