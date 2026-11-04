/// XHTML -> texto plano, un párrafo por línea.
pub fn to_text(html: &str) -> String {
    let mut out = String::with_capacity(html.len() / 2);
    let mut rest = html;
    let mut hidden = false; // dentro de head/style/script

    while let Some(start) = rest.find('<') {
        if !hidden {
            push_text(&mut out, &rest[..start]);
        }

        rest = &rest[start..];

        let Some(end) = rest.find('>') else {
            rest = "";
            break;
        };
        let tag = &rest[1..end];
        rest = &rest[end + 1..];

        let closing = tag.starts_with('/');
        let name = tag 
            .trim_start_matches('/')
            .split(|c: char| c.is_whitespace() || c == '/')
            .next()
            .unwrap_or("")
            .to_lowercase();

        match name.as_str() {
            "head" | "style" | "script" => hidden = !closing,
            "p" | "div" | "br" | "li" | "ul" | "ol" | "tr" | "table" | "hr" | "blockquote"
            | "section" | "article" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => out.push('\n'),
            _ => {}
        }   
    }
    if !hidden {
        push_text(&mut out, rest);
    }

    out.lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join("\n")
}

fn push_char(out: &mut String, c: char) {
    let c = if c.is_whitespace() { ' ' } else { c };
    if c == ' ' && out.ends_with(' ') {
        return;
    }
    out.push(c);
}

/// Decodifica entidades y colapsa espacios en blanco.
fn push_text(out: &mut String, s: &str) {
    let  mut rest = s;
    while let Some(i) = rest.find('&') {
        for c in rest[..i].chars() {
            push_char(out, c);
        }
        rest = &rest[i..];

        match rest.find(';') {
            Some(j) if j <= 10 => {
                match entity(&rest[1..j]) {
                    Some(c) => push_char(out, c),
                    None => {
                        for c in rest[..=j].chars() {
                            push_char(out, c);
                        }
                    }
                }
                rest = &rest[j + 1..];
            }
            _ => {
                push_char(out, '&');
                rest = &rest[1..];
            }
        }
    }
    for c in rest.chars() {
        push_char(out, c);
    }
}


fn entity(name: &str) -> Option<char> {
    match name {
        "amp" => Some('&'),
        "lt" => Some('<'),
        "gt" => Some('>'),
        "quot" => Some('"'),
        "apos" => Some('\''),
        "nbsp" => Some(' '),
        "mdash" => Some('—'),
        "ndash" => Some('–'),
        "hellip" => Some('…'),
        "laquo" => Some('«'),
        "raquo" => Some('»'),
        "ldquo" => Some('“'),
        "rdquo" => Some('”'),
        "lsquo" => Some('‘'),
        "rsquo" => Some('’'),
        _ => {
            let num = name.strip_prefix('#')?;
            let code = match num.strip_prefix(['x', 'X']) {
                Some(hex) => u32::from_str_radix(hex, 16).ok()?,
                None => num.parse().ok()?,
            };
            char::from_u32(code)
        }
    }
}
