use iced::widget::container;
use iced::{Background, Border, Color, Theme};

pub fn paper(_theme: &Theme) -> container::Style {
    container::Style {
        text_color: Some(Color::BLACK),
        background: Some(Background::Color(Color::WHITE)),
        ..container::Style::default()
    }
}

pub fn line(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::BLACK)),
        ..container::Style::default()
    }
}

/// Seleccionado: invertido. No seleccionado: solo contorno.
pub fn item(selected: bool) -> impl Fn(&Theme) -> container::Style {
    move |_theme| {
        let (text, bg) = if selected{
            (Color::WHITE, Color::BLACK)
        } else {
            (Color::BLACK, Color::WHITE)
        };
        container::Style {
            text_color: Some(text),
            background: Some(Background::Color(bg)),
            border: Border {
                color: Color::BLACK,
                width: 2.0,
                ..Border::default()
            },
            ..container::Style::default()
        }
    }
}