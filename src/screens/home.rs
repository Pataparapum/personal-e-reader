use iced::Element;

use super::{Action, Route};
use crate::input::Input;
use crate::ui::frame::frame;
use crate::ui::menu::{menu, MenuItem};

const ITEMS: [MenuItem; 2] = [
    MenuItem { label: "Libros", detail: "Leer" },
    MenuItem { label: "AudioLibros", detail: "Escuchar" },
];

const ROUTES: [Route; 2] = [Route::Library, Route::Audiobooks];

#[derive(Default)]
pub struct Home {
    selected:usize,
}

impl Home {
    pub fn handle(&mut self, input: Input) -> Option<Action> {
        let n = ITEMS.len();
        match input {
            Input::Up => {
                self.selected = (self.selected + n -1) % n;
                None
            }
            Input::Down => {
                self.selected = (self.selected + 1) % n;
                None
            }
            Input::Select => Some(Action::Navigate(ROUTES[self.selected])),
            _=> None,
        }
    }

    pub fn view<'a, M: 'a>(&self) -> Element<'a, M> {
        frame(
            "E-Reader",
            menu(&ITEMS, self.selected),
            "Arriba/Abajo: mover    Enter: abrir",
        )
    }
}