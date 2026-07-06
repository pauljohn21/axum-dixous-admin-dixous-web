use dioxus::prelude::*;
use dioxus::router::use_route;

use crate::models::menu::{MenuTreeNode, SysMenu};
use crate::router::Route;

/// 菜单项 CSS — 在 admin_layout 中注入全局
pub const MENU_CSS: &str = r#"
.sidebar-menu-item {
    display: flex;
    align-items: center;
    padding: 10px 20px;
    color: #bfcbd9;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
    border-left: 3px solid transparent;
    text-decoration: none;
    user-select: none;
}
.sidebar-menu-item:hover {
    background: #263445 !important;
    color: #fff !important;
}
.sidebar-menu-item-active {
    background: #1f2d3d !important;
    border-left-color: #409eff !important;
    color: #fff !important;
}
"#;

/// 将菜单 path 映射到 Route 枚举
fn menu_path_to_route(path: &str) -> Option<Route> {
    let path = path.trim_start_matches('/');
    match path {
        "" | "dashboard" => Some(Route::Dashboard {}),
        "user" | "users" => Some(Route::UserManage {}),
        "role" | "roles" => Some(Route::RoleManage {}),
        "menu" | "menus" => Some(Route::MenuManage {}),
        "api" | "apis" => Some(Route::ApiManage {}),
        "dictionary" | "dictionaries" => Some(Route::DictManage {}),
        _ => None,
    }
}

/// 判断某菜单的 route 是否与当前路由匹配
fn is_active_route(route: &Option<Route>, current: &Route) -> bool {
    if let Some(r) = route {
        r == current
    } else {
        false
    }
}

/// 递归检查子菜单中是否有匹配当前路由的
fn has_active_child(node: &MenuTreeNode, current: &Route) -> bool {
    for child in &node.children {
        let path = child.menu.path.clone().unwrap_or_default();
        if let Some(r) = menu_path_to_route(&path) {
            if r == *current {
                return true;
            }
        }
        if has_active_child(child, current) {
            return true;
        }
    }
    false
}

/// 获取菜单图标 — 将 Element Plus 图标类名映射为 emoji
fn get_menu_icon(menu: &SysMenu) -> String {
    // 优先使用后端返回的 icon 字段（映射为 emoji）
    if let Some(ref icon) = menu.icon {
        if !icon.is_empty() {
            return map_icon_class_to_emoji(icon);
        }
    }
    // 根据 path 兜底
    let path = menu.path.as_deref().unwrap_or("");
    match path.trim_start_matches('/') {
        "" | "dashboard" => "📊".to_string(),
        "user" | "users" => "👤".to_string(),
        "role" | "roles" => "👥".to_string(),
        "menu" | "menus" => "📋".to_string(),
        "api" | "apis" => "🔌".to_string(),
        "dictionary" | "dictionaries" => "📖".to_string(),
        _ => "📄".to_string(),
    }
}

/// 将 Element Plus 图标类名映射为 emoji
fn map_icon_class_to_emoji(icon_class: &str) -> String {
    match icon_class {
        // 仪表盘/首页
        "odometer" | "speedometer" | "dashboard" => "📊",
        // 用户相关
        "user" | "user-filled" => "👤",
        "avatar" | "user-solid" | "peoples" => "👥",
        // 角色/权限
        "lock" | "key" => "🔐",
        // 菜单
        "tickets" | "menu" | "list" => "📋",
        // API/接口
        "platform" | "api" | "connection" => "🔌",
        // 字典
        "dict" | "dictionary" | "book" => "📖",
        // 操作记录/历史
        "operation" | "history" | "time" | "pie-chart" => "📜",
        // 消息/通知
        "message" | "bell" | "notification" => "💬",
        // 工具
        "tools" | "setting" | "settings" | "tool" => "🛠️",
        // 系统配置
        "system" | "monitor" | "cloudy" => "⚙️",
        // 关于/信息
        "info-filled" | "info" | "about" => "ℹ️",
        // 默认
        _ => "📄",
    }
    .to_string()
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
    let title = menu
        .title
        .clone()
        .unwrap_or_else(|| menu.name.clone().unwrap_or_default());
    let has_children = !node.children.is_empty();
    let is_expanded = expanded_keys().contains(&menu.id);
    let menu_id = menu.id;
    let indent = format!("padding-left: {}px;", 16 + depth * 20);
    let navigator = navigator();
    let current_route = use_route::<Route>();

    // 判断当前菜单是否激活
    let path = menu.path.clone().unwrap_or_default();
    let route = menu_path_to_route(&path);
    let is_active = is_active_route(&route, &current_route);
    let child_active = has_active_child(&node, &current_route);

    if has_children && !collapsed {
        // 父节点 — 可展开/折叠
        let arrow = if is_expanded { "▼" } else { "▶" };
        let mut expanded_keys_clone = expanded_keys;
        let highlight = is_expanded || child_active;
        let class = if highlight {
            "sidebar-menu-item sidebar-menu-item-active"
        } else {
            "sidebar-menu-item"
        };

        rsx! {
            div {
                // 父节点标题
                div {
                    class: "{class}",
                    style: "{indent}",
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
                        style: "font-size: 18px; margin-right: 10px; flex-shrink: 0;",
                        "{icon}"
                    }
                    span {
                        style: "font-size: 14px; flex: 1; overflow: hidden; text-overflow: ellipsis;",
                        "{title}"
                    }
                    span {
                        style: "font-size: 12px; color: #8a9bb0; flex-shrink: 0;",
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
    } else if has_children && collapsed {
        // 折叠状态下，父菜单 — 点击展开侧边栏并展开该菜单
        let class = if child_active {
            "sidebar-menu-item sidebar-menu-item-active"
        } else {
            "sidebar-menu-item"
        };
        let mut expanded_keys_clone = expanded_keys;

        rsx! {
            div {
                class: "{class}",
                style: "{indent}",
                title: "{title}",
                onclick: move |_| {
                    let mut keys = expanded_keys_clone();
                    if !keys.contains(&menu_id) {
                        keys.push(menu_id);
                    }
                    expanded_keys_clone.set(keys);
                },
                span {
                    style: "font-size: 18px; flex-shrink: 0;",
                    "{icon}"
                }
            }
        }
    } else {
        // 叶子菜单 — 可点击导航
        let path = menu.path.clone().unwrap_or_default();
        let route = menu_path_to_route(&path);

        if let Some(r) = route {
            let route_clone = r.clone();
            let class = if is_active {
                "sidebar-menu-item sidebar-menu-item-active"
            } else {
                "sidebar-menu-item"
            };

            rsx! {
                div {
                    class: "{class}",
                    style: "{indent}",
                    onclick: move |_| {
                        navigator.push(route_clone.clone());
                    },
                    span {
                        style: "font-size: 18px; margin-right: 10px; flex-shrink: 0;",
                        "{icon}"
                    }
                    if !collapsed {
                        span {
                            style: "font-size: 14px; overflow: hidden; text-overflow: ellipsis;",
                            "{title}"
                        }
                    }
                }
            }
        } else {
            // 没有对应路由的菜单 — 仅显示文本
            rsx! {
                div {
                    class: "sidebar-menu-item",
                    style: "{indent}",
                    title: "{title}",
                    span {
                        style: "font-size: 18px; margin-right: 10px; flex-shrink: 0;",
                        "{icon}"
                    }
                    if !collapsed {
                        span {
                            style: "font-size: 14px; overflow: hidden; text-overflow: ellipsis;",
                            "{title}"
                        }
                    }
                }
            }
        }
    }
}
