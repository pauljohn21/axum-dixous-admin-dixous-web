use dioxus::prelude::*;
use dioxus::router::{Link, RouterConfig};
use dioxus_element_plug::prelude::*;
use route_guard::RouteGuard;

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
use crate::storage;

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

/// 创建带全局路由守卫的 RouterConfig
///
/// 使用 `route-guard` crate 实现，类似 Vue Router 的 beforeEach 全局守卫：
/// - 未登录用户访问受保护路由 → 自动跳转登录页
/// - 已登录用户访问登录页 → 自动跳转首页
///
/// 守卫在路由变化后、组件渲染前执行，因此不会有页面闪烁。
pub fn route_config() -> RouterConfig<Route> {
    RouteGuard::<Route>::new()
        .is_authenticated(|| storage::get_token().is_some())
        .requires_auth(|route| !matches!(route, Route::Login {}))
        .login_route(Route::Login {})
        .home_route(Route::Dashboard {})
        .config()
}

/// 404 页面
#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 100vh; background: var(--el-bg-color-page);",
            h1 {
                style: "font-size: 72px; font-weight: 700; color: var(--el-color-primary); margin: 0 0 16px 0;",
                "404"
            }
            p {
                style: "font-size: 18px; color: var(--el-text-color-secondary); margin: 0 0 24px 0;",
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
