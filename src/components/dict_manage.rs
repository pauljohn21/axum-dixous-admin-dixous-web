use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::api;
use crate::api::dictionary::{SysDictionary, SysDictionaryInsertDTO, SysDictionaryUpdateDTO};

/// 字典管理页面
#[component]
pub fn DictManage() -> Element {
    let mut dicts = use_signal(Vec::new);
    let mut total = use_signal(|| 0u64);
    let mut current_page = use_signal(|| 1u32);
    let page_size = 10u32;
    let mut keyword = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);

    let mut dialog_visible = use_signal(|| false);
    let mut is_edit = use_signal(|| false);
    let mut edit_id = use_signal(|| 0i32);
    let mut form_name = use_signal(String::new);
    let mut form_code = use_signal(String::new);
    let mut form_desc = use_signal(String::new);

    let mut fetch_dicts = move || {
        loading.set(true);
        error_msg.set(None);
        let kw = keyword();
        spawn(async move {
            match api::dictionary::list(Some(current_page()), Some(page_size), Some(&kw)).await {
                Ok(resp) => {
                    dicts.set(resp.list);
                    total.set(resp.total);
                }
                Err(e) => { error_msg.set(Some(e)); }
            }
            loading.set(false);
        });
    };

    use_effect(move || { fetch_dicts(); });

    let on_add = move |_| {
        is_edit.set(false);
        form_name.set(String::new());
        form_code.set(String::new());
        form_desc.set(String::new());
        dialog_visible.set(true);
    };

    let mut on_edit = move |item: SysDictionary| {
        is_edit.set(true);
        edit_id.set(item.id);
        form_name.set(item.name.clone().unwrap_or_default());
        form_code.set(item.code.clone().unwrap_or_default());
        form_desc.set(item.desc.clone().unwrap_or_default());
        dialog_visible.set(true);
    };

    let mut on_delete = move |id: i32| {
        spawn(async move {
            match api::dictionary::delete_dict(id).await {
                Ok(_) => { fetch_dicts(); }
                Err(e) => { error_msg.set(Some(e)); }
            }
        });
    };

    let on_submit = move |_| {
        if is_edit() {
            let dto = SysDictionaryUpdateDTO {
                name: if form_name().is_empty() { None } else { Some(form_name()) },
                code: if form_code().is_empty() { None } else { Some(form_code()) },
                desc: if form_desc().is_empty() { None } else { Some(form_desc()) },
                status: None,
            };
            let id = edit_id();
            spawn(async move {
                match api::dictionary::update(id, dto).await {
                    Ok(_) => { dialog_visible.set(false); fetch_dicts(); }
                    Err(e) => { error_msg.set(Some(e)); }
                }
            });
        } else {
            let dto = SysDictionaryInsertDTO {
                name: form_name(),
                code: form_code(),
                desc: if form_desc().is_empty() { None } else { Some(form_desc()) },
                status: None,
            };
            spawn(async move {
                match api::dictionary::create(dto).await {
                    Ok(_) => { dialog_visible.set(false); fetch_dicts(); }
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
                h2 { style: "font-size: 20px; font-weight: 600; color: #303030; margin: 0;", "字典管理" }
                Button { variant: ButtonVariant::Primary, on_click: on_add, "+ 新增字典" }
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
                        placeholder: Some("搜索字典名称/编码".to_string()),
                        on_change: move |e: Event<FormData>| { keyword.set(e.data().value()); }
                    }
                }
                Button { variant: ButtonVariant::Primary, on_click: move |_| { current_page.set(1); fetch_dicts(); }, "搜索" }
            }

            div {
                style: "background: white; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.04); overflow: hidden;",
                table {
                    style: "width: 100%; border-collapse: collapse;",
                    thead {
                        tr {
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "ID" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "字典名称" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "编码" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "描述" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "操作" }
                        }
                    }
                    tbody {
                        if loading() {
                            tr { td { colspan: "5", style: "text-align: center; padding: 40px; color: #909399;", "加载中..." } }
                        } else if dicts().is_empty() {
                            tr { td { colspan: "5", style: "text-align: center; padding: 40px; color: #909399;", "暂无数据" } }
                        } else {
                            for item in dicts() {
                                tr {
                                    style: "border-bottom: 1px solid #ebeef5;",
                                    td { style: "padding: 12px 16px; font-size: 14px; color: #606266;", "{item.id}" }
                                    td { style: "padding: 12px 16px; font-size: 14px; color: #606266;", "{item.name.clone().unwrap_or_default()}" }
                                    td {
                                        style: "padding: 12px 16px; font-size: 14px; color: #409eff; font-family: monospace;",
                                        "{item.code.clone().unwrap_or_default()}"
                                    }
                                    td { style: "padding: 12px 16px; font-size: 14px; color: #606266;", "{item.desc.clone().unwrap_or_default()}" }
                                    td {
                                        style: "padding: 12px 16px;",
                                        div {
                                            style: "display: flex; gap: 8px;",
                                            Button { variant: ButtonVariant::Primary, size: Some(ButtonSize::Small), on_click: move |_| on_delete(item.id), "编辑" }
                                            Button { variant: ButtonVariant::Danger, size: Some(ButtonSize::Small), on_click: move |_| on_edit(item.clone()), "删除" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                div {
                    style: "display: flex; justify-content: space-between; align-items: center; padding: 16px 20px; border-top: 1px solid #ebeef5;",
                    span { style: "font-size: 14px; color: #909399;", "共 {total()} 条记录，第 {current_page}/{total_pages} 页" }
                    div {
                        style: "display: flex; gap: 8px;",
                        Button { variant: ButtonVariant::Default, size: Some(ButtonSize::Small), disabled: current_page() <= 1, on_click: move |_| { current_page.set(current_page() - 1); fetch_dicts(); }, "上一页" }
                        Button { variant: ButtonVariant::Default, size: Some(ButtonSize::Small), disabled: current_page() >= total_pages as u32, on_click: move |_| { current_page.set(current_page() + 1); fetch_dicts(); }, "下一页" }
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
                        h3 { style: "font-size: 18px; font-weight: 600; color: #303030; margin: 0 0 24px 0;", if is_edit() { "编辑字典" } else { "新增字典" } }
                        div {
                            style: "margin-bottom: 16px;",
                            label { style: "display: block; font-size: 14px; color: #606266; margin-bottom: 8px;", "字典名称 *" }
                            Input { value: Some(form_name()), placeholder: Some("请输入字典名称".to_string()), on_change: move |e: Event<FormData>| { form_name.set(e.data().value()); } }
                        }
                        div {
                            style: "margin-bottom: 16px;",
                            label { style: "display: block; font-size: 14px; color: #606266; margin-bottom: 8px;", "编码 *" }
                            Input { value: Some(form_code()), placeholder: Some("请输入字典编码".to_string()), on_change: move |e: Event<FormData>| { form_code.set(e.data().value()); } }
                        }
                        div {
                            style: "margin-bottom: 24px;",
                            label { style: "display: block; font-size: 14px; color: #606266; margin-bottom: 8px;", "描述" }
                            Input { value: Some(form_desc()), placeholder: Some("请输入描述".to_string()), on_change: move |e: Event<FormData>| { form_desc.set(e.data().value()); } }
                        }
                        div {
                            style: "display: flex; justify-content: flex-end; gap: 12px;",
                            Button { variant: ButtonVariant::Default, on_click: move |_| { dialog_visible.set(false); }, "取消" }
                            Button { variant: ButtonVariant::Primary, on_click: on_submit, "确定" }
                        }
                    }
                }
            }
        }
    }
}
