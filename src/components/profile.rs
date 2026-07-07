use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::api;
use crate::i18n::{t, TKey};
use crate::models::user::SysUser;

/// 个人信息页面
#[component]
pub fn Profile() -> Element {
    let mut user = use_signal(|| SysUser {
        id: 0,
        username: None,
        nick_name: None,
        phone: None,
        email: None,
        header_img: None,
        side_mode: None,
        enable: None,
        created_at: None,
    });
    let mut old_password = use_signal(String::new);
    let mut new_password = use_signal(String::new);
    let mut confirm_password = use_signal(String::new);
    let mut error_msg = use_signal(|| None::<String>);
    let mut success_msg = use_signal(|| None::<String>);

    // 获取用户信息
    use_effect(move || {
        spawn(async move {
            // 先获取用户信息接口拿到 username，再通过 list 查找
            if let Ok(info) = api::auth::get_user_info().await {
                // 通过 username 查询用户详情
                if let Ok(resp) = api::user::list(Some(1), Some(1), Some(&info.username)).await {
                    if let Some(u) = resp.list.into_iter().next() {
                        user.set(u);
                    }
                }
            }
        });
    });

    let on_change_password = move |_| {
        error_msg.set(None);
        success_msg.set(None);

        if new_password().is_empty() || old_password().is_empty() {
            error_msg.set(Some(t(TKey::UsernamePasswordRequired)));
            return;
        }
        if new_password() != confirm_password() {
            error_msg.set(Some(t(TKey::PasswordMismatch)));
            return;
        }

        let old_pw = old_password();
        let new_pw = new_password();
        spawn(async move {
            match api::user::change_password(old_pw, new_pw).await {
                Ok(_) => {
                    success_msg.set(Some(t(TKey::PasswordChanged)));
                    old_password.set(String::new());
                    new_password.set(String::new());
                    confirm_password.set(String::new());
                }
                Err(e) => {
                    error_msg.set(Some(e));
                }
            }
        });
    };

    let u = user();

    rsx! {
        div {
            style: "padding: 20px;",

            h2 {
                style: "margin: 0 0 20px 0; font-size: 24px; font-weight: 500; color: var(--el-text-color-primary);",
                "{t(TKey::Profile)}"
            }

            // 基本信息卡片
            div {
                style: "background: var(--el-bg-color); border-radius: 4px; box-shadow: var(--el-box-shadow); margin-bottom: 20px;",

                div {
                    style: "padding: 16px 20px; border-bottom: 1px solid var(--el-border-color-lighter); font-size: 16px; font-weight: 500; color: var(--el-text-color-primary);",
                    "{t(TKey::BasicInfo)}"
                }

                div {
                    style: "padding: 20px; display: flex; flex-direction: column; gap: 16px;",

                    // 用户名
                    div {
                        style: "display: flex; align-items: center; gap: 16px;",
                        label { style: "width: 100px; color: var(--el-text-color-regular); font-size: 14px;", "{t(TKey::Username)}:" }
                        span { style: "color: var(--el-text-color-primary); font-size: 14px;", "{u.username.clone().unwrap_or_default()}" }
                    }

                    // 昵称
                    div {
                        style: "display: flex; align-items: center; gap: 16px;",
                        label { style: "width: 100px; color: var(--el-text-color-regular); font-size: 14px;", "{t(TKey::Nickname)}:" }
                        span { style: "color: var(--el-text-color-primary); font-size: 14px;", "{u.nick_name.clone().unwrap_or_default()}" }
                    }

                    // 手机号
                    div {
                        style: "display: flex; align-items: center; gap: 16px;",
                        label { style: "width: 100px; color: var(--el-text-color-regular); font-size: 14px;", "{t(TKey::Phone)}:" }
                        span { style: "color: var(--el-text-color-primary); font-size: 14px;", "{u.phone.clone().unwrap_or_default()}" }
                    }

                    // 邮箱
                    div {
                        style: "display: flex; align-items: center; gap: 16px;",
                        label { style: "width: 100px; color: var(--el-text-color-regular); font-size: 14px;", "{t(TKey::Email)}:" }
                        span { style: "color: var(--el-text-color-primary); font-size: 14px;", "{u.email.clone().unwrap_or_default()}" }
                    }

                    // 头像
                    div {
                        style: "display: flex; align-items: center; gap: 16px;",
                        label { style: "width: 100px; color: var(--el-text-color-regular); font-size: 14px;", "{t(TKey::Avatar)}:" }
                        div {
                            style: "width: 64px; height: 64px; border-radius: 50%; background: var(--el-color-primary); display: flex; align-items: center; justify-content: center; color: white; font-size: 24px;",
                            "{u.username.clone().unwrap_or_else(|| 'U'.to_string()).chars().next().unwrap_or('U')}"
                        }
                    }
                }
            }

            // 修改密码卡片
            div {
                style: "background: var(--el-bg-color); border-radius: 4px; box-shadow: var(--el-box-shadow);",

                div {
                    style: "padding: 16px 20px; border-bottom: 1px solid var(--el-border-color-lighter); font-size: 16px; font-weight: 500; color: var(--el-text-color-primary);",
                    "{t(TKey::ChangePassword)}"
                }

                div {
                    style: "padding: 20px;",

                    if let Some(msg) = error_msg() {
                        div {
                            style: "background: var(--el-color-danger-light-9); color: var(--el-color-danger); border-radius: 4px; padding: 10px 16px; margin-bottom: 16px; font-size: 14px;",
                            "{msg}"
                        }
                    }
                    if let Some(msg) = success_msg() {
                        div {
                            style: "background: var(--el-color-success-light-9); color: var(--el-color-success); border-radius: 4px; padding: 10px 16px; margin-bottom: 16px; font-size: 14px;",
                            "{msg}"
                        }
                    }

                    div {
                        style: "margin-bottom: 16px;",
                        label { style: "display: block; font-size: 14px; color: var(--el-text-color-regular); margin-bottom: 8px;", "{t(TKey::OldPassword)}" }
                        Input {
                            value: Some(old_password()),
                            input_type: InputType::Password,
                            placeholder: Some(t(TKey::OldPassword)),
                            on_change: move |e: Event<FormData>| { old_password.set(e.data().value()); }
                        }
                    }
                    div {
                        style: "margin-bottom: 16px;",
                        label { style: "display: block; font-size: 14px; color: var(--el-text-color-regular); margin-bottom: 8px;", "{t(TKey::NewPassword)}" }
                        Input {
                            value: Some(new_password()),
                            input_type: InputType::Password,
                            placeholder: Some(t(TKey::NewPassword)),
                            on_change: move |e: Event<FormData>| { new_password.set(e.data().value()); }
                        }
                    }
                    div {
                        style: "margin-bottom: 24px;",
                        label { style: "display: block; font-size: 14px; color: var(--el-text-color-regular); margin-bottom: 8px;", "{t(TKey::ConfirmPassword)}" }
                        Input {
                            value: Some(confirm_password()),
                            input_type: InputType::Password,
                            placeholder: Some(t(TKey::ConfirmPassword)),
                            on_change: move |e: Event<FormData>| { confirm_password.set(e.data().value()); }
                        }
                    }

                    Button {
                        variant: ButtonVariant::Primary,
                        on_click: on_change_password,
                        "{t(TKey::ChangePassword)}"
                    }
                }
            }
        }
    }
}
