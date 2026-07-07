use serde::{Deserialize, Serialize};

/// 统一响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

/// 分页请求
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(10),
            keyword: None,
        }
    }
}

/// 分页响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    pub list: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}
