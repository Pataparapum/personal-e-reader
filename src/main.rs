mod app;
mod config;
mod input;
mod screens;
mod status;
mod ui;

use app::App;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("E-reader")
        .subscription(App::subscription)
        .theme(App::theme)
        .window_size(config::window_size())
        .resizable(false)
        .run()
}