//! 代码生成器历史记录页面
//!
//! 功能:
//! - 分页列表查看生成历史
//! - 查看配置 (JSON)
//! - 回滚 (可选删除表)
//! - 删除历史记录

use dioxus::prelude::*;

use crate::api;
use crate::i18n::{t, TKey};
use crate::models::generator_history::GeneratorHistory;

/// 历史记录页面
#[component]
pub fn GeneratorHistoryPage() -> Element {
    let mut list_data = use_signal(Vec::new);
    let mut total = use_signal(|| 0u64);
    let mut current_page = use_signal(|| 1u32);
    let page_size = 10u32;
    let mut keyword = use_signal(String::new);
    let mut loading = use_signal(|| false);
    let mut error_msg = use_signal(|| None::<String>);

    // 查看配置对话框
    let mut show_yaml = use_signal(|| false);
    let mut yaml_text = use_signal(String::new);
    let mut yaml_title = use_signal(String::new);

    // 回滚确认对话框
    let mut show_rollback = use_signal(|| false);
    let mut rollback_id = use_signal(|| 0u64);
    let mut rollback_name = use_signal(String::new);
    let mut delete_table = use_signal(|| false);
    let mut rollback_loading = use_signal(|| false);

    let mut fetch_list = move || {
        loading.set(true);
        error_msg.set(None);
        let kw = keyword();
        spawn(async move {
            match api::generator_history::list(Some(current_page()), Some(page_size), Some(&kw)).await {
                Ok(resp) => {
                    list_data.set(resp.list);
                    total.set(resp.total);
                }
                Err(e) => {
                    error_msg.set(Some(e));
                }
            }
            loading.set(false);
        });
    };

    use_effect(move || {
        fetch_list();
    });

    let on_search = move |_| {
        current_page.set(1);
        fetch_list();
    };

    let on_prev_page = move |_| {
        if current_page() > 1 {
            current_page.set(current_page() - 1);
            fetch_list();
        }
    };

    let on_next_page = move |_| {
        let total_pages = (total() + page_size as u64 - 1) / page_size as u64;
        if (current_page() as u64) < total_pages {
            current_page.set(current_page() + 1);
            fetch_list();
        }
    };

    let mut on_view_yaml = move |item: GeneratorHistory| {
        yaml_title.set(format!("{} ({})", item.module_cn, item.table_name));
        // 尝试格式化 JSON
        let formatted = if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&item.request) {
            serde_json::to_string_pretty(&json_val).unwrap_or(item.request)
        } else {
            item.request
        };
        yaml_text.set(formatted);
        show_yaml.set(true);
    };

    let mut on_rollback = move |item: GeneratorHistory| {
        rollback_id.set(item.id);
        rollback_name.set(format!("{} ({})", item.module_cn, item.table_name));
        delete_table.set(false);
        show_rollback.set(true);
    };

    let on_confirm_rollback = move |_| {
        rollback_loading.set(true);
        let id = rollback_id();
        let del_table = delete_table();
        spawn(async move {
            match api::generator_history::rollback(id, del_table).await {
                Ok(_) => {
                    show_rollback.set(false);
                    fetch_list();
                }
                Err(e) => {
                    error_msg.set(Some(e));
                }
            }
            rollback_loading.set(false);
        });
    };

    let on_delete = move |id: u64| {
        spawn(async move {
            match api::generator_history::delete(id).await {
                Ok(_) => {
                    fetch_list();
                }
                Err(e) => {
                    error_msg.set(Some(e));
                }
            }
        });
    };

    // 样式
    let card_style = "background: var(--el-bg-color); border-radius: 4px; box-shadow: var(--el-box-shadow); margin-bottom: 16px;";
    let btn_primary = "padding: 8px 20px; background: var(--el-color-primary); color: #fff; border: none; border-radius: 4px; font-size: 14px; cursor: pointer;";
    let btn_default = "padding: 8px 20px; background: var(--el-bg-color); color: var(--el-text-color-regular); border: 1px solid var(--el-border-color); border-radius: 4px; font-size: 14px; cursor: pointer;";
    let btn_danger = "padding: 4px 12px; background: var(--el-color-danger); color: #fff; border: none; border-radius: 4px; font-size: 12px; cursor: pointer;";
    let btn_warning = "padding: 4px 12px; background: var(--el-color-warning); color: #fff; border: none; border-radius: 4px; font-size: 12px; cursor: pointer;";
    let btn_small = "padding: 4px 12px; background: var(--el-color-primary); color: #fff; border: none; border-radius: 4px; font-size: 12px; cursor: pointer;";
    let table_style = "width: 100%; border-collapse: collapse; font-size: 13px;";
    let th_style = "padding: 10px 8px; text-align: left; border-bottom: 2px solid var(--el-border-color); color: var(--el-text-color-secondary); font-weight: 500; white-space: nowrap;";
    let td_style = "padding: 8px; border-bottom: 1px solid var(--el-border-color-lighter); color: var(--el-text-color-regular);";
    let input_style = "flex: 1; padding: 8px 12px; border: 1px solid var(--el-border-color); border-radius: 4px; font-size: 14px; color: var(--el-text-color-primary); background: var(--el-bg-color);";

    let total_pages = (total() + page_size as u64 - 1) / page_size as u64;

    rsx! {
        div {
            style: "padding: 20px;",

            h2 {
                style: "margin: 0 0 20px 0; font-size: 24px; font-weight: 500; color: var(--el-text-color-primary);",
                "生成历史"
            }

            // 错误提示
            if let Some(msg) = error_msg() {
                div {
                    style: "padding: 10px 16px; margin-bottom: 16px; background: var(--el-color-danger-light-9); border: 1px solid var(--el-color-danger-light-5); border-radius: 4px; color: var(--el-color-danger); font-size: 14px;",
                    {msg}
                }
            }

            // 搜索栏
            div {
                style: "{card_style}",
                div {
                    style: "padding: 16px 20px; display: flex; gap: 12px; align-items: center;",
                    input {
                        style: "{input_style}",
                        r#type: "text",
                        placeholder: "搜索表名/资源名/模块名",
                        value: "{keyword()}",
                        oninput: move |evt| { keyword.set(evt.value()); }
                    }
                    button {
                        style: "{btn_primary}",
                        onclick: on_search,
                        "{t(TKey::Search)}"
                    }
                }
            }

            // 列表
            div {
                style: "{card_style}",

                if loading() {
                    div {
                        style: "padding: 40px; text-align: center; color: var(--el-text-color-secondary);",
                        "{t(TKey::Loading)}"
                    }
                } else if list_data().is_empty() {
                    div {
                        style: "padding: 40px; text-align: center; color: var(--el-text-color-secondary);",
                        "{t(TKey::NoData)}"
                    }
                } else {
                    div {
                        style: "overflow-x: auto;",
                        table {
                            style: "{table_style}",
                            thead {
                                tr {
                                    th { style: "{th_style}", "ID" }
                                    th { style: "{th_style}", "表名" }
                                    th { style: "{th_style}", "资源名" }
                                    th { style: "{th_style}", "模块名" }
                                    th { style: "{th_style}", "状态" }
                                    th { style: "{th_style}", "创建时间" }
                                    th { style: "{th_style}", "操作" }
                                }
                            }
                            tbody {
                                {list_data().into_iter().map(|item| {
                                    let item_view = item.clone();
                                    let item_rollback = item.clone();
                                    let item_id = item.id;
                                    let item_flag = item.flag;
                                    rsx! {
                                        tr {
                                            td { style: "{td_style}", "{item.id}" }
                                            td { style: "{td_style}", "{item.table_name}" }
                                            td { style: "{td_style}", "{item.resource}" }
                                            td { style: "{td_style}", "{item.module_cn}" }
                                            td { style: "{td_style}",
                                                if item_flag == 0 {
                                                    span {
                                                        style: "color: var(--el-color-success); font-weight: 500;",
                                                        "正常"
                                                    }
                                                } else {
                                                    span {
                                                        style: "color: var(--el-text-color-secondary);",
                                                        "已回滚"
                                                    }
                                                }
                                            }
                                            td { style: "{td_style}", "{item.created_at.as_deref().unwrap_or(\"—\")}" }
                                            td { style: "{td_style}",
                                                button {
                                                    style: "{btn_small}",
                                                    onclick: move |_| {
                                                        on_view_yaml(item_view.clone());
                                                    },
                                                    "查看配置"
                                                }
                                                if item_flag == 0 {
                                                    button {
                                                        style: "{btn_warning} margin-left: 4px;",
                                                        onclick: move |_| {
                                                            on_rollback(item_rollback.clone());
                                                        },
                                                        "回滚"
                                                    }
                                                }
                                                button {
                                                    style: "{btn_danger} margin-left: 4px;",
                                                    onclick: move |_| {
                                                        on_delete(item_id);
                                                    },
                                                    "{t(TKey::Delete)}"
                                                }
                                            }
                                        }
                                    }
                                })}
                            }
                        }
                    }

                    // 分页
                    div {
                        style: "padding: 16px 20px; display: flex; justify-content: space-between; align-items: center; border-top: 1px solid var(--el-border-color-lighter);",
                        span {
                            style: "font-size: 13px; color: var(--el-text-color-secondary);",
                            "共 {total()} 条记录，第 {current_page()}/{total_pages} 页"
                        }
                        div {
                            style: "display: flex; gap: 8px;",
                            button {
                                style: "{btn_default}",
                                disabled: "{current_page() <= 1}",
                                onclick: on_prev_page,
                                "{t(TKey::PrevPage)}"
                            }
                            button {
                                style: "{btn_default}",
                                disabled: "{(current_page() as u64) >= total_pages}",
                                onclick: on_next_page,
                                "{t(TKey::NextPage)}"
                            }
                        }
                    }
                }
            }

            // YAML 查看对话框
            if show_yaml() {
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); z-index: 2000; display: flex; align-items: center; justify-content: center;",
                    onclick: move |_| { show_yaml.set(false); },

                    div {
                        style: "background: var(--el-bg-color); border-radius: 8px; width: 80%; max-width: 800px; max-height: 80vh; display: flex; flex-direction: column;",
                        onclick: move |evt| { evt.stop_propagation(); },

                        div {
                            style: "padding: 16px 20px; border-bottom: 1px solid var(--el-border-color-lighter); display: flex; justify-content: space-between; align-items: center;",
                            span {
                                style: "font-size: 16px; font-weight: 500; color: var(--el-text-color-primary);",
                                "{yaml_title()}"
                            }
                            button {
                                style: "{btn_default}",
                                onclick: move |_| { show_yaml.set(false); },
                                "关闭"
                            }
                        }

                        div {
                            style: "padding: 16px 20px; overflow-y: auto; flex: 1;",
                            pre {
                                style: "margin: 0; padding: 16px; background: var(--el-fill-color-darker); border-radius: 4px; font-size: 13px; color: var(--el-text-color-primary); white-space: pre-wrap; word-break: break-all; font-family: 'Menlo', 'Monaco', monospace;",
                                "{yaml_text()}"
                            }
                        }
                    }
                }
            }

            // 回滚确认对话框
            if show_rollback() {
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); z-index: 2000; display: flex; align-items: center; justify-content: center;",
                    onclick: move |_| { show_rollback.set(false); },

                    div {
                        style: "background: var(--el-bg-color); border-radius: 8px; width: 450px; display: flex; flex-direction: column;",
                        onclick: move |evt| { evt.stop_propagation(); },

                        div {
                            style: "padding: 16px 20px; border-bottom: 1px solid var(--el-border-color-lighter); font-size: 16px; font-weight: 500; color: var(--el-text-color-primary);",
                            "确认回滚"
                        }

                        div {
                            style: "padding: 20px;",

                            p {
                                style: "margin: 0 0 16px 0; color: var(--el-text-color-regular); font-size: 14px;",
                                "确定要回滚以下记录吗？"
                            }
                            p {
                                style: "margin: 0 0 16px 0; color: var(--el-color-primary); font-size: 14px; font-weight: 500;",
                                "{rollback_name()}"
                            }

                            label {
                                style: "display: flex; align-items: center; gap: 8px; font-size: 14px; color: var(--el-text-color-regular); cursor: pointer;",
                                input {
                                    r#type: "checkbox",
                                    checked: "{delete_table()}",
                                    onchange: move |evt| { delete_table.set(evt.checked()); }
                                }
                                "同时删除数据库表 (危险操作)"
                            }

                            p {
                                style: "margin: 12px 0 0 0; color: var(--el-color-warning); font-size: 12px;",
                                "⚠️ 回滚后将标记为已回滚状态，回滚后不可恢复"
                            }
                        }

                        div {
                            style: "padding: 16px 20px; border-top: 1px solid var(--el-border-color-lighter); display: flex; justify-content: flex-end; gap: 8px;",
                            button {
                                style: "{btn_default}",
                                disabled: "{rollback_loading()}",
                                onclick: move |_| { show_rollback.set(false); },
                                "{t(TKey::Cancel)}"
                            }
                            button {
                                style: "{btn_danger}",
                                disabled: "{rollback_loading()}",
                                onclick: on_confirm_rollback,
                                if rollback_loading() { "回滚中..." } else { "确认回滚" }
                            }
                        }
                    }
                }
            }
        }
    }
}
