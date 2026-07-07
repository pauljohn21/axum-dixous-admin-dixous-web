use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::router::{route_config, Route};
use crate::theme::{current_theme, theme_css};

const FAVICON: Asset = asset!("/assets/favicon.ico");

/// 应用根组件
#[component]
pub fn App() -> Element {
    let styles = CompleteStyleManager::new().generate_complete_styles();
    let theme_styles = theme_css();

    let theme = current_theme();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        style { "{theme_styles}" }
        style { "{styles}" }
        div {
            "data-theme": "{theme.as_str()}",
            Router::<Route> {
                config: move |_| route_config()
            }
        }
    }
}
