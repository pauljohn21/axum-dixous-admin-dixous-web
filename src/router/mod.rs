use dioxus::prelude::*;
use dioxus::router::Link;
use dioxus_element_plug::prelude::*;

use crate::components::admin_layout::AdminLayout;
use crate::components::api_manage::ApiManage;
use crate::components::dashboard::Dashboard;
use crate::components::dict_manage::DictManage;
use crate::components::login::Login;
use crate::components::menu_manage::MenuManage;
use crate::components::profile::Profile;
use crate::components::role_manage::RoleManage;
use crate::components::settings::Settings;
use crate::components::user_manage::UserManage;
use crate::i18n::{t, TKey};

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
        #[route("/profile")]
        Profile {},
        #[route("/settings")]
        Settings {},
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
                "{t(TKey::PageNotFound)}: /{route.join(\"/\")}"
            }
            Link {
                to: Route::Dashboard {},
                Button {
                    variant: ButtonVariant::Primary,
                    "{t(TKey::BackHome)}"
                }
            }
        }
    }
}
