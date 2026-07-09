//! 代码生成器 API 封装
//!
//! 通过后端数据库 API 保存/加载配置，不再使用 localStorage 和 YAML。

use crate::http;
use crate::models::generator::GeneratorConfig;
use crate::models::generator_history::{CreateHistoryRequest, GeneratorHistory, PreviewCodeResponse};

/// 保存配置到数据库 (创建历史记录)
pub async fn save_to_db(config: &GeneratorConfig) -> Result<GeneratorHistory, String> {
    let request = serde_json::to_string(config).map_err(|e| e.to_string())?;
    let data = CreateHistoryRequest {
        table_name: config.table_name.clone(),
        resource: config.resource.clone(),
        module_cn: config.module_cn.clone(),
        request,
        generated_files: None,
    };
    http::post("/api/generator/history", &data).await
}

/// 从数据库历史记录加载配置
#[allow(dead_code)]
pub async fn load_from_db(id: u64) -> Result<GeneratorConfig, String> {
    let record: GeneratorHistory = http::get(&format!("/api/generator/history/{}", id)).await?;
    serde_json::from_str(&record.request).map_err(|e| format!("JSON 解析失败: {}", e))
}

/// 预览代码 - 根据配置生成代码内容
pub async fn preview_code(config: &GeneratorConfig) -> Result<PreviewCodeResponse, String> {
    let config_json = serde_json::to_string(config).map_err(|e| e.to_string())?;
    let data = serde_json::json!({ "config_json": config_json });
    http::post("/api/generator/preview", &data).await
}
