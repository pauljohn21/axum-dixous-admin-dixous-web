use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::api;
use crate::Route;

/// 登录页面
#[component]
pub fn Login() -> Element {
    let mut username = use_signal(|| "admin".to_string());
    let mut password = use_signal(|| String::new());
    let mut error_msg = use_signal(|| None::<String>);
    let mut loading = use_signal(|| false);
    let navigator = navigator();

    let do_login = move |_| {
        let username_val = username();
        let password_val = password();
        if username_val.is_empty() || password_val.is_empty() {
            error_msg.set(Some("用户名和密码不能为空".to_string()));
            return;
        }

        loading.set(true);
        error_msg.set(None);

        spawn(async move {
            match api::auth::login(username_val, password_val).await {
                Ok(resp) => {
                    crate::storage::set_token(&resp.token);
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
                style: "background: white; border-radius: 12px; padding: 40px; width: 400px; box-shadow: 0 20px 60px rgba(0,0,0,0.3);",

                // Logo
                div {
                    style: "text-align: center; margin-bottom: 30px;",
                    h1 {
                        style: "font-size: 28px; font-weight: 700; color: #303030; margin: 0 0 8px 0;",
                        "Axum Admin"
                    }
                    p {
                        style: "font-size: 14px; color: #909399; margin: 0;",
                        "后台管理系统"
                    }
                }

                // 错误提示
                if let Some(msg) = error_msg() {
                    div {
                        style: "background: #fef0f0; color: #f56c6c; border: 1px solid #fde2e2; border-radius: 4px; padding: 10px 16px; margin-bottom: 20px; font-size: 14px;",
                        "{msg}"
                    }
                }

                // 用户名输入
                div {
                    style: "margin-bottom: 20px;",
                    label {
                        style: "display: block; font-size: 14px; color: #606266; margin-bottom: 8px;",
                        "用户名"
                    }
                    Input {
                        value: Some(username()),
                        placeholder: Some("请输入用户名".to_string()),
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
                        style: "display: block; font-size: 14px; color: #606266; margin-bottom: 8px;",
                        "密码"
                    }
                    Input {
                        value: Some(password()),
                        input_type: InputType::Password,
                        placeholder: Some("请输入密码".to_string()),
                        size: InputSize::Large,
                        on_change: move |e: Event<FormData>| {
                            password.set(e.data().value());
                        }
                    }
                }

                // 登录按钮
                Button {
                    variant: ButtonVariant::Primary,
                    size: Some(ButtonSize::Large),
                    disabled: loading(),
                    style: Some("width: 100%;".to_string()),
                    on_click: do_login,
                    if loading() { "登录中..." } else { "登 录" }
                }

                // 底部信息
                div {
                    style: "text-align: center; margin-top: 24px; font-size: 12px; color: #c0c4cc;",
                    "Powered by Axum + Dioxus"
                }
            }
        }
    }
}
