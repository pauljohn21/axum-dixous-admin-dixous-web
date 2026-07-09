//! 代码生成器历史记录前端数据模型

use serde::{Deserialize, Serialize};

/// 历史记录
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GeneratorHistory {
    pub id: u64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub table_name: String,
    pub resource: String,
    pub module_cn: String,
    pub request: String,
    pub flag: i32,
    pub generated_files: Option<String>,
}

/// 创建历史记录请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateHistoryRequest {
    pub table_name: String,
    pub resource: String,
    pub module_cn: String,
    pub request: String,
    pub generated_files: Option<String>,
}

/// 回滚请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackRequest {
    pub id: u64,
    pub delete_table: bool,
}

/// 数据库信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseInfo {
    pub database: String,
}

/// 表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableInfo {
    pub table_name: String,
}

/// 字段信息
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnInfo {
    pub column_name: String,
    pub data_type: String,
    pub data_type_long: String,
    pub column_comment: String,
    pub primary_key: bool,
    pub ordinal_position: i32,
}

/// 从数据库生成 YAML 的请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateFromTableRequest {
    pub db_name: String,
    pub table_name: String,
}

/// 生成的代码文件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneratedFile {
    pub file_name: String,
    pub file_path: String,
    pub content: String,
    pub file_type: String,
}

/// 代码预览响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreviewCodeResponse {
    pub backend_files: Vec<GeneratedFile>,
    pub frontend_files: Vec<GeneratedFile>,
}
