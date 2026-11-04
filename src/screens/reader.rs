use std::path::Path;

use iced::widget::text::Wrapping;
use iced::widget::{column, container, text};
use iced::{Element, Fill, Font};

use super::Action;
use crate::config::{READER_COLS, READER_FONT_SIZE, READER_ROWS};
use crate::input::Input;
use crate::reader::{loader, paginate::paginate};
use crate::ui::frame::divider;
use crate::ui::list::truncate;

pub struct Reader {
    title: String,
    pages: Vec<String>,
    current: usize,
}

impl Reader {
    pub fn open(path: &Path) -> Self {
        let title = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        let text = loader::load(path)
            .unwrap_or_else(|e| format!("No se pudo abrir el libro.\n{e}"));

        let mut pages = paginate(&text, READER_COLS, READER_ROWS);
        if pages.is_empty() {
            pages.push("(libro vacío)".to_string());
        }

        Self { title, pages, current: 0 }
    }

    pub fn handle(&mut self, input: Input) -> Option<Action> {
        match input {
            Input::Down => {
                if self.current + 1 < self.pages.len() {
                    self.current += 1;
                }
                None
            }
            Input::Up => {
                self.current = self.current.saturating_sub(1);
                None
            }
            Input::Back => Some(Action::CloseBook),
            _ => None,
        }
    }

    pub fn view<'a, M: 'a>(&self) -> Element<'a, M> {
        let page = text(self.pages[self.current].clone())
            .size(READER_FONT_SIZE)
            .font(Font::MONOSPACE)
            .wrapping(Wrapping::None);

        let footer = format!(
            "{}/{}  {}",
            self.current + 1,
            self.pages.len(),
            truncate(&self.title, 20)
        );

        column![
            container(page).padding(12.0).width(Fill).height(Fill),
            divider(),
            container(text(footer).size(12.0)).padding([6.0, 14.0]).width(Fill),
        ]
        .width(Fill)
        .height(Fill)
        .into()
    }
}

