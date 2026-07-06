mod api;
mod app;
mod components;
mod config;
mod http;
mod models;
mod router;
mod storage;

use app::App;

fn main() {
    dioxus::launch(App);
}
