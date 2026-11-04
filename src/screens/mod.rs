pub mod home;
pub mod library;
pub mod lock;
pub mod placeholder;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Route {
    Home,
    Library,
    Audiobooks,
}

pub enum Action {
    Navigate(Route)
}