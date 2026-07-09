//! 代码生成器数据模型
//!
//! 与 generator/src/config.rs 中的 ModuleConfig / FieldConfig 对应，
//! 前端用于构建可视化配置页面，配置以 JSON 存储到数据库。

use serde::{Deserialize, Serialize};

/// 模块配置 (存储到数据库 sys_generator_history.request 字段)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorConfig {
    /// 数据库表名 (sys_ 前缀)
    pub table_name: String,
    /// 资源名 (snake_case 单数)
    pub resource: String,
    /// 中文模块名
    pub module_cn: String,
    /// Element Plus 图标名
    #[serde(default = "default_icon")]
    pub icon: String,
    /// Struct 中文描述 (可选, 默认同 module_cn)
    #[serde(default)]
    pub description: String,
    /// 是否生成后端
    #[serde(default = "default_true")]
    pub generate_backend: bool,
    /// 是否生成前端
    #[serde(default = "default_true")]
    pub generate_frontend: bool,
    /// 是否支持批量删除
    #[serde(default = "default_true")]
    pub batch_delete: bool,
    /// 字段列表
    pub fields: Vec<GeneratorField>,
}

/// 字段配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorField {
    /// 字段名 (snake_case)
    pub name: String,
    /// 字段类型
    #[serde(rename = "type")]
    pub field_type: String,
    /// 是否可空
    #[serde(default = "default_true")]
    pub nullable: bool,
    /// 数据库注释
    #[serde(default)]
    pub comment: String,
    /// 是否参与关键字搜索
    #[serde(default)]
    pub search: bool,
    /// 搜索类型: like, eq, ne, gt, lt, gte, lte, between
    #[serde(default)]
    pub search_type: String,
    /// 是否必填
    #[serde(default)]
    pub require: bool,
    /// 默认值
    #[serde(default)]
    pub default_value: String,
    /// 是否在表单中显示
    #[serde(default = "default_true")]
    pub form: bool,
    /// 是否在表格中显示
    #[serde(default = "default_true")]
    pub table: bool,
    /// 是否在详情中显示
    #[serde(default = "default_true")]
    pub desc: bool,
    /// 是否可排序
    #[serde(default)]
    pub sort: bool,
    /// 是否主键
    #[serde(default)]
    pub primary_key: bool,
    /// 枚举值 (仅 type=enum 时使用)
    #[serde(default)]
    pub enum_values: String,
}

impl Default for GeneratorConfig {
    fn default() -> Self {
        Self {
            table_name: String::new(),
            resource: String::new(),
            module_cn: String::new(),
            icon: "document".into(),
            description: String::new(),
            generate_backend: true,
            generate_frontend: true,
            batch_delete: true,
            fields: Vec::new(),
        }
    }
}

impl Default for GeneratorField {
    fn default() -> Self {
        Self {
            name: String::new(),
            field_type: "string".into(),
            nullable: true,
            comment: String::new(),
            search: false,
            search_type: String::new(),
            require: false,
            default_value: String::new(),
            form: true,
            table: true,
            desc: true,
            sort: false,
            primary_key: false,
            enum_values: String::new(),
        }
    }
}

/// 可用的字段类型选项
pub const FIELD_TYPES: &[(&str, &str)] = &[
    ("string", "字符串"),
    ("text", "富文本"),
    ("i8", "i8 (小整型)"),
    ("i16", "i16"),
    ("i32", "i32 (整型)"),
    ("i64", "i64 (长整型)"),
    ("u64", "u64"),
    ("f32", "f32"),
    ("f64", "f64 (浮点型)"),
    ("bool", "布尔值"),
    ("decimal", "Decimal (高精度)"),
    ("date", "日期"),
    ("datetime", "日期时间"),
    ("json", "JSON"),
    ("array", "数组"),
    ("enum", "枚举"),
];

/// 搜索类型选项
pub const SEARCH_TYPES: &[(&str, &str)] = &[
    ("", "不搜索"),
    ("like", "LIKE 模糊匹配"),
    ("eq", "= 等于"),
    ("ne", "≠ 不等于"),
    ("gt", "> 大于"),
    ("lt", "< 小于"),
    ("gte", "≥ 大于等于"),
    ("lte", "≤ 小于等于"),
    ("between", "BETWEEN 范围"),
];

/// 将配置序列化为 JSON 字符串
pub fn config_to_json(config: &GeneratorConfig) -> Result<String, String> {
    serde_json::to_string_pretty(config).map_err(|e| format!("JSON 序列化失败: {}", e))
}

/// 将 JSON 字符串反序列化为配置
pub fn json_to_config(json: &str) -> Result<GeneratorConfig, String> {
    serde_json::from_str(json).map_err(|e| format!("JSON 解析失败: {}", e))
}

fn default_icon() -> String {
    "document".into()
}

fn default_true() -> bool {
    true
}

/// 字段模板
#[derive(Debug, Clone)]
pub struct FieldTemplate {
    pub name: &'static str,
    pub label: &'static str,
    pub fields: Vec<GeneratorField>,
}

/// 获取常用字段模板
pub fn get_field_templates() -> Vec<FieldTemplate> {
    vec![
        FieldTemplate {
            name: "timestamps",
            label: "时间戳字段 (创建/更新时间)",
            fields: vec![
                GeneratorField {
                    name: "created_at".into(),
                    field_type: "datetime".into(),
                    nullable: true,
                    comment: "创建时间".into(),
                    search: false,
                    search_type: String::new(),
                    require: false,
                    default_value: String::new(),
                    form: false,
                    table: true,
                    desc: true,
                    sort: true,
                    primary_key: false,
                    enum_values: String::new(),
                },
                GeneratorField {
                    name: "updated_at".into(),
                    field_type: "datetime".into(),
                    nullable: true,
                    comment: "更新时间".into(),
                    search: false,
                    search_type: String::new(),
                    require: false,
                    default_value: String::new(),
                    form: false,
                    table: true,
                    desc: true,
                    sort: true,
                    primary_key: false,
                    enum_values: String::new(),
                },
            ],
        },
        FieldTemplate {
            name: "status",
            label: "状态字段 (启用/禁用)",
            fields: vec![
                GeneratorField {
                    name: "status".into(),
                    field_type: "i8".into(),
                    nullable: false,
                    comment: "状态 (0-禁用, 1-启用)".into(),
                    search: true,
                    search_type: "eq".into(),
                    require: true,
                    default_value: "1".into(),
                    form: true,
                    table: true,
                    desc: true,
                    sort: false,
                    primary_key: false,
                    enum_values: String::new(),
                },
            ],
        },
        FieldTemplate {
            name: "sort",
            label: "排序字段",
            fields: vec![
                GeneratorField {
                    name: "sort_order".into(),
                    field_type: "i32".into(),
                    nullable: false,
                    comment: "排序序号".into(),
                    search: false,
                    search_type: String::new(),
                    require: true,
                    default_value: "0".into(),
                    form: true,
                    table: true,
                    desc: true,
                    sort: true,
                    primary_key: false,
                    enum_values: String::new(),
                },
            ],
        },
        FieldTemplate {
            name: "remark",
            label: "备注字段",
            fields: vec![
                GeneratorField {
                    name: "remark".into(),
                    field_type: "text".into(),
                    nullable: true,
                    comment: "备注".into(),
                    search: false,
                    search_type: String::new(),
                    require: false,
                    default_value: String::new(),
                    form: true,
                    table: false,
                    desc: true,
                    sort: false,
                    primary_key: false,
                    enum_values: String::new(),
                },
            ],
        },
        FieldTemplate {
            name: "common",
            label: "常用业务字段 (名称+编码+描述)",
            fields: vec![
                GeneratorField {
                    name: "name".into(),
                    field_type: "string".into(),
                    nullable: false,
                    comment: "名称".into(),
                    search: true,
                    search_type: "like".into(),
                    require: true,
                    default_value: String::new(),
                    form: true,
                    table: true,
                    desc: true,
                    sort: false,
                    primary_key: false,
                    enum_values: String::new(),
                },
                GeneratorField {
                    name: "code".into(),
                    field_type: "string".into(),
                    nullable: false,
                    comment: "编码".into(),
                    search: true,
                    search_type: "eq".into(),
                    require: true,
                    default_value: String::new(),
                    form: true,
                    table: true,
                    desc: true,
                    sort: false,
                    primary_key: false,
                    enum_values: String::new(),
                },
                GeneratorField {
                    name: "description".into(),
                    field_type: "text".into(),
                    nullable: true,
                    comment: "描述".into(),
                    search: false,
                    search_type: String::new(),
                    require: false,
                    default_value: String::new(),
                    form: true,
                    table: false,
                    desc: true,
                    sort: false,
                    primary_key: false,
                    enum_values: String::new(),
                },
            ],
        },
    ]
}
