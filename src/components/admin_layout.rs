use dioxus::prelude::*;

use crate::api;
use crate::components::menu_item::{MenuItem, MENU_CSS};
use crate::components::user_dropdown::UserDropdown;
use crate::i18n::{current_locale, set_locale, Locale};
use crate::models::menu::{build_menu_tree, MenuTreeNode};
use crate::router::Route;
use crate::storage;
use crate::theme::{current_theme, toggle_theme, ThemeMode};

/// 管理后台布局 - 侧边栏 + 头部 + 内容区
#[component]
pub fn AdminLayout() -> Element {
    let mut username = use_signal(String::new);
    let mut sidebar_collapsed = use_signal(|| false);
    let mut expanded_keys: Signal<Vec<i32>> = use_signal(std::vec::Vec::new);
    let navigator = navigator();

    // 获取用户信息（含菜单）
    let user_info = use_resource(move || async move {
        if storage::get_token().is_some() {
            api::auth::get_user_info().await.ok()
        } else {
            None
        }
    });

    // 更新用户名和默认展开菜单
    use_effect(move || {
        if let Some(Some(info)) = (user_info)() {
            username.set(info.username.clone());
            let tree = build_menu_tree(&info.menus);
            let expand_ids: Vec<i32> = tree.iter()
                .filter(|n| !n.children.is_empty())
                .map(|n| n.menu.id)
                .collect();
            if !expand_ids.is_empty() && expanded_keys().is_empty() {
                expanded_keys.set(expand_ids);
            }
        }
    });

    let do_logout = move |_| {
        spawn(async move {
            // 通知后端将 token 加入黑名单
            let _ = api::auth::logout().await;
            // 清除前端 token
            storage::clear_token();
            navigator.replace(Route::Login {});
        });
    };

    let toggle_sidebar = move |_| {
        sidebar_collapsed.set(!sidebar_collapsed());
    };

    let sidebar_width = if sidebar_collapsed() { "64px" } else { "220px" };
    let theme = current_theme();

    // 构建菜单树
    let menu_tree: Vec<MenuTreeNode> = (user_info)()
        .flatten()
        .map(|info| build_menu_tree(&info.menus))
        .unwrap_or_default();

    rsx! {
        // 注入菜单 CSS
        style { "{MENU_CSS}" }

        div {
            style: "display: flex; min-height: 100vh; background: var(--el-bg-color-page);",

            // 侧边栏
            div {
                style: "width: {sidebar_width}; background: var(--el-sidebar-bg); transition: width var(--el-transition-duration); position: fixed; top: 0; left: 0; bottom: 0; z-index: 1001; overflow-y: auto;",

                // Logo
                div {
                    style: "height: 60px; display: flex; align-items: center; justify-content: center; border-bottom: 1px solid var(--el-sidebar-bg-deep);",
                    if sidebar_collapsed() {
                        span { style: "font-size: 20px; color: var(--el-color-primary); font-weight: bold;", "A" }
                    } else {
                        span { style: "font-size: 18px; color: var(--el-color-primary); font-weight: bold; white-space: nowrap;", "Axum Admin" }
                    }
                }

                // 动态菜单
                div {
                    style: "padding-top: 10px;",
                    for node in &menu_tree {
                        if node.menu.hidden.unwrap_or(0) == 0 {
                            MenuItem {
                                node: node.clone(),
                                depth: 0,
                                collapsed: sidebar_collapsed(),
                                expanded_keys: expanded_keys,
                            }
                        }
                    }
                }
            }

            // 主内容区
            div {
                style: "flex: 1; margin-left: {sidebar_width}; transition: margin-left var(--el-transition-duration); display: flex; flex-direction: column; min-height: 100vh;",

                // 头部
                div {
                    style: "height: 60px; background: var(--el-header-bg); box-shadow: var(--el-header-shadow); display: flex; align-items: center; justify-content: space-between; padding: 0 20px; position: sticky; top: 0; z-index: 1000;",

                    div {
                        style: "display: flex; align-items: center; gap: 16px;",
                        div {
                            style: "cursor: pointer; font-size: 20px; color: var(--el-header-text);",
                            onclick: toggle_sidebar,
                            if sidebar_collapsed() { "☰" } else { "✕" }
                        }
                    }

                    div {
                        style: "display: flex; align-items: center; gap: 16px;",

                        // 主题切换
                        div {
                            style: "display: flex; align-items: center;",
                            button {
                                style: "padding: 6px 10px; font-size: 16px; border: 1px solid var(--el-border-color); border-radius: 4px; cursor: pointer; background: transparent; color: var(--el-header-text); transition: all 0.2s;",
                                onclick: move |_| toggle_theme(),
                                if theme == ThemeMode::Dark { "☀" } else { "🌙" }
                            }
                        }

                        // 语言切换
                        div {
                            style: "display: flex; align-items: center; gap: 4px;",
                            button {
                                style: "padding: 4px 8px; font-size: 12px; border: 1px solid var(--el-border-color); border-radius: 4px; cursor: pointer; background: transparent; color: var(--el-text-color-regular);",
                                onclick: move |_| {
                                    let cur = current_locale();
                                    set_locale(match cur {
                                        Locale::ZhCN => Locale::EnUS,
                                        Locale::EnUS => Locale::ZhCN,
                                    });
                                },
                                "{current_locale().label()}"
                            }
                        }

                        // 用户下拉菜单
                        UserDropdown {
                            username: username(),
                            on_profile: move |_| {
                                navigator.push(Route::Profile {});
                            },
                            on_settings: move |_| {
                                navigator.push(Route::Settings {});
                            },
                            on_logout: do_logout,
                        }
                    }
                }

                // 内容区域
                div {
                    style: "flex: 1; padding: 20px; overflow-y: auto;",
                    Outlet::<Route> {}
                }
            }
        }
    }
}
