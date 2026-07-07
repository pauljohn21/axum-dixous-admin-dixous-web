use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::api;
use crate::i18n::{current_locale, set_locale, t, Locale, TKey};
use crate::router::Route;
use crate::storage;
use slider_captcha::SliderCaptcha;

/// 登录页面
#[component]
pub fn Login() -> Element {
    let mut username = use_signal(|| "admin".to_string());
    let mut password = use_signal(String::new);
    let mut error_msg = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let mut captcha_verified = use_signal(|| false);
    let navigator = navigator();

    let do_login = move |_| {
        let username_val = username();
        let password_val = password();
        if username_val.is_empty() || password_val.is_empty() {
            error_msg.set(Some(t(TKey::UsernamePasswordRequired)));
            return;
        }
        if !captcha_verified() {
            error_msg.set(Some(t(TKey::SliderVerifyFirst)));
            return;
        }

        loading.set(true);
        error_msg.set(None);

        spawn(async move {
            match api::auth::login(username_val.clone(), password_val).await {
                Ok(resp) => {
                    storage::set_token(&resp.token);
                    storage::set_username(&username_val);
                    navigator.replace(Route::Dashboard {});
                }
                Err(e) => {
                    error_msg.set(Some(e));
                    loading.set(false);
                }
            }
        });
    };

    rsx! {
        div {
            class: "login-container",
            style: "min-height: 100vh; display: flex; align-items: center; justify-content: center; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);",

            div {
                style: "background: var(--el-bg-color); border-radius: 12px; padding: 40px; width: 400px; box-shadow: 0 20px 60px rgba(0,0,0,0.3);",

                // Logo
                div {
                    style: "text-align: center; margin-bottom: 30px;",
                    h1 {
                        style: "font-size: 28px; font-weight: 700; color: var(--el-text-color-primary); margin: 0 0 8px 0;",
                        "Axum Admin"
                    }
                    p {
                        style: "font-size: 14px; color: var(--el-text-color-secondary); margin: 0;",
                        "{t(TKey::AdminSystem)}"
                    }
                }

                // 错误提示
                if let Some(msg) = error_msg() {
                    div {
                        style: "background: var(--el-color-danger-light-9); color: var(--el-color-danger); border: 1px solid var(--el-color-danger-light-7); border-radius: 4px; padding: 10px 16px; margin-bottom: 20px; font-size: 14px;",
                        "{msg}"
                    }
                }

                    // 用户名输入
                    div {
                        style: "margin-bottom: 20px;",
                        label {
                            style: "display: block; font-size: 14px; color: var(--el-text-color-regular); margin-bottom: 8px;",
                            "{t(TKey::Username)}"
                        }
                        Input {
                            value: Some(username()),
                            placeholder: Some(t(TKey::UsernamePlaceholder)),
                        size: InputSize::Large,
                        on_change: move |e: Event<FormData>| {
                            username.set(e.data().value());
                        }
                    }
                }

                    // 密码输入
                    div {
                        style: "margin-bottom: 24px;",
                        label {
                            style: "display: block; font-size: 14px; color: var(--el-text-color-regular); margin-bottom: 8px;",
                            "{t(TKey::Password)}"
                        }
                        Input {
                            value: Some(password()),
                            input_type: InputType::Password,
                            placeholder: Some(t(TKey::PasswordPlaceholder)),
                        size: InputSize::Large,
                        on_change: move |e: Event<FormData>| {
                            password.set(e.data().value());
                        }
                    }
                }

                // 滑块验证码
                SliderCaptcha {
                    placeholder: t(TKey::SliderVerify),
                    success_text: t(TKey::SliderVerified),
                    on_success: move |_| {
                        captcha_verified.set(true);
                    }
                }

                // 登录按钮
                Button {
                    variant: ButtonVariant::Primary,
                    size: Some(ButtonSize::Large),
                    disabled: loading() || !captcha_verified(),
                    style: Some("width: 100%;".to_string()),
                    on_click: do_login,
                    if loading() { "{t(TKey::LoggingIn)}" } else { "{t(TKey::Login)}" }
                }

                // 底部信息 + 语言切换
                div {
                    style: "text-align: center; margin-top: 24px; font-size: 12px; color: var(--el-text-color-placeholder); display: flex; flex-direction: column; align-items: center; gap: 12px;",
                    "Powered by Axum + Dioxus"
                    button {
                        style: "padding: 4px 12px; font-size: 12px; border: 1px solid var(--el-border-color); border-radius: 4px; cursor: pointer; background: transparent; color: var(--el-text-color-secondary);",
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
            }
        }
    }
}
