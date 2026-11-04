mod app;
mod config;
mod fonts;
mod input;
mod library;
mod reader;
mod screens;
mod status;
mod storage;
mod ui;

use app::App;

fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("E-reader")
        .subscription(App::subscription)
        .theme(App::theme)
        .font(fonts::SANS)
        .font(fonts::SERIF)
        .font(fonts::MONO)
        .default_font(fonts::ui_font())
        .window_size(config::window_size())
        .resizable(false)
        .run()
}