use iced::widget::{column, container, text, Column};
use iced::{Element, Fill};

use super::style;

pub struct MenuItem {
    pub label: &'static str,
    pub detail: &'static str,
}

pub fn menu<'a, M: 'a>(items: &[MenuItem], selected:usize) -> Element<'a, M> {
    let mut list: Column<'a, M> = Column::new().spacing(12.0).padding(12.0);

    for (i, item) in items.iter().enumerate() {
        let entry = container(
            column![text(item.label).size(30.0), text(item.detail).size(14.0)].spacing(4.0),
        )
        .padding(14.0)
        .width(Fill)
        .height(110.0)
        .style(style::item(i == selected));
        
        list = list.push(entry);
    }

        container(list).width(Fill).center_y(Fill).into()
}