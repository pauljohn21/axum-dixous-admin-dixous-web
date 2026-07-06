use serde::{Deserialize, Serialize};

use super::{get_with_query, post, put, delete, build_page_query, PageResponse};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SysDictionary {
    pub id: i32,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(default)]
    pub status: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysDictionaryInsertDTO {
    pub name: String,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysDictionaryUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

pub async fn list(
    page: Option<u32>,
    page_size: Option<u32>,
    keyword: Option<&str>,
) -> Result<PageResponse<SysDictionary>, String> {
    let query = build_page_query(page, page_size, keyword);
    get_with_query("/api/dictionary/list", &query).await
}

pub async fn create(data: SysDictionaryInsertDTO) -> Result<(), String> {
    post("/api/dictionary", &data).await
}

pub async fn update(id: i32, data: SysDictionaryUpdateDTO) -> Result<SysDictionary, String> {
    put(&format!("/api/dictionary/{}", id), &data).await
}

pub async fn delete_dict(id: i32) -> Result<(), String> {
    delete(&format!("/api/dictionary/{}", id)).await
}
