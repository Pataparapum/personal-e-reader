use iced::widget::{container, row, text, Row};
use iced::{Center, Element, Fill};

use super::icons;
use crate::status::Status;

pub fn status_bar<'a, M: 'a>(status: &Status)-> Element<'a, M> {
    let battery = row![
        icons::battery(status.battery.percent, status.battery.charging),
        text(format!("{}%", status.battery.percent)).size(13.0),
    ]
    .spacing(4.0)
    .align_y(Center);

    let time = text(status.time.clone()).size(15.0);

    let mut right: Row<'a, M> = Row::new().spacing(8.0).align_y(Center);
    right = right.push(icons::wifi(status.wifi));
    if status.usb {
        right = right.push(icons::usb());
    }

    container(
        row![
            container(battery).align_left(Fill),
            container(time).center_x(Fill),
            container(right).align_right(Fill),
        ]
        .align_y(Center),
    )
    .padding([4.0, 10.0])
    .width(Fill)
    .into()
}