mod api;
mod app;
mod components;
mod config;
mod http;
mod i18n;
mod theme;

mod models;
mod router;
mod storage;

use app::App;

fn main() {
    i18n::init_locale();
    theme::init_theme();
    dioxus::launch(App);
}
