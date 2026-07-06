use dioxus::prelude::*;
use dioxus::router::Link;
use dioxus_element_plug::prelude::*;

use crate::api;
use crate::Route;

/// 管理后台布局 - 侧边栏 + 头部 + 内容区
#[component]
pub fn AdminLayout() -> Element {
    let mut username = use_signal(|| String::new());
    let mut sidebar_collapsed = use_signal(|| false);
    let navigator = navigator();

    // 获取用户信息
    let user_info = use_resource(move || async move {
        if api::get_token().is_some() {
            api::auth::get_user_info().await.ok()
        } else {
            None
        }
    });

    // 如果没有token，跳转登录
    use_effect(move || {
        if api::get_token().is_none() {
            navigator.replace(Route::Login {});
        }
    });

    // 更新用户名
    use_effect(move || {
        if let Some(Some(info)) = (user_info)() {
            username.set(info.username);
        }
    });

    let do_logout = move |_| {
        api::clear_token();
        navigator.replace(Route::Login {});
    };

    let toggle_sidebar = move |_| {
        sidebar_collapsed.set(!sidebar_collapsed());
    };

    let sidebar_width = if sidebar_collapsed() { "64px" } else { "220px" };

    let menu_items = vec![
        ("仪表盘", "📊", Route::Dashboard {}),
        ("用户管理", "👤", Route::UserManage {}),
        ("角色管理", "👥", Route::RoleManage {}),
        ("菜单管理", "📋", Route::MenuManage {}),
        ("API管理", "🔌", Route::ApiManage {}),
        ("字典管理", "📖", Route::DictManage {}),
    ];

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

                // 菜单项
                div {
                    style: "padding-top: 10px;",
                    for (label, icon, route) in menu_items {
                        { rsx! {
                            Link {
                                to: route,
                                div {
                                    style: "display: flex; align-items: center; padding: 12px 20px; color: #bfcbd9; cursor: pointer; transition: background 0.3s; text-decoration: none; white-space: nowrap; border-left: 3px solid transparent;",
                                    onmouseenter: move |e: MouseEvent| {
                                        let _ = e;
                                    },
                                    span {
                                        style: "font-size: 18px; margin-right: 12px; flex-shrink: 0;",
                                        "{icon}"
                                    }
                                    if !sidebar_collapsed() {
                                        span {
                                            style: "font-size: 14px;",
                                            "{label}"
                                        }
                                    }
                                }
                            }
                        }}
                    }
                }
            }

            // 主内容区
            div {
                style: "flex: 1; margin-left: {sidebar_width}; transition: margin-left 0.3s; display: flex; flex-direction: column; min-height: 100vh;",

                // 头部
                div {
                    style: "height: 60px; background: white; box-shadow: 0 1px 4px rgba(0,21,41,0.08); display: flex; align-items: center; justify-content: space-between; padding: 0 20px; position: sticky; top: 0; z-index: 1000;",

                    // 折叠按钮
                    div {
                        style: "display: flex; align-items: center; gap: 16px;",
                        div {
                            style: "cursor: pointer; font-size: 20px; color: #5a5e66;",
                            onclick: toggle_sidebar,
                            if sidebar_collapsed() { "☰" } else { "✕" }
                        }
                    }

                    // 用户信息
                    div {
                        style: "display: flex; align-items: center; gap: 16px;",

                        // 用户名
                        span {
                            style: "font-size: 14px; color: #606266;",
                            "{username}"
                        }

                        // 退出按钮
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
