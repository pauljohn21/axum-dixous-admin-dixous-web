use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::api;
use crate::i18n::{t, t_paging, TKey};
use crate::models::user::{SysUser, SysUserInsertDTO, SysUserUpdateDTO};

/// 用户管理页面
#[component]
pub fn UserManage() -> Element {
    let mut users = use_signal(Vec::new);
    let mut total = use_signal(|| 0u64);
    let mut current_page = use_signal(|| 1u32);
    let page_size = 10u32;
    let mut keyword = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);

    // 弹窗状态
    let mut dialog_visible = use_signal(|| false);
    let mut is_edit = use_signal(|| false);
    let mut edit_id = use_signal(|| 0i32);
    let mut form_username = use_signal(String::new);
    let mut form_password = use_signal(String::new);
    let mut form_nick_name = use_signal(String::new);
    let mut form_phone = use_signal(String::new);
    let mut form_email = use_signal(String::new);

    let mut fetch_users = move || {
        loading.set(true);
        error_msg.set(None);
        let kw = keyword();
        spawn(async move {
            match api::user::list(Some(current_page()), Some(page_size), Some(&kw)).await {
                Ok(resp) => {
                    users.set(resp.list);
                    total.set(resp.total);
                }
                Err(e) => {
                    error_msg.set(Some(e));
                }
            }
            loading.set(false);
        });
    };

    // 初始加载
    use_effect(move || {
        fetch_users();
    });

    let on_search = move |_| {
        current_page.set(1);
        fetch_users();
    };

    let on_add = move |_| {
        is_edit.set(false);
        form_username.set(String::new());
        form_password.set(String::new());
        form_nick_name.set(String::new());
        form_phone.set(String::new());
        form_email.set(String::new());
        dialog_visible.set(true);
    };

    let mut on_edit = move |user: SysUser| {
        is_edit.set(true);
        edit_id.set(user.id);
        form_username.set(user.username.clone().unwrap_or_default());
        form_password.set(String::new());
        form_nick_name.set(user.nick_name.clone().unwrap_or_default());
        form_phone.set(user.phone.clone().unwrap_or_default());
        form_email.set(user.email.clone().unwrap_or_default());
        dialog_visible.set(true);
    };

    let mut delete_target = use_signal(|| None::<i32>);

    let mut on_delete = move |id: i32| {
        delete_target.set(Some(id));
    };

    let confirm_delete = move |_| {
        if let Some(id) = delete_target() {
            spawn(async move {
                match api::user::delete_user(id).await {
                    Ok(_) => { fetch_users(); }
                    Err(e) => { error_msg.set(Some(e)); }
                }
            });
        }
        delete_target.set(None);
    };

    let on_submit = move |_| {
        if is_edit() {
            let dto = SysUserUpdateDTO {
                nick_name: if form_nick_name().is_empty() { None } else { Some(form_nick_name()) },
                phone: if form_phone().is_empty() { None } else { Some(form_phone()) },
                email: if form_email().is_empty() { None } else { Some(form_email()) },
                header_img: None,
                side_mode: None,
                enable: None,
            };
            let id = edit_id();
            spawn(async move {
                match api::user::update(id, dto).await {
                    Ok(_) => {
                        dialog_visible.set(false);
                        fetch_users();
                    }
                    Err(e) => { error_msg.set(Some(e)); }
                }
            });
        } else {
            let dto = SysUserInsertDTO {
                username: form_username(),
                password: form_password(),
                nick_name: if form_nick_name().is_empty() { None } else { Some(form_nick_name()) },
                phone: if form_phone().is_empty() { None } else { Some(form_phone()) },
                email: if form_email().is_empty() { None } else { Some(form_email()) },
                role_id: None,
            };
            spawn(async move {
                match api::user::create(dto).await {
                    Ok(_) => {
                        dialog_visible.set(false);
                        fetch_users();
                    }
                    Err(e) => { error_msg.set(Some(e)); }
                }
            });
        }
    };

    let total_pages = total().div_ceil(page_size as u64);

    rsx! {
        div {
            // 页面标题
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",
                h2 { style: "font-size: 20px; font-weight: 600; color: var(--el-text-color-primary); margin: 0;", "{t(TKey::UserManage)}" }
                Button {
                    variant: ButtonVariant::Primary,
                    on_click: on_add,
                    "{t(TKey::AddUser)}"
                }
            }

            // 错误提示
            if let Some(msg) = error_msg() {
                div {
                    style: "background: var(--el-color-danger-light-9); color: var(--el-color-danger); border-radius: 4px; padding: 10px 16px; margin-bottom: 16px; font-size: 14px;",
                    "{msg}"
                }
            }

            // 搜索栏
            div {
                style: "display: flex; gap: 12px; margin-bottom: 20px; background: var(--el-bg-color); padding: 16px; border-radius: 8px; box-shadow: var(--el-box-shadow-light);",
                div {
                    style: "flex: 1; max-width: 300px;",
                    Input {
                        value: Some(keyword()),
                        placeholder: Some(t(TKey::SearchUserPlaceholder)),
                        on_change: move |e: Event<FormData>| {
                            keyword.set(e.data().value());
                        }
                    }
                }
                Button {
                    variant: ButtonVariant::Primary,
                    on_click: on_search,
                    "{t(TKey::Search)}"
                }
            }

            // 数据表格
            div {
                style: "background: var(--el-bg-color); border-radius: 8px; box-shadow: var(--el-box-shadow-light); overflow: hidden;",
                table {
                    style: "width: 100%; border-collapse: collapse;",
                    thead {
                        tr {
                            th { style: th_style(), "ID" }
                            th { style: th_style(), "{t(TKey::Username)}" }
                            th { style: th_style(), "{t(TKey::Nickname)}" }
                            th { style: th_style(), "{t(TKey::Phone)}" }
                            th { style: th_style(), "{t(TKey::Email)}" }
                            th { style: th_style(), "{t(TKey::Status)}" }
                            th { style: th_style(), "{t(TKey::Action)}" }
                        }
                    }
                    tbody {
                        if loading() {
                            tr {
                                td {
                                    colspan: "7",
                                    style: "text-align: center; padding: 40px; color: var(--el-text-color-secondary);",
                                    "{t(TKey::Loading)}"
                                }
                            }
                        } else if users().is_empty() {
                            tr {
                                td {
                                    colspan: "7",
                                    style: "text-align: center; padding: 40px; color: var(--el-text-color-secondary);",
                                    "{t(TKey::NoData)}"
                                }
                            }
                        } else {
                            for user in users() {
                                tr {
                                    style: "border-bottom: 1px solid var(--el-border-color-lighter);",
                                    td { style: td_style(), "{user.id}" }
                                    td { style: td_style(), "{user.username.clone().unwrap_or_default()}" }
                                    td { style: td_style(), "{user.nick_name.clone().unwrap_or_default()}" }
                                    td { style: td_style(), "{user.phone.clone().unwrap_or_default()}" }
                                    td { style: td_style(), "{user.email.clone().unwrap_or_default()}" }
                                    td {
                                        style: td_style(),
                                        if user.enable.unwrap_or(true) {
                                            span {
                                                style: "display: inline-block; padding: 2px 8px; background: var(--el-color-success-light-9); color: var(--el-color-success); border-radius: 4px; font-size: 12px;",
                                                "{t(TKey::Enabled)}"
                                            }
                                        } else {
                                            span {
                                                style: "display: inline-block; padding: 2px 8px; background: var(--el-color-danger-light-9); color: var(--el-color-danger); border-radius: 4px; font-size: 12px;",
                                                "{t(TKey::Disabled)}"
                                            }
                                        }
                                    }
                                    td {
                                        style: td_style(),
                                        div {
                                            style: "display: flex; gap: 8px;",
                                            Button {
                                                variant: ButtonVariant::Primary,
                                                size: Some(ButtonSize::Small),
                                                on_click: {
                                                    let u = user.clone();
                                                    move |_| on_edit(u.clone())
                                                },
                                                "{t(TKey::Edit)}"
                                            }
                                            Button {
                                                variant: ButtonVariant::Danger,
                                                size: Some(ButtonSize::Small),
                                                on_click: move |_| on_delete(user.id),
                                                "{t(TKey::Delete)}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // 分页
                div {
                    style: "display: flex; justify-content: space-between; align-items: center; padding: 16px 20px; border-top: 1px solid var(--el-border-color-lighter);",
                    span {
                        style: "font-size: 14px; color: var(--el-text-color-secondary);",
                        "{t_paging(total(), current_page(), total_pages)}"
                    }
                    div {
                        style: "display: flex; gap: 8px;",
                        Button {
                            variant: ButtonVariant::Default,
                            size: Some(ButtonSize::Small),
                            disabled: current_page() <= 1,
                            on_click: move |_| {
                                current_page.set(current_page() - 1);
                                fetch_users();
                            },
                            "{t(TKey::PrevPage)}"
                        }
                        Button {
                            variant: ButtonVariant::Default,
                            size: Some(ButtonSize::Small),
                            disabled: current_page() >= total_pages as u32,
                            on_click: move |_| {
                                current_page.set(current_page() + 1);
                                fetch_users();
                            },
                            "{t(TKey::NextPage)}"
                        }
                    }
                }
            }

            // 删除确认弹窗
            if let Some(_) = delete_target() {
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: var(--el-overlay-color); z-index: 2000; display: flex; align-items: center; justify-content: center;",
                    onclick: move |_| { delete_target.set(None); },
                    div {
                        style: "background: var(--el-bg-color-overlay); border-radius: 8px; padding: 24px; width: 400px;",
                        onclick: move |e: MouseEvent| { e.stop_propagation(); },
                        h3 { style: "font-size: 16px; font-weight: 600; color: var(--el-text-color-primary); margin: 0 0 16px 0;", "{t(TKey::Delete)}" }
                        p { style: "font-size: 14px; color: var(--el-text-color-regular); margin: 0 0 24px 0;", "{t(TKey::ConfirmDelete)}" }
                        div {
                            style: "display: flex; justify-content: flex-end; gap: 12px;",
                            Button { variant: ButtonVariant::Default, on_click: move |_| { delete_target.set(None); }, "{t(TKey::Cancel)}" }
                            Button { variant: ButtonVariant::Danger, on_click: confirm_delete, "{t(TKey::Confirm)}" }
                        }
                    }
                }
            }

            // 新增/编辑弹窗
            if dialog_visible() {
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: var(--el-overlay-color); z-index: 2000; display: flex; align-items: center; justify-content: center;",

                    div {
                        style: "background: var(--el-bg-color-overlay); border-radius: 8px; padding: 24px; width: 480px; max-height: 80vh; overflow-y: auto;",
                        onclick: move |e: MouseEvent| {
                            e.stop_propagation();
                        },

                        h3 {
                            style: "font-size: 18px; font-weight: 600; color: var(--el-text-color-primary); margin: 0 0 24px 0;",
                            if is_edit() { "{t(TKey::EditUser)}" } else { "{t(TKey::AddUser)}" }
                        }

                        // 用户名
                        if !is_edit() {
                            div {
                                style: "margin-bottom: 16px;",
                                label { style: label_style(), "{t(TKey::UsernameRequired)}" }
                                Input {
                                    value: Some(form_username()),
                                    placeholder: Some(t(TKey::UsernamePlaceholder)),
                                    on_change: move |e: Event<FormData>| {
                                        form_username.set(e.data().value());
                                    }
                                }
                            }
                            // 密码
                            div {
                                style: "margin-bottom: 16px;",
                                label { style: label_style(), "{t(TKey::PasswordRequired)}" }
                                Input {
                                    value: Some(form_password()),
                                    input_type: InputType::Password,
                                    placeholder: Some(t(TKey::PasswordPlaceholder)),
                                    on_change: move |e: Event<FormData>| {
                                        form_password.set(e.data().value());
                                    }
                                }
                            }
                        }

                        // 昵称
                        div {
                            style: "margin-bottom: 16px;",
                            label { style: label_style(), "{t(TKey::Nickname)}" }
                            Input {
                                value: Some(form_nick_name()),
                                placeholder: Some(t(TKey::NicknamePlaceholder)),
                                on_change: move |e: Event<FormData>| {
                                    form_nick_name.set(e.data().value());
                                }
                            }
                        }
                        // 手机号
                        div {
                            style: "margin-bottom: 16px;",
                            label { style: label_style(), "{t(TKey::Phone)}" }
                            Input {
                                value: Some(form_phone()),
                                placeholder: Some(t(TKey::PhonePlaceholder)),
                                on_change: move |e: Event<FormData>| {
                                    form_phone.set(e.data().value());
                                }
                            }
                        }
                        // 邮箱
                        div {
                            style: "margin-bottom: 24px;",
                            label { style: label_style(), "{t(TKey::Email)}" }
                            Input {
                                value: Some(form_email()),
                                placeholder: Some(t(TKey::EmailPlaceholder)),
                                on_change: move |e: Event<FormData>| {
                                    form_email.set(e.data().value());
                                }
                            }
                        }

                        // 按钮区
                        div {
                            style: "display: flex; justify-content: flex-end; gap: 12px;",
                            Button {
                                variant: ButtonVariant::Default,
                                on_click: move |_| {
                                    dialog_visible.set(false);
                                },
                                "{t(TKey::Cancel)}"
                            }
                            Button {
                                variant: ButtonVariant::Primary,
                                on_click: on_submit,
                                "{t(TKey::Confirm)}"
                            }
                        }
                    }
                }
            }
        }
    }
}

fn th_style() -> String {
    "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: var(--el-text-color-secondary); background: var(--el-fill-color-lighter); border-bottom: 1px solid var(--el-border-color-lighter);".to_string()
}

fn td_style() -> String {
    "padding: 12px 16px; font-size: 14px; color: var(--el-text-color-regular);".to_string()
}

fn label_style() -> String {
    "display: block; font-size: 14px; color: var(--el-text-color-regular); margin-bottom: 8px;".to_string()
}
