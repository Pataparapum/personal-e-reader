use iced::Size;

// Panel Waveshare 4.2 (400x300) usado en vertial
pub const SCREEN_WIDTH: f32 = 300.0;
pub const SCREEN_HEIGHT: f32 = 400.0;
pub const BOOKS_DIR: &str = "libros";

pub fn window_size() -> Size {
    Size::new(SCREEN_WIDTH, SCREEN_HEIGHT)
}