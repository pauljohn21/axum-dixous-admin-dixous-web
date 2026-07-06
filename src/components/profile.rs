use dioxus::prelude::*;

use crate::i18n::{t, TKey};
use crate::storage::get_username;

/// 个人信息页面
#[component]
pub fn Profile() -> Element {
    let username = get_username().unwrap_or_else(|| t(TKey::Guest).to_string());

    rsx! {
        div {
            style: "padding: 20px;",

            h2 {
                style: "margin: 0 0 20px 0; font-size: 24px; font-weight: 500; color: #303133;",
                "{t(TKey::Profile)}"
            }

            div {
                style: "background: white; border-radius: 4px; box-shadow: 0 2px 12px rgba(0,0,0,0.1);",

                // 卡片头部
                div {
                    style: "padding: 16px 20px; border-bottom: 1px solid #ebeef5; font-size: 16px; font-weight: 500; color: #303133;",
                    "{t(TKey::BasicInfo)}"
                }

                // 卡片内容
                div {
                    style: "padding: 20px; display: flex; flex-direction: column; gap: 16px;",

                    // 用户名
                    div {
                        style: "display: flex; align-items: center; gap: 16px;",

                        label {
                            style: "width: 100px; color: #606266; font-size: 14px;",
                            "{t(TKey::Username)}:"
                        }
                        span {
                            style: "color: #303133; font-size: 14px;",
                            "{username}"
                        }
                    }

                    // 头像
                    div {
                        style: "display: flex; align-items: center; gap: 16px;",

                        label {
                            style: "width: 100px; color: #606266; font-size: 14px;",
                            "{t(TKey::Avatar)}:"
                        }
                        div {
                            style: "width: 64px; height: 64px; border-radius: 50%; background: #409eff; display: flex; align-items: center; justify-content: center; color: white; font-size: 24px;",
                            "{username.chars().next().unwrap_or('U')}"
                        }
                    }
                }
            }
        }
    }
}
