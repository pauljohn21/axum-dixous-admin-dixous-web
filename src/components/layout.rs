use dioxus::prelude::*;
use dioxus::router::Link;
use dioxus_element_plug::prelude::*;

use crate::api;
use crate::api::auth::{build_menu_tree, MenuTreeNode, SysMenu};
use crate::Route;

/// 将菜单 path 映射到 Route 枚举
fn menu_path_to_route(path: &str) -> Option<Route> {
    let path = path.trim_start_matches('/');
    match path {
        "" | "dashboard" => Some(Route::Dashboard {}),
        "users" => Some(Route::UserManage {}),
        "roles" => Some(Route::RoleManage {}),
        "menus" => Some(Route::MenuManage {}),
        "apis" => Some(Route::ApiManage {}),
        "dictionaries" => Some(Route::DictManage {}),
        _ => None,
    }
}

/// 获取菜单图标
fn get_menu_icon(menu: &SysMenu) -> String {
    if let Some(ref icon) = menu.icon {
        if !icon.is_empty() {
            return icon.clone();
        }
    }
    let path = menu.path.as_deref().unwrap_or("");
    match path.trim_start_matches('/') {
        "" | "dashboard" => "📊".to_string(),
        "users" => "👤".to_string(),
        "roles" => "👥".to_string(),
        "menus" => "📋".to_string(),
        "apis" => "🔌".to_string(),
        "dictionaries" => "📖".to_string(),
        _ => "📄".to_string(),
    }
}

/// 管理后台布局 - 侧边栏 + 头部 + 内容区
#[component]
pub fn AdminLayout() -> Element {
    let mut username = use_signal(|| String::new());
    let mut sidebar_collapsed = use_signal(|| false);
    let mut expanded_keys: Signal<Vec<i32>> = use_signal(|| vec![]);
    let navigator = navigator();

    // 获取用户信息（含菜单）
    let user_info = use_resource(move || async move {
        if crate::storage::get_token().is_some() {
            api::auth::get_user_info().await.ok()
        } else {
            None
        }
    });

    // 如果没有token，跳转登录
    use_effect(move || {
        if crate::storage::get_token().is_none() {
            navigator.replace(Route::Login {});
        }
    });

    // 更新用户名和默认展开菜单
    use_effect(move || {
        if let Some(Some(info)) = (user_info)() {
            username.set(info.username.clone());
            // 默认展开所有有子菜单的根节点
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
        crate::storage::clear_token();
        navigator.replace(Route::Login {});
    };

    let toggle_sidebar = move |_| {
        sidebar_collapsed.set(!sidebar_collapsed());
    };

    let sidebar_width = if sidebar_collapsed() { "64px" } else { "220px" };

    // 构建菜单树
    let menu_tree: Vec<MenuTreeNode> = (user_info)()
        .flatten()
        .map(|info| build_menu_tree(&info.menus))
        .unwrap_or_default();

    rsx! {
        div {
            style: "display: flex; min-height: 100vh; background: #f0f2f5;",

            // 侧边栏
            div {
                style: "width: {sidebar_width}; background: #304156; transition: width 0.3s; position: fixed; top: 0; left: 0; bottom: 0; z-index: 1001; overflow-y: auto;",

                // Logo
                div {
                    style: "height: 60px; display: flex; align-items: center; justify-content: center; border-bottom: 1px solid #3d4b5c;",
                    if sidebar_collapsed() {
                        span { style: "font-size: 20px; color: #409eff; font-weight: bold;", "A" }
                    } else {
                        span { style: "font-size: 18px; color: #409eff; font-weight: bold; white-space: nowrap;", "Axum Admin" }
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
                style: "flex: 1; margin-left: {sidebar_width}; transition: margin-left 0.3s; display: flex; flex-direction: column; min-height: 100vh;",

                // 头部
                div {
                    style: "height: 60px; background: white; box-shadow: 0 1px 4px rgba(0,21,41,0.08); display: flex; align-items: center; justify-content: space-between; padding: 0 20px; position: sticky; top: 0; z-index: 1000;",

                    div {
                        style: "display: flex; align-items: center; gap: 16px;",
                        div {
                            style: "cursor: pointer; font-size: 20px; color: #5a5e66;",
                            onclick: toggle_sidebar,
                            if sidebar_collapsed() { "☰" } else { "✕" }
                        }
                    }

                    div {
                        style: "display: flex; align-items: center; gap: 16px;",

                        span {
                            style: "font-size: 14px; color: #606266;",
                            "{username}"
                        }

                        Button {
                            variant: ButtonVariant::Default,
                            size: Some(ButtonSize::Small),
                            on_click: do_logout,
                            "退出登录"
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

/// 菜单项组件 — 递归渲染菜单树
#[component]
fn MenuItem(
    node: MenuTreeNode,
    depth: usize,
    collapsed: bool,
    expanded_keys: Signal<Vec<i32>>,
) -> Element {
    let menu = &node.menu;
    let icon = get_menu_icon(menu);
    let title = menu.title.clone()
        .unwrap_or_else(|| menu.name.clone().unwrap_or_default());
    let has_children = !node.children.is_empty();
    let is_expanded = expanded_keys().contains(&menu.id);
    let menu_id = menu.id;
    let indent = format!("padding-left: {}px;", 20 + depth * 20);

    if has_children && !collapsed {
        // 父节点 — 可展开/折叠
        let arrow = if is_expanded { "▼" } else { "▶" };
        let mut expanded_keys_clone = expanded_keys;

        rsx! {
            div {
                // 父节点标题
                div {
                    style: "{indent} display: flex; align-items: center; padding: 12px 20px; color: #bfcbd9; cursor: pointer; transition: background 0.3s; white-space: nowrap; border-left: 3px solid transparent;",
                    onclick: move |_| {
                        let mut keys = expanded_keys_clone();
                        if keys.contains(&menu_id) {
                            keys.retain(|&k| k != menu_id);
                        } else {
                            keys.push(menu_id);
                        }
                        expanded_keys_clone.set(keys);
                    },
                    span {
                        style: "font-size: 18px; margin-right: 12px; flex-shrink: 0;",
                        "{icon}"
                    }
                    span {
                        style: "font-size: 14px; flex: 1;",
                        "{title}"
                    }
                    span {
                        style: "font-size: 12px; color: #8a9bb0;",
                        "{arrow}"
                    }
                }
                // 子菜单（展开时显示）
                if is_expanded {
                    for child in &node.children {
                        if child.menu.hidden.unwrap_or(0) == 0 {
                            MenuItem {
                                node: child.clone(),
                                depth: depth + 1,
                                collapsed: collapsed,
                                expanded_keys: expanded_keys,
                            }
                        }
                    }
                }
            }
        }
    } else {
        // 叶子菜单 — 可点击导航
        let path = menu.path.clone().unwrap_or_default();
        let route = menu_path_to_route(&path);

        if let Some(r) = route {
            rsx! {
                Link {
                    to: r,
                    div {
                        style: "{indent} display: flex; align-items: center; padding: 12px 20px; color: #bfcbd9; cursor: pointer; transition: background 0.3s; text-decoration: none; white-space: nowrap; border-left: 3px solid transparent;",
                        span {
                            style: "font-size: 18px; margin-right: 12px; flex-shrink: 0;",
                            "{icon}"
                        }
                        if !collapsed {
                            span {
                                style: "font-size: 14px;",
                                "{title}"
                            }
                        }
                    }
                }
            }
        } else {
            // 没有对应路由的菜单 — 仅显示文本
            rsx! {
                div {
                    style: "{indent} display: flex; align-items: center; padding: 12px 20px; color: #bfcbd9; cursor: default; white-space: nowrap; border-left: 3px solid transparent;",
                    span {
                        style: "font-size: 18px; margin-right: 12px; flex-shrink: 0;",
                        "{icon}"
                    }
                    if !collapsed {
                        span {
                            style: "font-size: 14px;",
                            "{title}"
                        }
                    }
                }
            }
        }
    }
}
