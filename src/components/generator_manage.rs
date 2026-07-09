//! 代码生成器页面
//!
//! 可视化配置模块和字段，保存配置到数据库。
//! 功能：
//! - 模块配置表单 (表名、资源名、模块名、图标等)
//! - 字段列表表格 (增删改、内联编辑)
//! - 保存配置到数据库
//! - 从数据库表结构自动生成配置

use dioxus::prelude::*;
use wasm_bindgen::JsCast;

use crate::api;
use crate::i18n::{t, TKey};
use crate::models::generator::{
    config_to_json, json_to_config, GeneratorConfig, GeneratorField, get_field_templates, FIELD_TYPES, SEARCH_TYPES,
};
use crate::models::generator_history::GeneratedFile;

/// 代码生成器页面
#[component]
pub fn GeneratorManage() -> Element {
    // ===== 模块配置信号 =====
    let mut cfg_table_name = use_signal(String::new);
    let mut cfg_resource = use_signal(String::new);
    let mut cfg_module_cn = use_signal(String::new);
    let mut cfg_icon = use_signal(|| "document".to_string());
    let mut cfg_description = use_signal(String::new);
    let mut cfg_gen_backend = use_signal(|| true);
    let mut cfg_gen_frontend = use_signal(|| true);
    let mut cfg_batch_delete = use_signal(|| true);

    // 字段列表
    let mut fields = use_signal(Vec::<GeneratorField>::new);

    // 状态
    let mut error_msg = use_signal(|| None::<String>);
    let mut success_msg = use_signal(|| None::<String>);
    let mut saving = use_signal(|| false);

    // 编辑字段的索引 (None = 新增)
    let mut edit_index = use_signal(|| None::<usize>);

    // 字段编辑对话框
    let mut show_field_dialog = use_signal(|| false);
    let mut dlg_name = use_signal(String::new);
    let mut dlg_type = use_signal(|| "string".to_string());
    let mut dlg_comment = use_signal(String::new);
    let mut dlg_nullable = use_signal(|| true);
    let mut dlg_search = use_signal(|| false);
    let mut dlg_search_type = use_signal(String::new);
    let mut dlg_require = use_signal(|| false);
    let mut dlg_default_value = use_signal(String::new);
    let mut dlg_form = use_signal(|| true);
    let mut dlg_table = use_signal(|| true);
    let mut dlg_desc = use_signal(|| true);
    let mut dlg_sort = use_signal(|| false);
    let mut dlg_primary_key = use_signal(|| false);
    let mut dlg_enum_values = use_signal(String::new);

    // 从数据库创建对话框
    let mut show_db_dialog = use_signal(|| false);
    let mut db_databases = use_signal(Vec::new);
    let mut db_tables = use_signal(Vec::new);
    let mut db_selected_db = use_signal(String::new);
    let mut db_selected_table = use_signal(String::new);
    let mut db_loading = use_signal(|| false);

    // 查看配置对话框
    let mut show_preview = use_signal(|| false);
    let mut json_text = use_signal(String::new);

    // 代码预览对话框
    let mut show_code_preview = use_signal(|| false);
    let mut preview_loading = use_signal(|| false);
    let mut preview_backend_files: Signal<Vec<GeneratedFile>> = use_signal(Vec::new);
    let mut preview_frontend_files: Signal<Vec<GeneratedFile>> = use_signal(Vec::new);
    let mut preview_selected_file: Signal<Option<usize>> = use_signal(|| None);
    let mut preview_selected_content = use_signal(String::new);
    let mut preview_active_tab = use_signal(|| "backend".to_string()); // "backend" | "frontend"

    // 使用 use_memo 缓存文件列表
    let backend_files_memo = use_memo(move || preview_backend_files());
    let frontend_files_memo = use_memo(move || preview_frontend_files());

    // 字段模板对话框
    let mut show_template_dialog = use_signal(|| false);

    // 从信号收集配置
    fn collect_config(
        table_name: String,
        resource: String,
        module_cn: String,
        icon: String,
        description: String,
        gen_backend: bool,
        gen_frontend: bool,
        batch_delete: bool,
        fields_val: Vec<GeneratorField>,
    ) -> GeneratorConfig {
        GeneratorConfig {
            table_name,
            resource,
            module_cn,
            icon,
            description,
            generate_backend: gen_backend,
            generate_frontend: gen_frontend,
            batch_delete,
            fields: fields_val,
        }
    }

    // 保存配置到数据库
    let do_save = move |_| {
        let config = collect_config(
            cfg_table_name(),
            cfg_resource(),
            cfg_module_cn(),
            cfg_icon(),
            cfg_description(),
            cfg_gen_backend(),
            cfg_gen_frontend(),
            cfg_batch_delete(),
            fields(),
        );
        if config.table_name.is_empty() || config.resource.is_empty() || config.module_cn.is_empty() {
            error_msg.set(Some("表名、资源名、中文名不能为空".to_string()));
            return;
        }
        saving.set(true);
        spawn(async move {
            match api::generator::save_to_db(&config).await {
                Ok(_) => {
                    success_msg.set(Some("配置已保存到数据库".to_string()));
                    error_msg.set(None);
                }
                Err(e) => {
                    error_msg.set(Some(e));
                    success_msg.set(None);
                }
            }
            saving.set(false);
        });
    };

    // 预览 JSON 配置
    let do_preview = move |_| {
        let config = collect_config(
            cfg_table_name(),
            cfg_resource(),
            cfg_module_cn(),
            cfg_icon(),
            cfg_description(),
            cfg_gen_backend(),
            cfg_gen_frontend(),
            cfg_batch_delete(),
            fields(),
        );
        match config_to_json(&config) {
            Ok(json) => {
                json_text.set(json);
                show_preview.set(true);
                error_msg.set(None);
            }
            Err(e) => {
                error_msg.set(Some(e));
                success_msg.set(None);
            }
        }
    };

    // 预览生成的代码 - 使用全局状态避免闭包捕获问题
    let do_code_preview = move |_: Event<MouseData>| {
        let table_name: String = cfg_table_name();
        let resource: String = cfg_resource();

        if table_name.is_empty() || resource.is_empty() {
            error_msg.set(Some("表名和资源名不能为空".to_string()));
            return;
        }

        let config = GeneratorConfig {
            table_name,
            resource,
            module_cn: cfg_module_cn(),
            icon: cfg_icon(),
            description: cfg_description(),
            generate_backend: cfg_gen_backend(),
            generate_frontend: cfg_gen_frontend(),
            batch_delete: cfg_batch_delete(),
            fields: fields(),
        };

        preview_loading.set(true);
        show_code_preview.set(true);
        preview_selected_file.set(None);
        preview_selected_content.set(String::new());

        spawn(async move {
            match api::generator::preview_code(&config).await {
                Ok(resp) => {
                    let backend_files = resp.backend_files.clone();
                    let frontend_files = resp.frontend_files.clone();
                    preview_backend_files.set(backend_files.clone());
                    preview_frontend_files.set(frontend_files.clone());
                    // 默认选择第一个后端文件
                    if !backend_files.is_empty() {
                        preview_selected_file.set(Some(0));
                        preview_selected_content.set(backend_files[0].content.clone());
                        preview_active_tab.set("backend".to_string());
                    } else if !frontend_files.is_empty() {
                        preview_selected_file.set(Some(0));
                        preview_selected_content.set(frontend_files[0].content.clone());
                        preview_active_tab.set("frontend".to_string());
                    }
                }
                Err(e) => {
                    error_msg.set(Some(format!("代码预览失败: {}", e)));
                    show_code_preview.set(false);
                }
            }
            preview_loading.set(false);
        });
    };

    // 导出 JSON 配置
    let do_export_json = move |_| {
        let config = collect_config(
            cfg_table_name(),
            cfg_resource(),
            cfg_module_cn(),
            cfg_icon(),
            cfg_description(),
            cfg_gen_backend(),
            cfg_gen_frontend(),
            cfg_batch_delete(),
            fields(),
        );

        match config_to_json(&config) {
            Ok(json) => {
                // 创建下载链接
                let blob = web_sys::Blob::new_with_str_sequence(&js_sys::Array::from(&json.into())).unwrap();
                let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
                let document = web_sys::window().unwrap().document().unwrap();
                let a = document.create_element("a").unwrap();
                a.set_attribute("href", &url).unwrap();
                a.set_attribute("download", &format!("{}_config.json", config.resource)).unwrap();
                document.body().unwrap().append_child(&a).unwrap();
                a.dyn_ref::<web_sys::HtmlElement>().unwrap().click();
                document.body().unwrap().remove_child(&a).unwrap();
                web_sys::Url::revoke_object_url(&url).unwrap();
                success_msg.set(Some("配置已导出".to_string()));
                error_msg.set(None);
            }
            Err(e) => {
                error_msg.set(Some(format!("导出失败: {}", e)));
                success_msg.set(None);
            }
        }
    };

    // 导入 JSON 配置 - 简化实现，通过粘贴板或文本输入
    let do_import_json = move |evt: Event<FormData>| {
        // 获取选中的文件
        let input_element = evt.data().value();
        if input_element.is_empty() {
            return;
        }

        // 尝试解析为 JSON
        match json_to_config(&input_element) {
            Ok(config) => {
                cfg_table_name.set(config.table_name);
                cfg_resource.set(config.resource);
                cfg_module_cn.set(config.module_cn);
                cfg_icon.set(config.icon);
                cfg_description.set(config.description);
                cfg_gen_backend.set(config.generate_backend);
                cfg_gen_frontend.set(config.generate_frontend);
                cfg_batch_delete.set(config.batch_delete);
                fields.set(config.fields);
                success_msg.set(Some("配置已导入".to_string()));
                error_msg.set(None);
            }
            Err(e) => {
                error_msg.set(Some(format!("导入失败: {}", e)));
                success_msg.set(None);
            }
        }
    };

    // 打开新增字段对话框
    let on_add_field = move |_| {
        edit_index.set(None);
        dlg_name.set(String::new());
        dlg_type.set("string".to_string());
        dlg_comment.set(String::new());
        dlg_nullable.set(true);
        dlg_search.set(false);
        dlg_search_type.set(String::new());
        dlg_require.set(false);
        dlg_default_value.set(String::new());
        dlg_form.set(true);
        dlg_table.set(true);
        dlg_desc.set(true);
        dlg_sort.set(false);
        dlg_primary_key.set(false);
        dlg_enum_values.set(String::new());
        show_field_dialog.set(true);
    };

    // 打开字段模板对话框
    let on_open_template: Box<dyn FnMut(_)> = Box::new(move |_| {
        show_template_dialog.set(true);
    });

    // 打开编辑字段对话框
    let mut on_edit_field = move |idx: usize| {
        if let Some(f) = fields().get(idx) {
            edit_index.set(Some(idx));
            dlg_name.set(f.name.clone());
            dlg_type.set(f.field_type.clone());
            dlg_comment.set(f.comment.clone());
            dlg_nullable.set(f.nullable);
            dlg_search.set(f.search);
            dlg_search_type.set(f.search_type.clone());
            dlg_require.set(f.require);
            dlg_default_value.set(f.default_value.clone());
            dlg_form.set(f.form);
            dlg_table.set(f.table);
            dlg_desc.set(f.desc);
            dlg_sort.set(f.sort);
            dlg_primary_key.set(f.primary_key);
            dlg_enum_values.set(f.enum_values.clone());
            show_field_dialog.set(true);
        }
    };

    // 删除字段
    let mut on_delete_field = move |idx: usize| {
        let mut list = fields();
        if idx < list.len() {
            list.remove(idx);
            fields.set(list);
        }
    };

    // 确认字段编辑
    let on_confirm_field = move |_| {
        let name = dlg_name().trim().to_string();
        if name.is_empty() {
            error_msg.set(Some("字段名不能为空".to_string()));
            return;
        }
        let field = GeneratorField {
            name,
            field_type: dlg_type(),
            nullable: dlg_nullable(),
            comment: dlg_comment(),
            search: dlg_search(),
            search_type: dlg_search_type(),
            require: dlg_require(),
            default_value: dlg_default_value(),
            form: dlg_form(),
            table: dlg_table(),
            desc: dlg_desc(),
            sort: dlg_sort(),
            primary_key: dlg_primary_key(),
            enum_values: dlg_enum_values(),
        };
        let mut list = fields();
        match edit_index() {
            Some(idx) => {
                if idx < list.len() {
                    list[idx] = field;
                }
            }
            None => {
                list.push(field);
            }
        }
        fields.set(list);
        show_field_dialog.set(false);
        error_msg.set(None);
    };

    // 打开从数据库创建对话框
    let on_open_db = move |_| {
        db_selected_db.set(String::new());
        db_selected_table.set(String::new());
        db_tables.set(Vec::new());
        show_db_dialog.set(true);
        // 加载数据库列表
        spawn(async move {
            match api::generator_history::get_databases().await {
                Ok(dbs) => { db_databases.set(dbs); }
                Err(e) => { error_msg.set(Some(e)); }
            }
        });
    };

    // 选择数据库后加载表
    let on_select_db = move |evt: Event<FormData>| {
        let db_name = evt.value();
        db_selected_db.set(db_name.clone());
        db_selected_table.set(String::new());
        db_tables.set(Vec::new());
        if !db_name.is_empty() {
            db_loading.set(true);
            spawn(async move {
                match api::generator_history::get_tables(&db_name).await {
                    Ok(tables) => { db_tables.set(tables); }
                    Err(e) => { error_msg.set(Some(e)); }
                }
                db_loading.set(false);
            });
        }
    };

    // 从数据库表生成配置并加载到表单
    let on_generate_from_table = move |_| {
        let db_name = db_selected_db();
        let table_name = db_selected_table();
        if db_name.is_empty() || table_name.is_empty() {
            error_msg.set(Some("请选择数据库和表".to_string()));
            return;
        }
        db_loading.set(true);
        spawn(async move {
            match api::generator_history::generate_from_table(&db_name, &table_name).await {
                Ok(json) => {
                    match json_to_config(&json) {
                        Ok(config) => {
                            cfg_table_name.set(config.table_name);
                            cfg_resource.set(config.resource);
                            cfg_module_cn.set(config.module_cn);
                            cfg_icon.set(config.icon);
                            cfg_description.set(config.description);
                            cfg_gen_backend.set(config.generate_backend);
                            cfg_gen_frontend.set(config.generate_frontend);
                            cfg_batch_delete.set(config.batch_delete);
                            fields.set(config.fields);
                            show_db_dialog.set(false);
                            success_msg.set(Some("从数据库导入成功".to_string()));
                            error_msg.set(None);
                        }
                        Err(e) => {
                            error_msg.set(Some(e));
                        }
                    }
                }
                Err(e) => { error_msg.set(Some(e)); }
            }
            db_loading.set(false);
        });
    };

    // 清空表单
    let do_clear = move |_| {
        cfg_table_name.set(String::new());
        cfg_resource.set(String::new());
        cfg_module_cn.set(String::new());
        cfg_icon.set("document".to_string());
        cfg_description.set(String::new());
        cfg_gen_backend.set(true);
        cfg_gen_frontend.set(true);
        cfg_batch_delete.set(true);
        fields.set(Vec::new());
        success_msg.set(Some("已清空表单".to_string()));
        error_msg.set(None);
    };

    // ===== 样式 =====
    let card_style = "background: var(--el-bg-color); border-radius: 4px; box-shadow: var(--el-box-shadow); margin-bottom: 16px;";
    let card_header_style = "padding: 12px 20px; border-bottom: 1px solid var(--el-border-color-lighter); font-size: 16px; font-weight: 500; color: var(--el-text-color-primary);";
    let card_body_style = "padding: 20px;";
    let label_style = "width: 120px; color: var(--el-text-color-regular); font-size: 14px; flex-shrink: 0;";
    let input_style = "flex: 1; padding: 8px 12px; border: 1px solid var(--el-border-color); border-radius: 4px; font-size: 14px; color: var(--el-text-color-primary); background: var(--el-bg-color);";
    let row_style = "display: flex; align-items: center; gap: 12px; margin-bottom: 16px;";
    let btn_primary = "padding: 8px 20px; background: var(--el-color-primary); color: #fff; border: none; border-radius: 4px; font-size: 14px; cursor: pointer;";
    let btn_default = "padding: 8px 20px; background: var(--el-bg-color); color: var(--el-text-color-regular); border: 1px solid var(--el-border-color); border-radius: 4px; font-size: 14px; cursor: pointer;";
    let btn_danger = "padding: 4px 12px; background: var(--el-color-danger); color: #fff; border: none; border-radius: 4px; font-size: 12px; cursor: pointer;";
    let btn_small = "padding: 4px 12px; background: var(--el-color-primary); color: #fff; border: none; border-radius: 4px; font-size: 12px; cursor: pointer;";

    // 表格样式
    let table_style = "width: 100%; border-collapse: collapse; font-size: 13px;";
    let th_style = "padding: 10px 8px; text-align: left; border-bottom: 2px solid var(--el-border-color); color: var(--el-text-color-secondary); font-weight: 500; white-space: nowrap;";
    let td_style = "padding: 8px; border-bottom: 1px solid var(--el-border-color-lighter); color: var(--el-text-color-regular);";

    rsx! {
        div {
            style: "padding: 20px;",

            // 页面标题
            h2 {
                style: "margin: 0 0 20px 0; font-size: 24px; font-weight: 500; color: var(--el-text-color-primary);",
                "{t(TKey::Generator)}"
            }

            // 消息提示
            if let Some(msg) = error_msg() {
                div {
                    style: "padding: 10px 16px; margin-bottom: 16px; background: var(--el-color-danger-light-9); border: 1px solid var(--el-color-danger-light-5); border-radius: 4px; color: var(--el-color-danger); font-size: 14px;",
                    {msg}
                }
            }
            if let Some(msg) = success_msg() {
                div {
                    style: "padding: 10px 16px; margin-bottom: 16px; background: var(--el-color-success-light-9); border: 1px solid var(--el-color-success-light-5); border-radius: 4px; color: var(--el-color-success); font-size: 14px;",
                    {msg}
                }
            }

            // ===== 模块配置卡片 =====
            div {
                style: "{card_style}",

                div { style: "{card_header_style}", "{t(TKey::GeneratorConfig)}" }

                div {
                    style: "{card_body_style}",

                    // 表名 + 资源名
                    div {
                        style: "{row_style}",
                        label { style: "{label_style}", "{t(TKey::TableName)} *" }
                        input {
                            style: "{input_style}",
                            r#type: "text",
                            placeholder: "如 sys_products (sys_ 前缀)",
                            value: "{cfg_table_name()}",
                            oninput: move |evt| { cfg_table_name.set(evt.value()); }
                        }
                        label {
                            style: "width: 100px; color: var(--el-text-color-regular); font-size: 14px; flex-shrink: 0;",
                            "{t(TKey::ResourceName)} *"
                        }
                        input {
                            style: "{input_style}",
                            r#type: "text",
                            placeholder: "如 product (snake_case)",
                            value: "{cfg_resource()}",
                            oninput: move |evt| { cfg_resource.set(evt.value()); }
                        }
                    }

                    // 模块名 + 图标
                    div {
                        style: "{row_style}",
                        label { style: "{label_style}", "{t(TKey::ModuleCn)} *" }
                        input {
                            style: "{input_style}",
                            r#type: "text",
                            placeholder: "如 产品管理",
                            value: "{cfg_module_cn()}",
                            oninput: move |evt| { cfg_module_cn.set(evt.value()); }
                        }
                        label {
                            style: "width: 100px; color: var(--el-text-color-regular); font-size: 14px; flex-shrink: 0;",
                            "{t(TKey::IconName)}"
                        }
                        input {
                            style: "{input_style}",
                            r#type: "text",
                            placeholder: "Element Plus 图标名",
                            value: "{cfg_icon()}",
                            oninput: move |evt| { cfg_icon.set(evt.value()); }
                        }
                    }

                    // 描述
                    div {
                        style: "{row_style}",
                        label { style: "{label_style}", "{t(TKey::Description)}" }
                        input {
                            style: "{input_style}",
                            r#type: "text",
                            placeholder: "Struct 中文描述 (可选, 默认同中文名)",
                            value: "{cfg_description()}",
                            oninput: move |evt| { cfg_description.set(evt.value()); }
                        }
                    }

                    // 选项
                    div {
                        style: "display: flex; gap: 32px; margin-top: 8px;",

                        label {
                            style: "display: flex; align-items: center; gap: 6px; font-size: 14px; color: var(--el-text-color-regular); cursor: pointer;",
                            input {
                                r#type: "checkbox",
                                checked: "{cfg_gen_backend()}",
                                onchange: move |evt| { cfg_gen_backend.set(evt.checked()); }
                            }
                            "{t(TKey::GenBackend)}"
                        }

                        label {
                            style: "display: flex; align-items: center; gap: 6px; font-size: 14px; color: var(--el-text-color-regular); cursor: pointer;",
                            input {
                                r#type: "checkbox",
                                checked: "{cfg_gen_frontend()}",
                                onchange: move |evt| { cfg_gen_frontend.set(evt.checked()); }
                            }
                            "{t(TKey::GenFrontend)}"
                        }

                        label {
                            style: "display: flex; align-items: center; gap: 6px; font-size: 14px; color: var(--el-text-color-regular); cursor: pointer;",
                            input {
                                r#type: "checkbox",
                                checked: "{cfg_batch_delete()}",
                                onchange: move |evt| { cfg_batch_delete.set(evt.checked()); }
                            }
                            "{t(TKey::BatchDelete)}"
                        }
                    }
                }
            }

            // ===== 字段列表卡片 =====
            div {
                style: "{card_style}",

                div {
                    style: "{card_header_style} display: flex; justify-content: space-between; align-items: center;",
                    span { "{t(TKey::GeneratorFields)} ({fields().len()})" }
                    button {
                        style: "{btn_primary}",
                        onclick: on_add_field,
                        "{t(TKey::AddField)}"
                    }
                }

                div {
                    style: "{card_body_style}; overflow-x: auto;",

                    if fields().is_empty() {
                        div {
                            style: "text-align: center; padding: 40px; color: var(--el-text-color-secondary); font-size: 14px;",
                            "{t(TKey::NoData)}"
                        }
                    } else {
                        table { style: "{table_style}",
                            thead {
                                tr {
                                    th { style: "{th_style}", "#" }
                                    th { style: "{th_style}", "{t(TKey::FieldName)}" }
                                    th { style: "{th_style}", "{t(TKey::FieldType)}" }
                                    th { style: "{th_style}", "{t(TKey::FieldComment)}" }
                                    th { style: "{th_style}", "{t(TKey::SearchType)}" }
                                    th { style: "{th_style}", "{t(TKey::Required)}" }
                                    th { style: "{th_style}", "{t(TKey::FormDisplay)}" }
                                    th { style: "{th_style}", "{t(TKey::TableDisplay)}" }
                                    th { style: "{th_style}", "{t(TKey::Sortable)}" }
                                    th { style: "{th_style}", "{t(TKey::PrimaryKey)}" }
                                    th { style: "{th_style}", "{t(TKey::DefaultValue)}" }
                                    th { style: "{th_style}", "{t(TKey::Action)}" }
                                }
                            }
                            tbody {
                                for (idx, field) in fields().iter().enumerate() {
                                    tr {
                                        td { style: "{td_style}", "{idx + 1}" }
                                        td { style: "{td_style}", "{field.name}" }
                                        td { style: "{td_style}", "{field.field_type}" }
                                        td { style: "{td_style}", "{field.comment}" }
                                        td { style: "{td_style}",
                                            if field.search || !field.search_type.is_empty() {
                                                if !field.search_type.is_empty() {
                                                    "{field.search_type}"
                                                } else {
                                                    "like"
                                                }
                                            } else {
                                                "—"
                                            }
                                        }
                                        td { style: "{td_style}",
                                            if field.require { "✓" } else { "—" }
                                        }
                                        td { style: "{td_style}",
                                            if field.form { "✓" } else { "—" }
                                        }
                                        td { style: "{td_style}",
                                            if field.table { "✓" } else { "—" }
                                        }
                                        td { style: "{td_style}",
                                            if field.sort { "✓" } else { "—" }
                                        }
                                        td { style: "{td_style}",
                                            if field.primary_key { "✓" } else { "—" }
                                        }
                                        td { style: "{td_style}", "{field.default_value}" }
                                        td { style: "{td_style}",
                                            // 上移按钮
                                            button {
                                                style: "{btn_small}",
                                                disabled: "{idx == 0}",
                                                onclick: move |_| {
                                                    if idx > 0 {
                                                        let mut current_fields = fields();
                                                        current_fields.swap(idx, idx - 1);
                                                        fields.set(current_fields);
                                                    }
                                                },
                                                "↑"
                                            }
                                            // 下移按钮
                                            button {
                                                style: "{btn_small} margin-left: 4px;",
                                                onclick: move |_| {
                                                    let mut current_fields = fields();
                                                    if idx < current_fields.len() - 1 {
                                                        current_fields.swap(idx, idx + 1);
                                                        fields.set(current_fields);
                                                    }
                                                },
                                                "↓"
                                            }
                                            button {
                                                style: "{btn_small} margin-left: 4px;",
                                                onclick: move |_| { on_edit_field(idx); },
                                                "{t(TKey::Edit)}"
                                            }
                                            button {
                                                style: "{btn_danger} margin-left: 4px;",
                                                onclick: move |_| { on_delete_field(idx); },
                                                "{t(TKey::Delete)}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ===== 操作按钮 =====
            div {
                style: "display: flex; gap: 12px; justify-content: flex-end; padding: 16px 0; flex-wrap: wrap;",

                button {
                    style: "{btn_default}",
                    onclick: on_open_db,
                    "从数据库创建"
                }
                button {
                    style: "{btn_default}",
                    onclick: do_clear,
                    "清空表单"
                }
                button {
                    style: "{btn_default}",
                    onclick: do_preview,
                    "预览配置"
                }
                button {
                    style: "{btn_default}",
                    onclick: do_code_preview,
                    "预览代码"
                }
                button {
                    style: "{btn_default}",
                    onclick: do_export_json,
                    "导出JSON"
                }
                label {
                    style: "{btn_default} display: inline-block; cursor: pointer;",
                    input {
                        r#type: "file",
                        accept: ".json",
                        style: "display: none;",
                        onchange: do_import_json,
                    }
                    "导入JSON"
                }
                button {
                    style: "{btn_default}",
                    onclick: on_open_template,
                    "快速添加字段"
                }
                button {
                    style: "{btn_primary}",
                    disabled: "{saving()}",
                    onclick: do_save,
                    if saving() { "保存中..." } else { "{t(TKey::SaveConfig)}" }
                }
            }

            // ===== 配置预览对话框 =====
            if show_preview() {
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); z-index: 2000; display: flex; align-items: center; justify-content: center;",
                    onclick: move |_| { show_preview.set(false); },

                    div {
                        style: "background: var(--el-bg-color); border-radius: 8px; width: 80%; max-width: 800px; max-height: 80vh; display: flex; flex-direction: column;",
                        onclick: move |evt| { evt.stop_propagation(); },

                        // 头部
                        div {
                            style: "padding: 16px 20px; border-bottom: 1px solid var(--el-border-color-lighter); display: flex; justify-content: space-between; align-items: center;",

                            span {
                                style: "font-size: 16px; font-weight: 500; color: var(--el-text-color-primary);",
                                "配置预览 (JSON)"
                            }
                            button {
                                style: "{btn_default}",
                                onclick: move |_| { show_preview.set(false); },
                                "关闭"
                            }
                        }

                        // 内容
                        div {
                            style: "padding: 16px 20px; overflow-y: auto; flex: 1;",

                            pre {
                                style: "margin: 0; padding: 16px; background: var(--el-fill-color-darker); border-radius: 4px; font-size: 13px; color: var(--el-text-color-primary); white-space: pre-wrap; word-break: break-all; font-family: 'Menlo', 'Monaco', monospace;",
                                "{json_text()}"
                            }
                        }
                    }
                }
            }

            // ===== 代码预览对话框 =====
            if show_code_preview() {
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); z-index: 2000; display: flex; align-items: center; justify-content: center;",
                    onclick: move |_| { show_code_preview.set(false); },

                    div {
                        style: "background: var(--el-bg-color); border-radius: 8px; width: 90%; max-width: 1200px; height: 85vh; display: flex; flex-direction: column;",
                        onclick: move |evt| { evt.stop_propagation(); },

                        // 头部
                        div {
                            style: "padding: 16px 20px; border-bottom: 1px solid var(--el-border-color-lighter); display: flex; justify-content: space-between; align-items: center;",

                            span {
                                style: "font-size: 16px; font-weight: 500; color: var(--el-text-color-primary);",
                                "代码预览"
                            }
                            div {
                                style: "display: flex; gap: 8px;",
                                if preview_loading() {
                                    span {
                                        style: "color: var(--el-text-color-secondary); font-size: 14px;",
                                        "加载中..."
                                    }
                                }
                                button {
                                    style: "{btn_default}",
                                    onclick: move |_| { show_code_preview.set(false); },
                                    "关闭"
                                }
                            }
                        }

                        // Tab 切换
                        div {
                            style: "display: flex; border-bottom: 1px solid var(--el-border-color-lighter);",

                            button {
                                style: if preview_active_tab() == "backend" { "padding: 12px 24px; background: var(--el-bg-color); color: var(--el-color-primary); border: none; border-bottom: 2px solid var(--el-color-primary); font-size: 14px; cursor: pointer;" } else { "padding: 12px 24px; background: var(--el-bg-color); color: var(--el-text-color-regular); border: none; border-bottom: 2px solid transparent; font-size: 14px; cursor: pointer;" },
                                onclick: move |_| {
                                    preview_active_tab.set("backend".to_string());
                                    preview_selected_file.set(None);
                                    preview_selected_content.set(String::new());
                                },
                                "后端代码 ("
                                {preview_backend_files().len().to_string()}
                                ")"
                            }
                            button {
                                style: if preview_active_tab() == "frontend" { "padding: 12px 24px; background: var(--el-bg-color); color: var(--el-color-primary); border: none; border-bottom: 2px solid var(--el-color-primary); font-size: 14px; cursor: pointer;" } else { "padding: 12px 24px; background: var(--el-bg-color); color: var(--el-text-color-regular); border: none; border-bottom: 2px solid transparent; font-size: 14px; cursor: pointer;" },
                                onclick: move |_| {
                                    preview_active_tab.set("frontend".to_string());
                                    preview_selected_file.set(None);
                                    preview_selected_content.set(String::new());
                                },
                                "前端代码 ("
                                {preview_frontend_files().len().to_string()}
                                ")"
                            }
                        }

                        // 内容区域
                        div {
                            style: "display: flex; flex: 1; overflow: hidden;",

                            // 左侧文件列表
                            div {
                                style: "width: 280px; border-right: 1px solid var(--el-border-color-lighter); overflow-y: auto; background: var(--el-fill-color-lighter);",

                                // 文件列表 - 后端
                                for idx in 0..backend_files_memo().len() {
                                    if preview_active_tab() == "backend" {
                                        div {
                                            key: "{idx}",
                                            style: if preview_selected_file() == Some(idx) { "padding: 12px 16px; cursor: pointer; background: var(--el-bg-color); border-left: 3px solid var(--el-color-primary);" } else { "padding: 12px 16px; cursor: pointer; border-left: 3px solid transparent;" },
                                            onclick: move |_| {
                                                preview_selected_file.set(Some(idx));
                                                preview_selected_content.set(backend_files_memo()[idx].content.clone());
                                            },

                                            div {
                                                style: "font-size: 13px; color: var(--el-text-color-primary); font-weight: 500;",
                                                "{backend_files_memo()[idx].file_name}"
                                            }
                                            div {
                                                style: "font-size: 11px; color: var(--el-text-color-secondary); margin-top: 4px;",
                                                "{backend_files_memo()[idx].file_path}"
                                            }
                                        }
                                    }
                                }
                                // 文件列表 - 前端
                                for idx in 0..frontend_files_memo().len() {
                                    if preview_active_tab() == "frontend" {
                                        div {
                                            key: "{idx}",
                                            style: if preview_selected_file() == Some(idx) { "padding: 12px 16px; cursor: pointer; background: var(--el-bg-color); border-left: 3px solid var(--el-color-primary);" } else { "padding: 12px 16px; cursor: pointer; border-left: 3px solid transparent;" },
                                            onclick: move |_| {
                                                preview_selected_file.set(Some(idx));
                                                preview_selected_content.set(frontend_files_memo()[idx].content.clone());
                                            },

                                            div {
                                                style: "font-size: 13px; color: var(--el-text-color-primary); font-weight: 500;",
                                                "{frontend_files_memo()[idx].file_name}"
                                            }
                                            div {
                                                style: "font-size: 11px; color: var(--el-text-color-secondary); margin-top: 4px;",
                                                "{frontend_files_memo()[idx].file_path}"
                                            }
                                        }
                                    }
                                }
                            }

                            // 右侧代码内容
                            div {
                                style: "flex: 1; overflow: auto; background: var(--el-fill-color-darker);",

                                if preview_selected_content().is_empty() {
                                    div {
                                        style: "display: flex; align-items: center; justify-content: center; height: 100%; color: var(--el-text-color-secondary); font-size: 14px;",
                                        "请选择左侧文件查看代码"
                                    }
                                } else {
                                    pre {
                                        style: "margin: 0; padding: 20px; font-size: 13px; line-height: 1.6; color: var(--el-text-color-primary); white-space: pre; font-family: 'Menlo', 'Monaco', 'Consolas', monospace;",
                                        "{preview_selected_content()}"
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ===== 字段编辑对话框 =====
            if show_field_dialog() {
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); z-index: 2000; display: flex; align-items: center; justify-content: center;",
                    onclick: move |_| { show_field_dialog.set(false); },

                    div {
                        style: "background: var(--el-bg-color); border-radius: 8px; width: 600px; max-height: 85vh; display: flex; flex-direction: column; overflow-y: auto;",
                        onclick: move |evt| { evt.stop_propagation(); },

                        // 头部
                        div {
                            style: "padding: 16px 20px; border-bottom: 1px solid var(--el-border-color-lighter); display: flex; justify-content: space-between; align-items: center;",
                            span {
                                style: "font-size: 16px; font-weight: 500; color: var(--el-text-color-primary);",
                                if edit_index().is_some() { "{t(TKey::EditField)}" } else { "{t(TKey::AddField)}" }
                            }
                            button {
                                style: "{btn_default}",
                                onclick: move |_| { show_field_dialog.set(false); },
                                "{t(TKey::Cancel)}"
                            }
                        }

                        // 内容
                        div {
                            style: "padding: 20px;",

                            // 字段名
                            div { style: "{row_style}",
                                label { style: "{label_style}", "{t(TKey::FieldName)} *" }
                                input {
                                    style: "{input_style}",
                                    r#type: "text",
                                    placeholder: "snake_case, 如 product_name",
                                    value: "{dlg_name()}",
                                    oninput: move |evt| { dlg_name.set(evt.value()); }
                                }
                            }

                            // 类型
                            div { style: "{row_style}",
                                label { style: "{label_style}", "{t(TKey::FieldType)} *" }
                                select {
                                    style: "{input_style}",
                                    value: "{dlg_type()}",
                                    onchange: move |evt| { dlg_type.set(evt.value()); },
                                    for (val, label) in FIELD_TYPES {
                                        option {
                                            value: "{val}",
                                            selected: dlg_type() == *val,
                                            "{label}"
                                        }
                                    }
                                }
                            }

                            // 注释
                            div { style: "{row_style}",
                                label { style: "{label_style}", "{t(TKey::FieldComment)}" }
                                input {
                                    style: "{input_style}",
                                    r#type: "text",
                                    placeholder: "数据库字段注释",
                                    value: "{dlg_comment()}",
                                    oninput: move |evt| { dlg_comment.set(evt.value()); }
                                }
                            }

                            // 搜索类型
                            div { style: "{row_style}",
                                label { style: "{label_style}", "{t(TKey::SearchType)}" }
                                select {
                                    style: "{input_style}",
                                    value: "{dlg_search_type()}",
                                    onchange: move |evt| { dlg_search_type.set(evt.value()); },
                                    for (val, label) in SEARCH_TYPES {
                                        option {
                                            value: "{val}",
                                            selected: dlg_search_type() == *val,
                                            "{label}"
                                        }
                                    }
                                }
                                label {
                                    style: "display: flex; align-items: center; gap: 4px; font-size: 13px; color: var(--el-text-color-secondary); cursor: pointer; white-space: nowrap;",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{dlg_search()}",
                                        onchange: move |evt| { dlg_search.set(evt.checked()); }
                                    }
                                    "{t(TKey::UseSimplifiedSearch)}"
                                }
                            }

                            // 默认值
                            div { style: "{row_style}",
                                label { style: "{label_style}", "{t(TKey::DefaultValue)}" }
                                input {
                                    style: "{input_style}",
                                    r#type: "text",
                                    placeholder: "可选",
                                    value: "{dlg_default_value()}",
                                    oninput: move |evt| { dlg_default_value.set(evt.value()); }
                                }
                            }

                            // 枚举值 (仅 type=enum 时显示)
                            if dlg_type() == "enum" {
                                div { style: "{row_style}",
                                    label { style: "{label_style}", "{t(TKey::EnumValues)}" }
                                    input {
                                        style: "{input_style}",
                                        r#type: "text",
                                        placeholder: "逗号分隔, 如 low,medium,high",
                                        value: "{dlg_enum_values()}",
                                        oninput: move |evt| { dlg_enum_values.set(evt.value()); }
                                    }
                                }
                            }

                            // 复选框组
                            div {
                                style: "display: flex; flex-wrap: wrap; gap: 16px; margin-top: 20px; padding-top: 16px; border-top: 1px solid var(--el-border-color-lighter);",

                                label {
                                    style: "display: flex; align-items: center; gap: 6px; font-size: 14px; color: var(--el-text-color-regular); cursor: pointer;",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{dlg_nullable()}",
                                        onchange: move |evt| { dlg_nullable.set(evt.checked()); }
                                    }
                                    "{t(TKey::Nullable)}"
                                }
                                label {
                                    style: "display: flex; align-items: center; gap: 6px; font-size: 14px; color: var(--el-text-color-regular); cursor: pointer;",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{dlg_require()}",
                                        onchange: move |evt| { dlg_require.set(evt.checked()); }
                                    }
                                    "{t(TKey::Required)}"
                                }
                                label {
                                    style: "display: flex; align-items: center; gap: 6px; font-size: 14px; color: var(--el-text-color-regular); cursor: pointer;",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{dlg_form()}",
                                        onchange: move |evt| { dlg_form.set(evt.checked()); }
                                    }
                                    "{t(TKey::FormDisplay)}"
                                }
                                label {
                                    style: "display: flex; align-items: center; gap: 6px; font-size: 14px; color: var(--el-text-color-regular); cursor: pointer;",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{dlg_table()}",
                                        onchange: move |evt| { dlg_table.set(evt.checked()); }
                                    }
                                    "{t(TKey::TableDisplay)}"
                                }
                                label {
                                    style: "display: flex; align-items: center; gap: 6px; font-size: 14px; color: var(--el-text-color-regular); cursor: pointer;",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{dlg_desc()}",
                                        onchange: move |evt| { dlg_desc.set(evt.checked()); }
                                    }
                                    "{t(TKey::DescDisplay)}"
                                }
                                label {
                                    style: "display: flex; align-items: center; gap: 6px; font-size: 14px; color: var(--el-text-color-regular); cursor: pointer;",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{dlg_sort()}",
                                        onchange: move |evt| { dlg_sort.set(evt.checked()); }
                                    }
                                    "{t(TKey::Sortable)}"
                                }
                                label {
                                    style: "display: flex; align-items: center; gap: 6px; font-size: 14px; color: var(--el-text-color-regular); cursor: pointer;",
                                    input {
                                        r#type: "checkbox",
                                        checked: "{dlg_primary_key()}",
                                        onchange: move |evt| { dlg_primary_key.set(evt.checked()); }
                                    }
                                    "{t(TKey::PrimaryKey)}"
                                }
                            }
                        }

                        // 底部按钮
                        div {
                            style: "padding: 16px 20px; border-top: 1px solid var(--el-border-color-lighter); display: flex; justify-content: flex-end; gap: 8px;",
                            button {
                                style: "{btn_default}",
                                onclick: move |_| { show_field_dialog.set(false); },
                                "{t(TKey::Cancel)}"
                            }
                            button {
                                style: "{btn_primary}",
                                onclick: on_confirm_field,
                                "{t(TKey::Confirm)}"
                            }
                        }
                    }
                }
            }

            // ===== 字段模板对话框 =====
            if show_template_dialog() {
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); z-index: 2000; display: flex; align-items: center; justify-content: center;",
                    onclick: move |_| { show_template_dialog.set(false); },

                    div {
                        style: "background: var(--el-bg-color); border-radius: 8px; width: 500px; max-height: 80vh; display: flex; flex-direction: column;",
                        onclick: move |evt| { evt.stop_propagation(); },

                        // 头部
                        div {
                            style: "padding: 16px 20px; border-bottom: 1px solid var(--el-border-color-lighter); display: flex; justify-content: space-between; align-items: center;",

                            span {
                                style: "font-size: 16px; font-weight: 500; color: var(--el-text-color-primary);",
                                "快速添加字段"
                            }
                            button {
                                style: "{btn_default}",
                                onclick: move |_| { show_template_dialog.set(false); },
                                "关闭"
                            }
                        }

                        // 内容
                        div {
                            style: "padding: 20px; overflow-y: auto;",

                            div {
                                style: "display: flex; flex-direction: column; gap: 12px;",

                                for template in get_field_templates() {
                                    div {
                                        key: "{template.name}",
                                        style: "padding: 16px; border: 1px solid var(--el-border-color); border-radius: 6px; cursor: pointer; transition: all 0.2s;",
                                        class: "template-item",
                                        onclick: {
                                            let template = template.clone();
                                            move |_| {
                                                let mut current_fields = fields();
                                                for field in &template.fields {
                                                    current_fields.push(field.clone());
                                                }
                                                fields.set(current_fields);
                                                show_template_dialog.set(false);
                                                success_msg.set(Some(format!("已添加: {}", template.label)));
                                                error_msg.set(None);
                                            }
                                        },

                                        div {
                                            style: "font-size: 15px; font-weight: 500; color: var(--el-text-color-primary); margin-bottom: 8px;",
                                            "{template.label}"
                                        }
                                        div {
                                            style: "font-size: 13px; color: var(--el-text-color-secondary);",
                                            "包含字段: "
                                            {template.fields.iter().map(|f| f.name.clone()).collect::<Vec<_>>().join(", ")}
                                        }
                                    }
                                }
                            }
                        }

                        // 底部提示
                        div {
                            style: "padding: 12px 20px; border-top: 1px solid var(--el-border-color-lighter); background: var(--el-fill-color-lighter); font-size: 12px; color: var(--el-text-color-secondary);",
                            "点击上方模板即可快速添加一组预设字段到当前列表"
                        }
                    }
                }
            }

            // ===== 从数据库创建对话框 =====
            if show_db_dialog() {
                div {
                    style: "position: fixed; top: 0; left: 0; right: 0; bottom: 0; background: rgba(0,0,0,0.5); z-index: 2000; display: flex; align-items: center; justify-content: center;",
                    onclick: move |_| { show_db_dialog.set(false); },

                    div {
                        style: "background: var(--el-bg-color); border-radius: 8px; width: 500px; display: flex; flex-direction: column;",
                        onclick: move |evt| { evt.stop_propagation(); },

                        div {
                            style: "padding: 16px 20px; border-bottom: 1px solid var(--el-border-color-lighter); font-size: 16px; font-weight: 500; color: var(--el-text-color-primary);",
                            "从数据库创建"
                        }

                        div {
                            style: "padding: 20px; display: flex; flex-direction: column; gap: 16px;",

                            // 选择数据库
                            div {
                                style: "{row_style}",
                                label { style: "{label_style}", "数据库" }
                                select {
                                    style: "{input_style}",
                                    value: "{db_selected_db()}",
                                    onchange: on_select_db,
                                    option { value: "", "请选择数据库" }
                                    for db in db_databases().iter() {
                                        option {
                                            value: "{db.database}",
                                            selected: "{db_selected_db() == db.database}",
                                            "{db.database}"
                                        }
                                    }
                                }
                            }

                            // 选择表
                            div {
                                style: "{row_style}",
                                label { style: "{label_style}", "表名" }
                                select {
                                    style: "{input_style}",
                                    value: "{db_selected_table()}",
                                    disabled: "{db_tables().is_empty()}",
                                    onchange: move |evt: Event<FormData>| {
                                        db_selected_table.set(evt.value());
                                    },
                                    option { value: "", "请选择表" }
                                    for tbl in db_tables().iter() {
                                        option {
                                            value: "{tbl.table_name}",
                                            selected: "{db_selected_table() == tbl.table_name}",
                                            "{tbl.table_name}"
                                        }
                                    }
                                }
                            }

                            if db_loading() {
                                div {
                                    style: "text-align: center; color: var(--el-text-color-secondary); font-size: 14px;",
                                    "{t(TKey::Loading)}"
                                }
                            }
                        }

                        div {
                            style: "padding: 16px 20px; border-top: 1px solid var(--el-border-color-lighter); display: flex; justify-content: flex-end; gap: 8px;",
                            button {
                                style: "{btn_default}",
                                onclick: move |_| { show_db_dialog.set(false); },
                                "{t(TKey::Cancel)}"
                            }
                            button {
                                style: "{btn_primary}",
                                disabled: "{db_selected_db().is_empty() || db_selected_table().is_empty() || db_loading()}",
                                onclick: on_generate_from_table,
                                "生成配置"
                            }
                        }
                    }
                }
            }

            // ===== 使用说明 =====
            div {
                style: "{card_style}",

                div { style: "{card_header_style}", "{t(TKey::UsageGuide)}" }

                div {
                    style: "{card_body_style}; color: var(--el-text-color-secondary); font-size: 13px; line-height: 1.8;",

                    p { "1. 配置模块信息 (表名、资源名、中文名等)" }
                    p { "2. 添加字段，设置类型、搜索、排序等属性" }
                    p { "3. 点击「保存配置」将配置保存到数据库" }
                    p { "4. 可使用「从数据库创建」自动读取表结构生成配置" }
                    p { "5. 在「生成历史」页面可查看、回滚已保存的配置" }
                }
            }
        }
    }
}
