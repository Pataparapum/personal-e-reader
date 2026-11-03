use iced::widget::{column, container, text};
use iced::{Element, Fill};

use super::style;

pub fn divider<'a, M: 'a>() -> Element<'a, M> {
    container(text("")).width(Fill).height(2.0).style(style::line).into()
}

/// Marco común: título arriba, contenido al centro, ayuda abajo.
pub fn frame<'a, M: 'a>(
    title: &'a str,
    body: impl Into<Element<'a, M>>,
    hint: &'a str,
) -> Element<'a, M> {
    container(column![
        container(text(title).size(22.0)).padding([12.0, 14.0]).width(Fill),
        divider(),
        container(body).width(Fill).height(Fill),
        divider(),
        container(text(hint).size(12.0)).padding([6.0, 14.0]).width(Fill),
    ])
    .width(Fill)
    .height(Fill)
    .style(style::paper)
    .into()
}