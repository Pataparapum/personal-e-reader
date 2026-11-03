use iced::widget::{container, text};
use iced::{Element, Fill};

use super::{Action, Route};
use crate::input::Input;
use crate::ui::frame::frame;

pub struct Placeholder {
    title: &'static str,
}

impl Placeholder {
    pub fn new(title: &'static str) -> Self {
        Self { title }
    }

    pub fn handle(&mut self, input: Input) -> Option<Action> {
        match input {
            Input::Back => Some(Action::Navigate(Route::Home)),
            _=> None,
        }
    }

    pub fn view<'a, M: 'a>(&self) -> Element<'a, M> {
        frame(
            self.title,
            container(text("Proximamente").size(20.0)).center(Fill),
            "Esc: volver",
        )
    }
}