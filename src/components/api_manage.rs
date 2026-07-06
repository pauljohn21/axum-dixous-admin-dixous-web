use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

use crate::api;
use crate::api::sys_api::{SysApi, SysApiInsertDTO, SysApiUpdateDTO};

fn method_color(method: &str) -> String {
    match method {
        "GET" => "#67c23a".to_string(),
        "POST" => "#e6a23c".to_string(),
        "PUT" => "#409eff".to_string(),
        "DELETE" => "#f56c6c".to_string(),
        _ => "#909399".to_string(),
    }
}

/// 预计算的 API 行数据
struct ApiRow {
    id: i32,
    path: String,
    method: String,
    color: String,
    group: String,
    description: String,
    original: SysApi,
}

/// API管理页面
#[component]
pub fn ApiManage() -> Element {
    let mut apis = use_signal(Vec::new);
    let mut total = use_signal(|| 0u64);
    let mut current_page = use_signal(|| 1u32);
    let page_size = 10u32;
    let mut keyword = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);

    let mut dialog_visible = use_signal(|| false);
    let mut is_edit = use_signal(|| false);
    let mut edit_id = use_signal(|| 0i32);
    let mut form_path = use_signal(String::new);
    let mut form_method = use_signal(String::new);
    let mut form_group = use_signal(String::new);
    let mut form_description = use_signal(String::new);

    let mut fetch_apis = move || {
        loading.set(true);
        error_msg.set(None);
        let kw = keyword();
        spawn(async move {
            match api::sys_api::list(Some(current_page()), Some(page_size), Some(&kw)).await {
                Ok(resp) => {
                    apis.set(resp.list);
                    total.set(resp.total);
                }
                Err(e) => { error_msg.set(Some(e)); }
            }
            loading.set(false);
        });
    };

    use_effect(move || { fetch_apis(); });

    let mut on_edit = move |item: SysApi| {
        is_edit.set(true);
        edit_id.set(item.id);
        form_path.set(item.path.unwrap_or_default());
        form_method.set(item.method.unwrap_or_default());
        form_group.set(item.group.unwrap_or_default());
        form_description.set(item.description.unwrap_or_default());
        dialog_visible.set(true);
    };

    let mut on_delete = move |id: i32| {
        spawn(async move {
            match api::sys_api::delete_api(id).await {
                Ok(_) => { fetch_apis(); }
                Err(e) => { error_msg.set(Some(e)); }
            }
        });
    };

    let on_submit = move |_| {
        if is_edit() {
            let dto = SysApiUpdateDTO {
                path: if form_path().is_empty() { None } else { Some(form_path()) },
                method: if form_method().is_empty() { None } else { Some(form_method()) },
                group: if form_group().is_empty() { None } else { Some(form_group()) },
                description: if form_description().is_empty() { None } else { Some(form_description()) },
            };
            let id = edit_id();
            spawn(async move {
                match api::sys_api::update(id, dto).await {
                    Ok(_) => { dialog_visible.set(false); fetch_apis(); }
                    Err(e) => { error_msg.set(Some(e)); }
                }
            });
        } else {
            let dto = SysApiInsertDTO {
                path: form_path(),
                method: form_method(),
                group: if form_group().is_empty() { None } else { Some(form_group()) },
                description: if form_description().is_empty() { None } else { Some(form_description()) },
            };
            spawn(async move {
                match api::sys_api::create(dto).await {
                    Ok(_) => { dialog_visible.set(false); fetch_apis(); }
                    Err(e) => { error_msg.set(Some(e)); }
                }
            });
        }
    };

    let total_pages = (total() + page_size as u64 - 1) / page_size as u64;

    // 预计算表格行数据
    let rows: Vec<ApiRow> = apis().into_iter().map(|item| {
        let method_str = item.method.clone().unwrap_or_default();
        let color = method_color(&method_str);
        ApiRow {
            id: item.id,
            path: item.path.clone().unwrap_or_default(),
            method: method_str,
            color,
            group: item.group.clone().unwrap_or_default(),
            description: item.description.clone().unwrap_or_default(),
            original: item,
        }
    }).collect();

    rsx! {
        div {
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 20px;",
                h2 { style: "font-size: 20px; font-weight: 600; color: #303030; margin: 0;", "API管理" }
                Button { variant: ButtonVariant::Primary, on_click: move |_| {
                    is_edit.set(false);
                    form_path.set(String::new());
                    form_method.set(String::from("GET"));
                    form_group.set(String::new());
                    form_description.set(String::new());
                    dialog_visible.set(true);
                }, "+ 新增API" }
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
                        placeholder: Some("搜索API路径/描述".to_string()),
                        on_change: move |e: Event<FormData>| { keyword.set(e.data().value()); }
                    }
                }
                Button { variant: ButtonVariant::Primary, on_click: move |_| { current_page.set(1); fetch_apis(); }, "搜索" }
            }

            div {
                style: "background: white; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.04); overflow: hidden;",
                table {
                    style: "width: 100%; border-collapse: collapse;",
                    thead {
                        tr {
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "ID" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "路径" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "方法" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "分组" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "描述" }
                            th { style: "padding: 12px 16px; text-align: left; font-size: 14px; font-weight: 600; color: #909399; background: #fafafa; border-bottom: 1px solid #ebeef5;", "操作" }
                        }
                    }
                    tbody {
                        if loading() {
                            tr { td { colspan: "6", style: "text-align: center; padding: 40px; color: #909399;", "加载中..." } }
                        } else if rows.is_empty() {
                            tr { td { colspan: "6", style: "text-align: center; padding: 40px; color: #909399;", "暂无数据" } }
                        } else {
                            for row in rows {
                                tr {
                                    style: "border-bottom: 1px solid #ebeef5;",
                                    td { style: "padding: 12px 16px; font-size: 14px; color: #606266;", "{row.id}" }
                                    td { style: "padding: 12px 16px; font-size: 14px; color: #606266; font-family: monospace;", "{row.path}" }
                                    td {
                                        style: "padding: 12px 16px; font-size: 14px;",
                                        span {
                                            style: "display: inline-block; padding: 2px 8px; border-radius: 4px; font-size: 12px; font-weight: 600; color: {row.color}; background: {row.color}1a;",
                                            "{row.method}"
                                        }
                                    }
                                    td { style: "padding: 12px 16px; font-size: 14px; color: #606266;", "{row.group}" }
                                    td { style: "padding: 12px 16px; font-size: 14px; color: #606266;", "{row.description}" }
                                    td {
                                        style: "padding: 12px 16px;",
                                        div {
                                            style: "display: flex; gap: 8px;",
                                            Button {
                                                variant: ButtonVariant::Primary,
                                                size: Some(ButtonSize::Small),
                                                on_click: move |_| on_edit(row.original.clone()),
                                                "编辑"
                                            }
                                            Button {
                                                variant: ButtonVariant::Danger,
                                                size: Some(ButtonSize::Small),
                                                on_click: move |_| on_delete(row.id),
                                                "删除"
                                            }
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
                        Button { variant: ButtonVariant::Default, size: Some(ButtonSize::Small), disabled: current_page() <= 1, on_click: move |_| { current_page.set(current_page() - 1); fetch_apis(); }, "上一页" }
                        Button { variant: ButtonVariant::Default, size: Some(ButtonSize::Small), disabled: current_page() >= total_pages as u32, on_click: move |_| { current_page.set(current_page() + 1); fetch_apis(); }, "下一页" }
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
                        h3 { style: "font-size: 18px; font-weight: 600; color: #303030; margin: 0 0 24px 0;", if is_edit() { "编辑API" } else { "新增API" } }
                        div {
                            style: "margin-bottom: 16px;",
                            label { style: "display: block; font-size: 14px; color: #606266; margin-bottom: 8px;", "路径 *" }
                            Input { value: Some(form_path()), placeholder: Some("如 /api/user/list".to_string()), on_change: move |e: Event<FormData>| { form_path.set(e.data().value()); } }
                        }
                        div {
                            style: "margin-bottom: 16px;",
                            label { style: "display: block; font-size: 14px; color: #606266; margin-bottom: 8px;", "方法 *" }
                            Input { value: Some(form_method()), placeholder: Some("GET/POST/PUT/DELETE".to_string()), on_change: move |e: Event<FormData>| { form_method.set(e.data().value()); } }
                        }
                        div {
                            style: "margin-bottom: 16px;",
                            label { style: "display: block; font-size: 14px; color: #606266; margin-bottom: 8px;", "分组" }
                            Input { value: Some(form_group()), placeholder: Some("如 用户管理".to_string()), on_change: move |e: Event<FormData>| { form_group.set(e.data().value()); } }
                        }
                        div {
                            style: "margin-bottom: 24px;",
                            label { style: "display: block; font-size: 14px; color: #606266; margin-bottom: 8px;", "描述" }
                            Input { value: Some(form_description()), placeholder: Some("请输入API描述".to_string()), on_change: move |e: Event<FormData>| { form_description.set(e.data().value()); } }
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
