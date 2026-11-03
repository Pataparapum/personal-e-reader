use iced::event::{self, Event, Status};
use iced::keyboard::{self, Key};
use iced::{window, Subscription};

/// Acciones abstractas de los 3 botones (Back = pulsación larga de Select).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Up,
    Down,
    Select,
    Back,
    Lock,
}

/// Hoy: teclado. Mañana: GPIO de la Raspberry Pi.
pub fn subscription() -> Subscription<Input> {
    event::listen_with(map_event)
}

fn map_event(event: Event, _status: Status, _window: window::Id) -> Option<Input> {
    use keyboard::key::Named;

    match event {
        Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => match key {
            Key::Named(Named::ArrowUp) => Some(Input::Up),
            Key::Named(Named::ArrowDown) => Some(Input::Down),
            Key::Named(Named::Enter) => Some(Input::Select),
            Key::Named(Named::Escape) | Key::Named(Named::Backspace) => Some(Input::Back),
            Key::Character(c) if c.as_str() == "1" => Some(Input::Lock),
            _ => None,
        },
        _ => None,
    }
}