use serde::{Deserialize, Serialize};

use super::{get_with_query, post, put, delete, build_page_query, PageResponse};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SysApi {
    pub id: i32,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub method: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysApiInsertDTO {
    pub path: String,
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysApiUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

pub async fn list(
    page: Option<u32>,
    page_size: Option<u32>,
    keyword: Option<&str>,
) -> Result<PageResponse<SysApi>, String> {
    let query = build_page_query(page, page_size, keyword);
    get_with_query("/api/api/list", &query).await
}

pub async fn create(data: SysApiInsertDTO) -> Result<(), String> {
    post("/api/api", &data).await
}

pub async fn update(id: i32, data: SysApiUpdateDTO) -> Result<SysApi, String> {
    put(&format!("/api/api/{}", id), &data).await
}

pub async fn delete_api(id: i32) -> Result<(), String> {
    delete(&format!("/api/api/{}", id)).await
}
