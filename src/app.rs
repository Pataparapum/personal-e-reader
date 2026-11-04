use iced::widget::{column, container};
use iced::{Element, Fill, Subscription, Theme};

use crate::input::{self, Input};
use crate::status::Status;
use crate::ui::frame::divider;
use crate::ui::status_bar::status_bar;
use crate::ui::style;
use crate::screens::{
    home::Home, library::LibraryScreen, lock, placeholder::Placeholder, Action, Route,
};

#[derive(Debug, Clone)]
pub enum Message {
    Input(Input),
}

enum Screen {
    Home(Home),
    Library(LibraryScreen),
    Audiobooks(Placeholder),
}

impl From<Route> for Screen {
    fn from(route: Route) -> Self {
        match route {
            Route::Home => Screen::Home(Home::default()),
                        Route::Library => Screen::Library(LibraryScreen::new()),
            Route::Audiobooks => Screen::Audiobooks(Placeholder::new("Audiolibros")),
        }
    }
}

pub struct App {
    screen: Screen,
    status: Status,
    locked: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Screen::from(Route::Home),
            status: Status::mock(),
            locked: true,
        }
    }
}

impl App {
    pub fn update(&mut self, message: Message) {
        let Message::Input(input) = message;

        if self.locked {
            if input == Input::Select {
                self.locked = false;
            }
            return;
        }

        if input == Input::Lock {
            self.locked = true;
            return;
        }

        let action = match &mut self.screen {
            Screen::Home(s) => s.handle(input),
            Screen::Library(s) => s.handle(input),
            Screen::Audiobooks(s) => s.handle(input),
        };

        if let Some(Action::Navigate(route)) = action {
            self.screen = Screen::from(route);
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        if self.locked {
            return lock::view(&self.status);
        }

                let screen: Element<'_, Message> = match &self.screen {
            Screen::Home(s) => s.view(),
            Screen::Library(s) => s.view(),
            Screen::Audiobooks(s) => s.view(),
        };

        container(column![status_bar(&self.status), divider(), screen])
            .width(Fill)
            .height(Fill)
            .style(style::paper)
            .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        input::subscription().map(Message::Input)
    }

    pub fn theme(&self) -> Theme {
        Theme::Light
    }
}