use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::i18n::provide_locale;
use crate::router::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");

/// 应用根组件
#[component]
pub fn App() -> Element {
    let styles = CompleteStyleManager::new().generate_complete_styles();
    provide_locale();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        style { "{styles}" }
        Router::<Route> {}
    }
}
