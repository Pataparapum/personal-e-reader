use iced::widget::{Container, container, column, row};
use iced::{Element, Never};
fn main() -> iced::Result {
    iced::application(|| (), (), view)
        .title("E-reader")
        .window_size(iced::Size::new(300.0, 400.0))
        .run()
}

fn view(_:&()) -> Container<'_, Never> {
    
    container(explorador())
}

fn explorador() -> Element<'static, Never> {

    container(navbar()).into()

}

fn navbar() -> Element<'static, Never> {
    use iced::Fill;

    container(
        column![
            "top",
            row!["left", "right"].spacing(10),
            "bottom"
        ]
        .spacing(10)
    )
    .padding(10)
    .center_x(Fill)
    .center_y(Fill)
    .into()

}