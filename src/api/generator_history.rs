//! 代码生成器历史 API 封装

use crate::http::{build_page_query, delete_void, get, get_with_query, post, post_void};
use crate::models::generator_history::*;
use crate::models::common::PageResponse;

/// 获取历史记录列表
pub async fn list(page: Option<u32>, page_size: Option<u32>, keyword: Option<&str>) -> Result<PageResponse<GeneratorHistory>, String> {
    let query = build_page_query(page, page_size, keyword);
    get_with_query("/api/generator/history/list", &query).await
}

/// 获取单条历史记录
#[allow(dead_code)]
pub async fn get_by_id(id: u64) -> Result<GeneratorHistory, String> {
    get(&format!("/api/generator/history/{}", id)).await
}

/// 获取历史记录的 JSON 配置
#[allow(dead_code)]
pub async fn get_meta(id: u64) -> Result<String, String> {
    get(&format!("/api/generator/history/{}/meta", id)).await
}

/// 创建历史记录
#[allow(dead_code)]
pub async fn create(data: CreateHistoryRequest) -> Result<GeneratorHistory, String> {
    post("/api/generator/history", &data).await
}

/// 删除历史记录
pub async fn delete(id: u64) -> Result<(), String> {
    delete_void(&format!("/api/generator/history/{}", id)).await
}

/// 回滚
pub async fn rollback(id: u64, delete_table: bool) -> Result<(), String> {
    let req = RollbackRequest { id, delete_table };
    post_void("/api/generator/rollback", &req).await
}

/// 获取所有数据库名
pub async fn get_databases() -> Result<Vec<DatabaseInfo>, String> {
    get("/api/generator/databases").await
}

/// 获取指定数据库的表名
pub async fn get_tables(db_name: &str) -> Result<Vec<TableInfo>, String> {
    get_with_query("/api/generator/tables", &format!("db_name={}", urlencoding::encode(db_name))).await
}

/// 获取表字段信息
#[allow(dead_code)]
pub async fn get_columns(db_name: &str, table_name: &str) -> Result<Vec<ColumnInfo>, String> {
    let query = format!("db_name={}&table_name={}", urlencoding::encode(db_name), urlencoding::encode(table_name));
    get_with_query("/api/generator/columns", &query).await
}

/// 根据数据库表结构生成 JSON 配置
pub async fn generate_from_table(db_name: &str, table_name: &str) -> Result<String, String> {
    let req = GenerateFromTableRequest {
        db_name: db_name.to_string(),
        table_name: table_name.to_string(),
    };
    post("/api/generator/generate-from-table", &req).await
}
