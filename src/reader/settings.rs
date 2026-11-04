use std::fs;

use iced::Font;

use crate::config::{DATA_DIR, READER_VERTICAL_MARGIN, READER_VIEW_HEIGHT, SCREEN_WIDTH};
use crate::fonts;

struct FontDef {
    label: &'static str,
    key: &'static str,
    name: &'static str,
    bytes: &'static [u8],
}

const FONT_DEFS: [FontDef; 3] = [
    FontDef { label: "Serif", key: "serif", name: fonts::SERIF_NAME, bytes: fonts::SERIF },
    FontDef { label: "Sans", key: "sans", name: fonts::SANS_NAME, bytes: fonts::SANS },
    FontDef { label: "Monoespaciada", key: "mono", name: fonts::MONO_NAME, bytes: fonts::MONO },
];

/// (etiqueta, clave, interlineado relativo)
const SPACINGS: [(&str, &str, f32); 3] = [
    ("Compacto", "compact", 1.15),
    ("Normal", "normal", 1.35),
    ("Amplio", "wide", 1.6),
];

/// (etiqueta, clave, margen horizontal en px)
const MARGINS: [(&str, &str, f32); 3] = [
    ("Estrecho", "narrow", 8.0),
    ("Normal", "normal", 16.0),
    ("Ancho", "wide", 28.0),
];

const SIZES: [u8; 6] = [12, 14, 16, 18, 20, 22];

/// Cada ajuste editable del lector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Setting {
    Font,
    Size,
    Spacing,
    Margin,
    Justify,
}

impl Setting {
    pub const ALL: [Setting; 5] = [
        Setting::Font,
        Setting::Size,
        Setting::Spacing,
        Setting::Margin,
        Setting::Justify,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Setting::Font => "Fuente",
            Setting::Size => "Tamaño",
            Setting::Spacing => "Interlineado",
            Setting::Margin => "Márgenes",
            Setting::Justify => "Justificado",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Settings {
    font: usize,
    pub size: u8,
    spacing: usize,
    margin: usize,
    pub justify: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self { font: 0, size: 16, spacing: 1, margin: 1, justify: false }
    }
}

fn path() -> String {
    format!("{DATA_DIR}/reader.cfg")
}

impl Settings {
    // --- valores usados por el lector ---

    pub fn font(&self) -> Font {
        Font::with_name(FONT_DEFS[self.font].name)
    }

    pub fn font_bytes(&self) -> &'static [u8] {
        FONT_DEFS[self.font].bytes
    }

    pub fn line_height(&self) -> f32 {
        SPACINGS[self.spacing].2
    }

    pub fn margin(&self) -> f32 {
        MARGINS[self.margin].2
    }

    pub fn text_width(&self) -> f32 {
        SCREEN_WIDTH - 2.0 * self.margin()
    }

    /// Líneas que caben en una página con estos ajustes.
    pub fn rows(&self) -> usize {
        let usable = READER_VIEW_HEIGHT - 2.0 * READER_VERTICAL_MARGIN;
        let line = f32::from(self.size) * self.line_height();
        ((usable / line).floor() as usize).max(1)
    }

    // --- edición desde el menú ---

    /// Todas las opciones posibles de un ajuste, en orden.
    pub fn options(&self, setting: Setting) -> Vec<String> {
        match setting {
            Setting::Font => FONT_DEFS.iter().map(|d| d.label.to_string()).collect(),
            Setting::Size => SIZES.iter().map(|s| s.to_string()).collect(),
            Setting::Spacing => SPACINGS.iter().map(|d| d.0.to_string()).collect(),
            Setting::Margin => MARGINS.iter().map(|d| d.0.to_string()).collect(),
            Setting::Justify => vec!["No".to_string(), "Sí".to_string()],
        }
    }

    /// Índice, dentro de `options`, del valor actual.
    pub fn current(&self, setting: Setting) -> usize {
        match setting {
            Setting::Font => self.font,
            Setting::Size => SIZES.iter().position(|&s| s == self.size).unwrap_or(0),
            Setting::Spacing => self.spacing,
            Setting::Margin => self.margin,
            Setting::Justify => usize::from(self.justify),
        }
    }

    pub fn value_label(&self, setting: Setting) -> String {
        self.options(setting)
            .into_iter()
            .nth(self.current(setting))
            .unwrap_or_default()
    }

    /// Aplica la opción `index` (la posición dentro de `options`).
    pub fn apply(&mut self, setting: Setting, index: usize) {
        match setting {
            Setting::Font => self.font = index.min(FONT_DEFS.len() - 1),
            Setting::Size => self.size = SIZES[index.min(SIZES.len() - 1)],
            Setting::Spacing => self.spacing = index.min(SPACINGS.len() - 1),
            Setting::Margin => self.margin = index.min(MARGINS.len() - 1),
            Setting::Justify => self.justify = index == 1,
        }
    }

    // --- persistencia ---

    pub fn load() -> Self {
        let mut s = Self::default();
        let Ok(content) = fs::read_to_string(path()) else {
            return s;
        };

        for line in content.lines() {
            let Some((key, value)) = line.split_once('=') else {
                continue;
            };
            let value = value.trim();

            match key.trim() {
                "font" => {
                    if let Some(i) = FONT_DEFS.iter().position(|d| d.key == value) {
                        s.font = i;
                    }
                }
                "size" => {
                    if let Ok(n) = value.parse::<u8>() {
                        if SIZES.contains(&n) {
                            s.size = n;
                        }
                    }
                }
                "spacing" => {
                    if let Some(i) = SPACINGS.iter().position(|d| d.1 == value) {
                        s.spacing = i;
                    }
                }
                "margin" => {
                    if let Some(i) = MARGINS.iter().position(|d| d.1 == value) {
                        s.margin = i;
                    }
                }
                "justify" => s.justify = value == "true",
                _ => {}
            }
        }
        s
    }

    pub fn save(&self) {
        let _ = fs::create_dir_all(DATA_DIR);
        let content = format!(
            "font={}\nsize={}\nspacing={}\nmargin={}\njustify={}\n",
            FONT_DEFS[self.font].key,
            self.size,
            SPACINGS[self.spacing].1,
            MARGINS[self.margin].1,
            self.justify,
        );
        let _ = fs::write(path(), content);
    }
}