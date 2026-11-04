use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use iced::widget::text::{Alignment, Wrapping};
use iced::widget::{column, container, text, Column};
use iced::{Element, Fill};

use super::Action;
use crate::config::{DOUBLE_BACK_MS, READER_VERTICAL_MARGIN};
use crate::input::Input;
use crate::reader::loader;
use crate::reader::paginate::{page_for_offset, paginate, Page};
use crate::reader::settings::{Setting, Settings};
use crate::storage;
use crate::ui::frame::{divider, frame};
use crate::ui::list::{paged_list_with, truncate, ListRow};

/// Una fila por ajuste.
const MENU_ROWS: usize = Setting::ALL.len();
const LIST_PAGE: usize = 6;
const LIST_ROW_HEIGHT: f32 = 36.0;

#[derive(Clone, Copy)]
enum Mode {
    Reading,
    Menu {
        selected: usize,
    },
    Edit {
        setting: Setting,
        /// Opción resaltada (aún sin guardar).
        selected: usize,
        /// Fila del menú a la que se vuelve.
        row: usize,
    },
}

pub struct Reader {
    path: PathBuf,
    title: String,
    text: String,
    settings: Settings,
    pages: Vec<Page>,
    current: usize,
    mode: Mode,
    /// Momento del último Esc, para detectar la doble pulsación.
    last_back: Option<Instant>,
}

impl Reader {
    pub fn open(path: &Path) -> Self {
        let title = path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();

        let text = loader::load(path)
            .unwrap_or_else(|e| format!("No se pudo abrir el libro.\n{e}"));

        let settings = Settings::load();
        let pages = paginate(&text, &settings);
        let saved = storage::load_position(path).unwrap_or(0);
        let current = page_for_offset(&pages, saved);

        Self {
            path: path.to_path_buf(),
            title,
            text,
            settings,
            pages,
            current,
            mode: Mode::Reading,
            last_back: None,
        }
    }

    fn goto(&mut self, page: usize) {
        self.current = page;
        storage::save_position(&self.path, self.pages[page].start);
    }

    /// Recalcula las páginas conservando el punto de lectura.
    fn repaginate(&mut self) {
        let offset = self.pages[self.current].start;
        self.pages = paginate(&self.text, &self.settings);
        self.current = page_for_offset(&self.pages, offset);
    }

    /// Registra un Esc y dice si es el segundo, rápido, de una doble pulsación.
    fn double_back(&mut self) -> bool {
        let now = Instant::now();
        let window = Duration::from_millis(DOUBLE_BACK_MS);
        let is_double = self.last_back.is_some_and(|t| now.duration_since(t) <= window);

        self.last_back = if is_double { None } else { Some(now) };
        is_double
    }

    pub fn handle(&mut self, input: Input) -> Option<Action> {
        // Cualquier otra tecla entre dos Esc rompe la doble pulsación.
        if input != Input::Back {
            self.last_back = None;
        }

        match self.mode {
            Mode::Reading => self.handle_reading(input),
            Mode::Menu { selected } => self.handle_menu(selected, input),
            Mode::Edit { setting, selected, row } => {
                self.handle_edit(setting, selected, row, input)
            }
        }
    }

    fn handle_reading(&mut self, input: Input) -> Option<Action> {
        match input {
            Input::Down if self.current + 1 < self.pages.len() => self.goto(self.current + 1),
            Input::Up if self.current > 0 => self.goto(self.current - 1),
            Input::Back => {
                if self.double_back() {
                    return Some(Action::CloseBook);
                }
                self.mode = Mode::Menu { selected: 0 };
            }
            _ => {}
        }
        None
    }

    fn handle_menu(&mut self, selected: usize, input: Input) -> Option<Action> {
        match input {
            Input::Up => {
                self.mode = Mode::Menu { selected: (selected + MENU_ROWS - 1) % MENU_ROWS }
            }
            Input::Down => self.mode = Mode::Menu { selected: (selected + 1) % MENU_ROWS },
            Input::Back => {
                if self.double_back() {
                    return Some(Action::CloseBook);
                }
                self.mode = Mode::Reading;
            }
            Input::Select => {
                let setting = Setting::ALL[selected];
                self.mode = Mode::Edit {
                    setting,
                    selected: self.settings.current(setting),
                    row: selected,
                };
            }
            _ => {}
        }
        None
    }

    fn handle_edit(
        &mut self,
        setting: Setting,
        selected: usize,
        row: usize,
        input: Input,
    ) -> Option<Action> {
        let n = self.settings.options(setting).len();

        match input {
            Input::Up => {
                self.mode = Mode::Edit { setting, selected: (selected + n - 1) % n, row }
            }
            Input::Down => self.mode = Mode::Edit { setting, selected: (selected + 1) % n, row },
            // Cancelar: vuelve al menú sin tocar nada (no cuenta para el doble Esc).
            Input::Back => self.mode = Mode::Menu { selected: row },
            Input::Select => {
                self.settings.apply(setting, selected);
                self.settings.save();
                if setting != Setting::Justify {
                    self.repaginate();
                }
                self.mode = Mode::Menu { selected: row };
            }
            _ => {}
        }
        None
    }

    pub fn view<'a, M: 'a>(&self) -> Element<'a, M> {
        match self.mode {
            Mode::Reading => self.page_view(),
            Mode::Menu { selected } => self.menu_view(selected),
            Mode::Edit { setting, selected, .. } => self.edit_view(setting, selected),
        }
    }

    fn page_view<'a, M: 'a>(&self) -> Element<'a, M> {
        let s = &self.settings;
        let align = if s.justify { Alignment::Justified } else { Alignment::Left };

        let mut body: Column<'a, M> = Column::new()
            .padding([READER_VERTICAL_MARGIN, s.margin()])
            .width(Fill);

        for block in &self.pages[self.current].blocks {
            body = body.push(
                text(block.clone())
                    .size(f32::from(s.size))
                    .font(s.font())
                    .line_height(s.line_height())
                    .width(Fill)
                    .align_x(align)
                    .wrapping(Wrapping::Word),
            );
        }

        let footer = format!(
            "{}/{}   {}",
            self.current + 1,
            self.pages.len(),
            truncate(&self.title, 20)
        );

        column![
            container(body).width(Fill).height(Fill),
            divider(),
            container(text(footer).size(12.0)).padding([6.0, 14.0]).width(Fill),
        ]
        .width(Fill)
        .height(Fill)
        .into()
    }

    fn menu_view<'a, M: 'a>(&self, selected: usize) -> Element<'a, M> {
        let rows: Vec<ListRow> = Setting::ALL
            .iter()
            .map(|&setting| {
                ListRow::new(setting.label()).detail(self.settings.value_label(setting))
            })
            .collect();

        frame(
            "Ajustes de texto",
            paged_list_with(&rows, selected, LIST_PAGE, LIST_ROW_HEIGHT),
            "Esc: volver   Esc x2: Libros",
        )
    }

    fn edit_view<'a, M: 'a>(&self, setting: Setting, selected: usize) -> Element<'a, M> {
        let current = self.settings.current(setting);

        let rows: Vec<ListRow> = self
            .settings
            .options(setting)
            .into_iter()
            .enumerate()
            .map(|(i, label)| {
                let row = ListRow::new(label);
                if i == current { row.detail("actual") } else { row }
            })
            .collect();

        frame(
            setting.label(),
            paged_list_with(&rows, selected, LIST_PAGE, LIST_ROW_HEIGHT),
            "Enter: guardar   Esc: cancelar",
        )
    }
}