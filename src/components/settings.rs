use dioxus::prelude::*;

use crate::i18n::{current_locale, set_locale, t, Locale, TKey};
use crate::theme::{current_theme, set_theme, ThemeMode};

/// 系统设置页面
#[component]
pub fn Settings() -> Element {
    let locale = current_locale();
    let theme = current_theme();

    rsx! {
        div {
            style: "padding: 20px;",

            h2 {
                style: "margin: 0 0 20px 0; font-size: 24px; font-weight: 500; color: var(--el-text-color-primary);",
                "{t(TKey::Settings)}"
            }

            // ===== 系统设置卡片 =====
            div {
                style: "background: var(--el-bg-color); border-radius: 4px; box-shadow: var(--el-box-shadow); margin-bottom: 20px;",

                // 卡片头部
                div {
                    style: "padding: 16px 20px; border-bottom: 1px solid var(--el-border-color-lighter); font-size: 16px; font-weight: 500; color: var(--el-text-color-primary);",
                    "{t(TKey::SystemSettings)}"
                }

                // 卡片内容
                div {
                    style: "padding: 20px; display: flex; flex-direction: column; gap: 24px;",

                    // 语言设置
                    div {
                        style: "display: flex; align-items: center; gap: 16px;",

                        label {
                            style: "width: 100px; color: var(--el-text-color-regular); font-size: 14px;",
                            "{t(TKey::Language)}:"
                        }

                        select {
                            style: "padding: 8px 12px; border: 1px solid var(--el-border-color); border-radius: 4px; font-size: 14px; color: var(--el-text-color-regular); background: var(--el-bg-color); cursor: pointer;",
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

                    // 主题模式设置
                    div {
                        style: "display: flex; align-items: center; gap: 16px;",

                        label {
                            style: "width: 100px; color: var(--el-text-color-regular); font-size: 14px;",
                            "{t(TKey::ThemeMode)}:"
                        }

                        // 亮色/暗色切换按钮组
                        div {
                            style: "display: flex; gap: 12px;",

                            // 亮色
                            div {
                                style: if theme == ThemeMode::Light {
                                    "display: flex; flex-direction: column; align-items: center; gap: 8px; padding: 16px 24px; border: 2px solid var(--el-color-primary); border-radius: 8px; cursor: pointer; background: var(--el-color-primary-light-9); transition: all var(--el-transition-duration);"
                                } else {
                                    "display: flex; flex-direction: column; align-items: center; gap: 8px; padding: 16px 24px; border: 2px solid var(--el-border-color); border-radius: 8px; cursor: pointer; background: var(--el-fill-color-lighter); transition: all var(--el-transition-duration);"
                                },
                                onclick: move |_| set_theme(ThemeMode::Light),

                                div {
                                    style: "font-size: 32px;",
                                    "☀"
                                }
                                span {
                                    style: "font-size: 14px; color: var(--el-text-color-regular);",
                                    "{t(TKey::LightMode)}"
                                }
                            }

                            // 暗色
                            div {
                                style: if theme == ThemeMode::Dark {
                                    "display: flex; flex-direction: column; align-items: center; gap: 8px; padding: 16px 24px; border: 2px solid var(--el-color-primary); border-radius: 8px; cursor: pointer; background: var(--el-color-primary-light-9); transition: all var(--el-transition-duration);"
                                } else {
                                    "display: flex; flex-direction: column; align-items: center; gap: 8px; padding: 16px 24px; border: 2px solid var(--el-border-color); border-radius: 8px; cursor: pointer; background: var(--el-fill-color-lighter); transition: all var(--el-transition-duration);"
                                },
                                onclick: move |_| set_theme(ThemeMode::Dark),

                                div {
                                    style: "font-size: 32px;",
                                    "🌙"
                                }
                                span {
                                    style: "font-size: 14px; color: var(--el-text-color-regular);",
                                    "{t(TKey::DarkMode)}"
                                }
                            }
                        }
                    }
                }
            }

            // ===== 主题色预览卡片 =====
            div {
                style: "background: var(--el-bg-color); border-radius: 4px; box-shadow: var(--el-box-shadow);",

                // 卡片头部
                div {
                    style: "padding: 16px 20px; border-bottom: 1px solid var(--el-border-color-lighter); font-size: 16px; font-weight: 500; color: var(--el-text-color-primary);",
                    "{t(TKey::ThemeColor)}"
                }

                // 卡片内容 — 色彩预览
                div {
                    style: "padding: 20px; display: grid; grid-template-columns: repeat(auto-fit, minmax(160px, 1fr)); gap: 16px;",

                    // 主色
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-size: 14px; color: var(--el-text-color-secondary);", "{t(TKey::PrimaryColor)}" }
                        div {
                            style: "height: 60px; border-radius: 8px; background: var(--el-color-primary); display: flex; align-items: center; justify-content: center; color: white; font-size: 14px; font-weight: 600;",
                            "#409EFF"
                        }
                    }

                    // 成功色
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-size: 14px; color: var(--el-text-color-secondary);", "{t(TKey::SuccessColor)}" }
                        div {
                            style: "height: 60px; border-radius: 8px; background: var(--el-color-success); display: flex; align-items: center; justify-content: center; color: white; font-size: 14px; font-weight: 600;",
                            "#67C23A"
                        }
                    }

                    // 警告色
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-size: 14px; color: var(--el-text-color-secondary);", "{t(TKey::WarningColor)}" }
                        div {
                            style: "height: 60px; border-radius: 8px; background: var(--el-color-warning); display: flex; align-items: center; justify-content: center; color: white; font-size: 14px; font-weight: 600;",
                            "#E6A23C"
                        }
                    }

                    // 危险色
                    div {
                        style: "display: flex; flex-direction: column; gap: 8px;",
                        span { style: "font-size: 14px; color: var(--el-text-color-secondary);", "{t(TKey::DangerColor)}" }
                        div {
                            style: "height: 60px; border-radius: 8px; background: var(--el-color-danger); display: flex; align-items: center; justify-content: center; color: white; font-size: 14px; font-weight: 600;",
                            "#F56C6C"
                        }
                    }
                }
            }
        }
    }
}
