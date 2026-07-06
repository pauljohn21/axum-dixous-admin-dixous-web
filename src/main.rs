mod api;
mod components;

use dioxus::prelude::*;
use dioxus::router::Link;
use dioxus_element_plug::prelude::*;

use components::{
    api_manage::ApiManage,
    dashboard::Dashboard,
    dict_manage::DictManage,
    layout::AdminLayout,
    login::Login,
    menu_manage::MenuManage,
    role_manage::RoleManage,
    user_manage::UserManage,
};

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let styles = CompleteStyleManager::new().generate_complete_styles();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        style { "{styles}" }
        Router::<Route> {}
    }
}

/// 路由枚举
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/login")]
    Login {},

    #[layout(AdminLayout)]
        #[route("/")]
        Dashboard {},
        #[route("/users")]
        UserManage {},
        #[route("/roles")]
        RoleManage {},
        #[route("/menus")]
        MenuManage {},
        #[route("/apis")]
        ApiManage {},
        #[route("/dictionaries")]
        DictManage {},
    #[end_layout]

    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

/// 404 页面
#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; background: #f0f2f5;",
            h1 {
                style: "font-size: 72px; font-weight: 700; color: #409eff; margin: 0 0 16px 0;",
                "404"
            }
            p {
                style: "font-size: 18px; color: #909399; margin: 0 0 24px 0;",
                "页面未找到: /{route.join(\"/\")}"
            }
            Link {
                to: Route::Dashboard {},
                Button {
                    variant: ButtonVariant::Primary,
                    "返回首页"
                }
            }
        }
    }
}
