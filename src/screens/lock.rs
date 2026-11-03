use iced::widget::{column, container, text};
use iced::{Center, Element, Fill};

use crate::status::Status;
use crate::ui::{icons, style};

pub fn view<'a, M: 'a>(status: &Status) -> Element<'a, M> {
    container(
        column![
            icons::logo(130.0),
            text(status.time.clone()).size(44.0),
            text("Enter: desbloquear").size(13.0),
        ]
        .spacing(18.0)
        .align_x(Center),
    )
    .center(Fill)
    .style(style::paper)
    .into()
}