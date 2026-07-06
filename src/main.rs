mod api;
mod app;
mod components;
mod config;
mod http;
mod i18n;
mod models;
mod router;
mod storage;

use app::App;

fn main() {
    i18n::init_locale();
    dioxus::launch(App);
}
