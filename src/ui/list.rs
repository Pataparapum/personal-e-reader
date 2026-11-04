use iced::widget::{container, text, Column, Row};
use iced::{Center, Element, Fill};

use super::style;

pub const PAGE_SIZE: usize = 5;
const ROW_HEIGHT: f32 = 46.0;
const LABEL_MAX_CHARS: usize = 24;

pub struct ListRow {
    pub label: String,
    pub detail: Option<String>,
}

impl ListRow {
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), detail: None }
    }

    pub fn detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }
}

pub fn page_count(len: usize) -> usize {
    len.div_ceil(PAGE_SIZE).max(1)
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let mut t: String = s.chars().take(max - 1).collect();
        t.push('…');
        t
    }
}

/// Muestra solo la página que contiene 'selected'.
pub fn paged_list<'a, M: 'a>(rows: &[ListRow], selected: usize) -> Element<'a, M> {
    let start = selected / PAGE_SIZE * PAGE_SIZE;
    let mut list: Column<'a, M> = Column::new().spacing(6.0).padding(8.0);

    for (i, row) in rows.iter().enumerate().skip(start).take(PAGE_SIZE) {
        let mut line: Row<'a, M> = Row::new().spacing(8.0).align_y(Center);
        line = line.push(text(truncate(&row.label, LABEL_MAX_CHARS)).size(16.0).width(Fill));
        if let Some(detail) = &row.detail {
            line = line.push(text(detail.clone()).size(13.0));
        }

        list = list.push(
            container(line)
            .padding([0.0, 12.0])
            .width(Fill)
            .center_y(ROW_HEIGHT)
            .style(style::item(i == selected)),
        );
    }

    list.into()
}