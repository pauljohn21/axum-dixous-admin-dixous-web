use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::api;
use crate::i18n::{t, t_paging, TKey};
use crate::models::role::{SysRole, SysRoleInsertDTO, SysRoleUpdateDTO};

/// 角色管理页面
#[component]
pub fn RoleManage() -> Element {
    let mut roles = use_signal(Vec::new);
    let mut total = use_signal(|| 0u64);
    let mut current_page = use_signal(|| 1u32);
    let page_size = 10u32;
    let mut keyword = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);

    let mut dialog_visible = use_signal(|| false);
    let mut is_edit = use_signal(|| false);
    let mut edit_id = use_signal(|| 0i32);
    let mut form_en_name = use_signal(String::new);
    let mut form_cn_name = use_signal(String::new);

    let mut fetch_roles = move || {
        loading.set(true);
        error_msg.set(None);
        let kw = keyword();
        spawn(async move {
            match api::role::list(Some(current_page()), Some(page_size), Some(&kw)).await {
                Ok(resp) => {
                    roles.set(resp.list);
                    total.set(resp.total);
                }
                Err(e) => { error_msg.set(Some(e)); }
            }
            loading.set(false);
        });
    };

    use_effect(move || { fetch_roles(); });

    let on_add = move |_| {
        is_edit.set(false);
        form_en_name.set(String::new());
        form_cn_name.set(String::new());
        dialog_visible.set(true);
    };

    let mut on_edit = move |role: SysRole| {
        is_edit.set(true);
        edit_id.set(role.id);
        form_en_name.set(role.en_name.clone().unwrap_or_default());
        form_cn_name.set(role.cn_name.clone().unwrap_or_default());
        dialog_visible.set(true);
    };

    let mut delete_target = use_signal(|| None::<i32>);

    let mut on_delete = move |id: i32| {
        delete_target.set(Some(id));
    };

    let confirm_delete = move |_| {
        if let Some(id) = delete_target() {
            spawn(async move {
                match api::role::delete_role(id).await {
                    Ok(_) => { fetch_roles(); }
                    Err(e) => { error_msg.set(Some(e)); }
                }
            });
        }
        delete_target.set(None);
    };

    let on_submit = move |_| {
        if is_edit() {
            let dto = SysRoleUpdateDTO {
                en_name: if form_en_name().is_empty() { None } else { Some(form_en_name()) },
                cn_name: if form_cn_name().is_empty() { None } else { Some(form_cn_name()) },
                parent_id: None,
            };
            let id = edit_id();
            spawn(async move {
                match api::role::update(id, dto).await {
                    Ok(_) => { dialog_visible.set(false); fetch_roles(); }
                    Err(e) => { error_msg.set(Some(e)); }
                }
            });
        } else {
            let dto = SysRoleInsertDTO {
                en_name: form_en_name(),
                cn_name: form_cn_name(),
                parent_id: None,
            };
            spawn(async move {
                match api::role::create(dto).await {
                    Ok(_) => { dialog_visible.set(false); fetch_roles(); }
                    Err(e) => { error_msg.set(Some(e)); }
                }
            });
        }
    };

    let total_pages = total().div_ceil(page_size as u64);

    rsx! {
        div {
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",
                h2 { style: "font-size: 20px; font-weight: 600; color: var(--el-text-color-primary); margin: 0;", "{t(TKey::RoleManage)}" }
                Button {
                    variant: ButtonVariant::Primary,
                    on_click: on_add,
                    "{t(TKey::AddRole)}"
                }
            }

            if let Some(msg) = error_msg() {
                div {
                    style: "background: var(--el-color-danger-light-9); color: var(--el-color-danger); border-radius: 4px; padding: 10px 16px; margin-bottom: 16px; font-size: 14px;",
                    "{msg}"
                }
            }

            div {
                style: "display: flex; gap: 12px; margin-bottom: 20px; background: var(--el-bg-color); padding: 16px; border-radius: 8px; box-shadow: var(--el-box-shadow-light);",
                div {
                    style: "flex: 1; max-width: 300px;",
                    Input {
                        value: Some(keyword()),
                        placeholder: Some(t(TKey::SearchRolePlaceholder)),
                        on_change: move |e: Event<FormData>| { keyword.set(e.data().value()); }
                    }
                }
                Button {
                    variant: ButtonVariant::Primary,
                    on_click: move |_| { current_page.set(1); fetch_roles(); },
                    "{t(TKey::Search)}"
                }
            }

            div {
                style: "background: var(--el-bg-color); border-radius: 8px; box-shadow: var(--el-box-shadow-light); overflow: hidden;",
                table {
                    style: "width: 100%; border-collapse: collapse;",
                    thead {
                        tr {
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: var(--el-text-color-secondary); background: var(--el-fill-color-lighter); border-bottom: 1px solid var(--el-border-color-lighter);", "ID" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: var(--el-text-color-secondary); background: var(--el-fill-color-lighter); border-bottom: 1px solid var(--el-border-color-lighter);", "{t(TKey::RoleEnName)}" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: var(--el-text-color-secondary); background: var(--el-fill-color-lighter); border-bottom: 1px solid var(--el-border-color-lighter);", "{t(TKey::RoleCnName)}" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: var(--el-text-color-secondary); background: var(--el-fill-color-lighter); border-bottom: 1px solid var(--el-border-color-lighter);", "{t(TKey::Action)}" }
                        }
                    }
                    tbody {
                        if loading() {
                            tr { td { colspan: "4", style: "text-align: center; padding: 40px; color: var(--el-text-color-secondary);", "{t(TKey::Loading)}" } }
                        } else if roles().is_empty() {
                            tr { td { colspan: "4", style: "text-align: center; padding: 40px; color: var(--el-text-color-secondary);", "{t(TKey::NoData)}" } }
                        } else {
                            for role in roles() {
                                tr {
                                    style: "border-bottom: 1px solid var(--el-border-color-lighter);",
                                    td { style: "padding: 12px 16px; font-size: 14px; color: var(--el-text-color-regular);", "{role.id}" }
                                    td { style: "padding: 12px 16px; font-size: 14px; color: var(--el-text-color-regular);", "{role.en_name.clone().unwrap_or_default()}" }
                                    td { style: "padding: 12px 16px; font-size: 14px; color: var(--el-text-color-regular);", "{role.cn_name.clone().unwrap_or_default()}" }
                                    td {
                                        style: "padding: 12px 16px; font-size: 14px;",
                                        div {
                                            style: "display: flex; gap: 8px;",
                                            Button {
                                                variant: ButtonVariant::Primary,
                                                size: Some(ButtonSize::Small),
                                                on_click: {
                                                    let r = role.clone();
                                                    move |_| on_edit(r.clone())
                                                },
                                                "{t(TKey::Edit)}"
                                            }
                                            Button {
                                                variant: ButtonVariant::Danger,
                                                size: Some(ButtonSize::Small),
                                                on_click: move |_| on_delete(role.id),
                                                "{t(TKey::Delete)}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div {
                    style: "display: flex; justify-content: space-between; align-items: center; padding: 16px 20px; border-top: 1px solid var(--el-border-color-lighter);",
                    span { style: "font-size: 14px; color: var(--el-text-color-secondary);", "{t_paging(total(), current_page(), total_pages)}" }
                    div {
                        style: "display: flex; gap: 8px;",
                        Button {
                            variant: ButtonVariant::Default,
                            size: Some(ButtonSize::Small),
                            disabled: current_page() <= 1,
                            on_click: move |_| { current_page.set(current_page() - 1); fetch_roles(); },
                            "{t(TKey::PrevPage)}"
                        }
                        Button {
                            variant: ButtonVariant::Default,
                            size: Some(ButtonSize::Small),
                            disabled: current_page() >= total_pages as u32,
                            on_click: move |_| { current_page.set(current_page() + 1); fetch_roles(); },
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

            // 弹窗
            if dialog_visible() {
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: var(--el-overlay-color); z-index: 2000; display: flex; align-items: center; justify-content: center;",
                    onclick: move |_| { dialog_visible.set(false); },

                    div {
                        style: "background: var(--el-bg-color-overlay); border-radius: 8px; padding: 24px; width: 480px;",
                        onclick: move |e: MouseEvent| { e.stop_propagation(); },

                        h3 { style: "font-size: 18px; font-weight: 600; color: var(--el-text-color-primary); margin: 0 0 24px 0;", if is_edit() { "{t(TKey::EditRole)}" } else { "{t(TKey::AddRole)}" } }

                        div {
                            style: "margin-bottom: 16px;",
                            label { style: "display: block; font-size: 14px; color: var(--el-text-color-regular); margin-bottom: 8px;", "{t(TKey::RoleEnName)} *" }
                            Input {
                                value: Some(form_en_name()),
                                placeholder: Some(t(TKey::RoleEnNamePlaceholder)),
                                on_change: move |e: Event<FormData>| { form_en_name.set(e.data().value()); }
                            }
                        }
                        div {
                            style: "margin-bottom: 24px;",
                            label { style: "display: block; font-size: 14px; color: var(--el-text-color-regular); margin-bottom: 8px;", "{t(TKey::RoleCnName)} *" }
                            Input {
                                value: Some(form_cn_name()),
                                placeholder: Some(t(TKey::RoleCnNamePlaceholder)),
                                on_change: move |e: Event<FormData>| { form_cn_name.set(e.data().value()); }
                            }
                        }

                        div {
                            style: "display: flex; justify-content: flex-end; gap: 12px;",
                            Button { variant: ButtonVariant::Default, on_click: move |_| { dialog_visible.set(false); }, "{t(TKey::Cancel)}" }
                            Button { variant: ButtonVariant::Primary, on_click: on_submit, "{t(TKey::Confirm)}" }
                        }
                    }
                }
            }
        }
    }
}
