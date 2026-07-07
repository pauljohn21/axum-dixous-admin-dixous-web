use dioxus::prelude::*;
use crate::i18n::{t, TKey};

/// 用户下拉菜单组件
#[component]
pub fn UserDropdown(
    username: String,
    on_profile: EventHandler<()>,
    on_settings: EventHandler<()>,
    on_logout: EventHandler<()>,
) -> Element {
    let mut is_open = use_signal(|| false);

    rsx! {
        div {
            style: "position: relative;",

            // 用户名按钮
            button {
                style: "display: flex; align-items: center; gap: 8px; padding: 8px 12px; border: none; background: transparent; cursor: pointer; font-size: 14px; color: var(--el-header-text); border-radius: 4px; transition: background 0.2s;",
                onclick: move |_| is_open.set(!is_open()),
                onmouseenter: move |_| is_open.set(true),

                span { "{username}" }
                span { "▼" }
            }

            // 下拉菜单
            if is_open() {
                div {
                    style: "position: absolute; top: 100%; right: 0; margin-top: 4px; background: var(--el-bg-color-overlay); border-radius: 4px; box-shadow: var(--el-box-shadow); min-width: 160px; z-index: 1001;",
                    onmouseleave: move |_| is_open.set(false),

                    // 个人信息
                    div {
                        style: "padding: 10px 16px; cursor: pointer; font-size: 14px; color: var(--el-text-color-regular); transition: background 0.2s;",
                        onclick: move |_| {
                            is_open.set(false);
                            on_profile.call(());
                        },
                        "{t(TKey::Profile)}"
                    }

                    // 系统配置
                    div {
                        style: "padding: 10px 16px; cursor: pointer; font-size: 14px; color: var(--el-text-color-regular); transition: background 0.2s;",
                        onclick: move |_| {
                            is_open.set(false);
                            on_settings.call(());
                        },
                        "{t(TKey::Settings)}"
                    }

                    // 分隔线
                    div { style: "height: 1px; background: var(--el-border-color-lighter); margin: 4px 0;" }

                    // 退出登录
                    div {
                        style: "padding: 10px 16px; cursor: pointer; font-size: 14px; color: var(--el-color-danger); transition: background 0.2s;",
                        onclick: move |_| {
                            is_open.set(false);
                            on_logout.call(());
                        },
                        "{t(TKey::Logout)}"
                    }
                }
            }
        }
    }
}
