use iced::widget::{container, text};
use iced::{Element, Fill};

use super::{Action, Route};
use crate::input::Input;
use crate::library::{
    self,
    sort::{self, Group},
};

use crate::ui::frame::frame;
use crate::ui::list::{page_count, paged_list, ListRow};

enum Level {
    Authors,
    Books(usize),
}

pub struct LibraryScreen {
    groups: Vec<Group>,
    level: Level,
    selected: usize,
    /// Para volver a la misma fila de la lista de autores.
    author_cursor: usize,
}

impl LibraryScreen {
    pub fn new() -> Self {
        Self {
            groups: sort::by_author(&library::mock()),
            level: Level::Authors,
            selected: 0,
            author_cursor: 0,
        }
    }

    fn len(&self) -> usize {
        match self.level {
            Level::Authors => self.groups.len(),
            Level::Books(g) => self.groups[g].books.len(),
        }
    }

    pub fn handle(&mut self, input: Input) -> Option<Action> {
        let n = self.len();

        match input {
            Input::Up if n > 0 => {
                self.selected = (self.selected + n - 1) % n;
                None
            }
            Input::Down if n > 0 => {
                self.selected = (self.selected + 1) % n;
                None
            }
            Input::Select => {
                if let Level::Authors = self.level {
                    if n > 0 {
                        self.author_cursor = self.selected;
                        self.level = Level::Books(self.selected);
                        self.selected = 0;
                    }
                }
                // Nivel Books: aquí se abrirá el lector.
                None
            }
            Input::Back => match self.level {
                Level::Authors => Some(Action::Navigate(Route::Home)),
                Level::Books(_) => {
                    self.level = Level::Authors;
                    self.selected = self.author_cursor;
                    None
                }
            },
            _ => None,
        }
        
    }

    pub fn view<'a, M: 'a>(&self) -> Element<'a, M> {
        let (title, rows): (String, Vec<ListRow>) = match self.level {
            Level::Authors => (
                "Libros".to_string(),
                self.groups
                    .iter()
                    .map(|g| ListRow::new(g.name.clone()).detail(count(g.books.len())))
                    .collect(),
            ),
            Level::Books(g) => {
                let group = &self.groups[g];
                (
                    group.name.clone(),
                    group.books.iter().map(|b| ListRow::new(b.title.clone())).collect(),
                )
            }
        };

        let body: Element<'a, M> = if rows.is_empty() {
            container(text("No hay libros").size(18.0)).center(Fill).into()
        } else {
            paged_list(&rows, self.selected)
        };

        let page = self.selected / crate::ui::list::PAGE_SIZE + 1;
        let hint = format!("Pág {}/{}   Esc: volver", page, page_count(rows.len()));

        frame(title, body, hint)
    }
}

fn count(n: usize) -> String {
    if n == 1 { "1 libro".to_string() } else { format!("{n} libros") }
}