use dioxus::prelude::*;

use crate::i18n::{t, TKey, current_locale, set_locale, Locale};

/// 系统设置页面
#[component]
pub fn Settings() -> Element {
    let locale = current_locale();

    rsx! {
        div {
            style: "padding: 20px;",

            h2 {
                style: "margin: 0 0 20px 0; font-size: 24px; font-weight: 500; color: #303133;",
                "{t(TKey::Settings)}"
            }

            div {
                style: "background: white; border-radius: 4px; box-shadow: 0 2px 12px rgba(0,0,0,0.1);",

                // 卡片头部
                div {
                    style: "padding: 16px 20px; border-bottom: 1px solid #ebeef5; font-size: 16px; font-weight: 500; color: #303133;",
                    "{t(TKey::SystemSettings)}"
                }

                // 卡片内容
                div {
                    style: "padding: 20px; display: flex; flex-direction: column; gap: 24px;",

                    // 语言设置
                    div {
                        style: "display: flex; align-items: center; gap: 16px;",

                        label {
                            style: "width: 100px; color: #606266; font-size: 14px;",
                            "{t(TKey::Language)}:"
                        }

                        select {
                            style: "padding: 8px 12px; border: 1px solid #dcdfe6; border-radius: 4px; font-size: 14px; color: #606266; background: white; cursor: pointer;",
                            onchange: move |evt| {
                                let value = evt.value();
                                match value.as_str() {
                                    "zh-CN" => set_locale(Locale::ZhCN),
                                    "en-US" => set_locale(Locale::EnUS),
                                    _ => {}
                                }
                            },

                            option {
                                value: "zh-CN",
                                selected: matches!(locale, Locale::ZhCN),
                                "中文"
                            }
                            option {
                                value: "en-US",
                                selected: matches!(locale, Locale::EnUS),
                                "English"
                            }
                        }
                    }

                    // 主题设置
                    div {
                        style: "display: flex; align-items: center; gap: 16px;",

                        label {
                            style: "width: 100px; color: #606266; font-size: 14px;",
                            "{t(TKey::Theme)}:"
                        }

                        span {
                            style: "color: #909399; font-size: 14px;",
                            "{t(TKey::ComingSoon)}"
                        }
                    }
                }
            }
        }
    }
}
