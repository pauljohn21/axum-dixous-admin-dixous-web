use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::api;
use crate::i18n::{t, t_paging, TKey};
use crate::models::menu::{SysMenu, SysMenuInsertDTO, SysMenuUpdateDTO};

/// 菜单管理页面
#[component]
pub fn MenuManage() -> Element {
    let mut menus = use_signal(Vec::new);
    let mut total = use_signal(|| 0u64);
    let mut current_page = use_signal(|| 1u32);
    let page_size = 20u32;
    let mut keyword = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);

    let mut dialog_visible = use_signal(|| false);
    let mut is_edit = use_signal(|| false);
    let mut edit_id = use_signal(|| 0i32);
    let mut form_name = use_signal(String::new);
    let mut form_path = use_signal(String::new);
    let mut form_title = use_signal(String::new);
    let mut form_icon = use_signal(String::new);
    let mut form_sort = use_signal(String::new);

    let mut fetch_menus = move || {
        loading.set(true);
        error_msg.set(None);
        let kw = keyword();
        spawn(async move {
            match api::menu::list(Some(current_page()), Some(page_size), Some(&kw)).await {
                Ok(resp) => {
                    menus.set(resp.list);
                    total.set(resp.total);
                }
                Err(e) => { error_msg.set(Some(e)); }
            }
            loading.set(false);
        });
    };

    use_effect(move || { fetch_menus(); });

    let on_add = move |_| {
        is_edit.set(false);
        form_name.set(String::new());
        form_path.set(String::new());
        form_title.set(String::new());
        form_icon.set(String::new());
        form_sort.set(String::new());
        dialog_visible.set(true);
    };

    let mut on_edit = move |menu: SysMenu| {
        is_edit.set(true);
        edit_id.set(menu.id);
        form_name.set(menu.name.clone().unwrap_or_default());
        form_path.set(menu.path.clone().unwrap_or_default());
        form_title.set(menu.title.clone().unwrap_or_default());
        form_icon.set(menu.icon.clone().unwrap_or_default());
        form_sort.set(menu.sort.unwrap_or(0).to_string());
        dialog_visible.set(true);
    };

    let mut on_delete = move |id: i32| {
        spawn(async move {
            match api::menu::delete_menu(id).await {
                Ok(_) => { fetch_menus(); }
                Err(e) => { error_msg.set(Some(e)); }
            }
        });
    };

    let on_submit = move |_| {
        let sort_val = form_sort().parse::<i64>().unwrap_or(0);
        if is_edit() {
            let dto = SysMenuUpdateDTO {
                name: if form_name().is_empty() { None } else { Some(form_name()) },
                parent_id: None,
                path: if form_path().is_empty() { None } else { Some(form_path()) },
                component: None,
                icon: if form_icon().is_empty() { None } else { Some(form_icon()) },
                sort: Some(sort_val),
                hidden: None,
                title: if form_title().is_empty() { None } else { Some(form_title()) },
            };
            let id = edit_id();
            spawn(async move {
                match api::menu::update(id, dto).await {
                    Ok(_) => { dialog_visible.set(false); fetch_menus(); }
                    Err(e) => { error_msg.set(Some(e)); }
                }
            });
        } else {
            let dto = SysMenuInsertDTO {
                name: form_name(),
                parent_id: None,
                path: if form_path().is_empty() { None } else { Some(form_path()) },
                component: None,
                icon: if form_icon().is_empty() { None } else { Some(form_icon()) },
                sort: Some(sort_val),
                hidden: None,
                title: if form_title().is_empty() { None } else { Some(form_title()) },
            };
            spawn(async move {
                match api::menu::create(dto).await {
                    Ok(_) => { dialog_visible.set(false); fetch_menus(); }
                    Err(e) => { error_msg.set(Some(e)); }
                }
            });
        }
    };

    let total_pages = (total() + page_size as u64 - 1) / page_size as u64;

    rsx! {
        div {
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",
                h2 { style: "font-size: 20px; font-weight: 600; color: #303030; margin: 0;", "{t(TKey::MenuManage)}" }
                Button { variant: ButtonVariant::Primary, on_click: on_add, "{t(TKey::AddMenu)}" }
            }

            if let Some(msg) = error_msg() {
                div { style: "background: #fef0f0; color: #f56c6c; border-radius: 4px; padding: 10px 16px; margin-bottom: 16px; font-size: 14px;", "{msg}" }
            }

            div {
                style: "display: flex; gap: 12px; margin-bottom: 20px; background: white; padding: 16px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.04);",
                div {
                    style: "flex: 1; max-width: 300px;",
                    Input {
                        value: Some(keyword()),
                        placeholder: Some(t(TKey::SearchMenuPlaceholder)),
                        on_change: move |e: Event<FormData>| { keyword.set(e.data().value()); }
                    }
                }
                Button { variant: ButtonVariant::Primary, on_click: move |_| { current_page.set(1); fetch_menus(); }, "{t(TKey::Search)}" }
            }

            div {
                style: "background: white; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.04); overflow: hidden;",
                table {
                    style: "width: 100%; border-collapse: collapse;",
                    thead {
                        tr {
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "ID" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "{t(TKey::MenuName)}" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "{t(TKey::MenuTitle)}" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "{t(TKey::MenuPath)}" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "{t(TKey::MenuIcon)}" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "{t(TKey::MenuSort)}" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "{t(TKey::Action)}" }
                        }
                    }
                    tbody {
                        if loading() {
                            tr { td { colspan: "7", style: "text-align: center; padding: 40px; color: #909399;", "{t(TKey::Loading)}" } }
                        } else if menus().is_empty() {
                            tr { td { colspan: "7", style: "text-align: center; padding: 40px; color: #909399;", "{t(TKey::NoData)}" } }
                        } else {
                            for menu in menus() {
                                tr {
                                    style: "border-bottom: 1px solid #ebeef5;",
                                    td { style: "padding: 12px 16px; font-size: 14px; color: #606266;", "{menu.id}" }
                                    td { style: "padding: 12px 16px; font-size: 14px; color: #606266;", "{menu.name.clone().unwrap_or_default()}" }
                                    td { style: "padding: 12px 16px; font-size: 14px; color: #606266;", "{menu.title.clone().unwrap_or_default()}" }
                                    td { style: "padding: 12px 16px; font-size: 14px; color: #606266;", "{menu.path.clone().unwrap_or_default()}" }
                                    td { style: "padding: 12px 16px; font-size: 14px; color: #606266;", "{menu.icon.clone().unwrap_or_default()}" }
                                    td { style: "padding: 12px 16px; font-size: 14px; color: #606266;", "{menu.sort.unwrap_or(0)}" }
                                    td {
                                        style: "padding: 12px 16px;",
                                        div {
                                            style: "display: flex; gap: 8px;",
                                            Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Small), on_click: move |_| on_delete(menu.id), "{t(TKey::Edit)}" }
                                            Button { variant: ButtonVariant::Danger, size: Some(ButtonSize::Small), on_click: move |_| on_edit(menu.clone()), "{t(TKey::Delete)}" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div {
                    style: "display: flex; justify-content: space-between; align-items: center; padding: 16px 20px; border-top: 1px solid #ebeef5;",
                    span { style: "font-size: 14px; color: #909399;", "{t_paging(total(), current_page(), total_pages)}" }
                    div {
                        style: "display: flex; gap: 8px;",
                        Button { variant: ButtonVariant::Default, size: Some(ButtonSize::Small), disabled: current_page() <= 1, on_click: move |_| { current_page.set(current_page() - 1); fetch_menus(); }, "{t(TKey::PrevPage)}" }
                        Button { variant: ButtonVariant::Default, size: Some(ButtonSize::Small), disabled: current_page() >= total_pages as u32, on_click: move |_| { current_page.set(current_page() + 1); fetch_menus(); }, "{t(TKey::NextPage)}" }
                    }
                }
            }

            if dialog_visible() {
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); z-index: 2000; display: flex; align-items: center; justify-content: center;",
                    onclick: move |_| { dialog_visible.set(false); },
                    div {
                        style: "background: white; border-radius: 8px; padding: 24px; width: 480px;",
                        onclick: move |e: MouseEvent| { e.stop_propagation(); },
                        h3 { style: "font-size: 18px; font-weight: 600; color: #303030; margin: 0 0 24px 0;", if is_edit() { "{t(TKey::EditMenu)}" } else { "{t(TKey::AddMenu)}" } }
                        div {
                            style: "margin-bottom: 16px;",
                            label { style: "display: block; font-size: 14px; color: #606266; margin-bottom: 8px;", "{t(TKey::MenuName)} *" }
                            Input { value: Some(form_name()), placeholder: Some(t(TKey::MenuNamePlaceholder)), on_change: move |e: Event<FormData>| { form_name.set(e.data().value()); } }
                        }
                        div {
                            style: "margin-bottom: 16px;",
                            label { style: "display: block; font-size: 14px; color: #606266; margin-bottom: 8px;", "{t(TKey::MenuTitle)}" }
                            Input { value: Some(form_title()), placeholder: Some(t(TKey::MenuTitlePlaceholder)), on_change: move |e: Event<FormData>| { form_title.set(e.data().value()); } }
                        }
                        div {
                            style: "margin-bottom: 16px;",
                            label { style: "display: block; font-size: 14px; color: #606266; margin-bottom: 8px;", "{t(TKey::MenuPath)}" }
                            Input { value: Some(form_path()), placeholder: Some(t(TKey::MenuPathPlaceholder)), on_change: move |e: Event<FormData>| { form_path.set(e.data().value()); } }
                        }
                        div {
                            style: "margin-bottom: 16px;",
                            label { style: "display: block; font-size: 14px; color: #606266; margin-bottom: 8px;", "{t(TKey::MenuIcon)}" }
                            Input { value: Some(form_icon()), placeholder: Some(t(TKey::MenuIconPlaceholder)), on_change: move |e: Event<FormData>| { form_icon.set(e.data().value()); } }
                        }
                        div {
                            style: "margin-bottom: 24px;",
                            label { style: "display: block; font-size: 14px; color: #606266; margin-bottom: 8px;", "{t(TKey::MenuSort)}" }
                            Input { value: Some(form_sort()), placeholder: Some(t(TKey::MenuSortPlaceholder)), on_change: move |e: Event<FormData>| { form_sort.set(e.data().value()); } }
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
