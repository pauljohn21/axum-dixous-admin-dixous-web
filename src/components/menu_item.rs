use dioxus::prelude::*;
use dioxus::router::Link;

use crate::models::menu::{MenuTreeNode, SysMenu};
use crate::router::Route;

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

/// 菜单项组件 — 递归渲染菜单树
#[component]
pub fn MenuItem(
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
