use iced::Font;

pub const SANS: &[u8] = include_bytes!("../assets/fonts/DejaVuSans.ttf");
pub const SERIF: &[u8] = include_bytes!("../assets/fonts/DejaVuSerif.ttf");
pub const MONO: &[u8] = include_bytes!("../assets/fonts/DejaVuSansMono.ttf");

pub const SANS_NAME: &str = "DejaVu Sans";
pub const SERIF_NAME: &str = "DejaVu Serif";
pub const MONO_NAME:  &str = "DejaVu Sans Mono";

/// Fuente de la interfaz (menús, barra de estado).
pub fn ui_font() -> Font {
    Font::with_name(SANS_NAME)
}