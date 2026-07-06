use serde::{Deserialize, Serialize};

use super::{get_with_query, post, put, delete, build_page_query, PageResponse};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SysRole {
    pub id: i32,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub keyword: Option<String>,
    #[serde(default)]
    pub desc: Option<String>,
    #[serde(default)]
    pub sort: Option<i32>,
    #[serde(default)]
    pub status: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysRoleInsertDTO {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysRoleUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

pub async fn list(
    page: Option<u32>,
    page_size: Option<u32>,
    keyword: Option<&str>,
) -> Result<PageResponse<SysRole>, String> {
    let query = build_page_query(page, page_size, keyword);
    get_with_query("/api/role/list", &query).await
}

pub async fn create(data: SysRoleInsertDTO) -> Result<(), String> {
    post("/api/role", &data).await
}

pub async fn update(id: i32, data: SysRoleUpdateDTO) -> Result<SysRole, String> {
    put(&format!("/api/role/{}", id), &data).await
}

pub async fn delete_role(id: i32) -> Result<(), String> {
    delete(&format!("/api/role/{}", id)).await
}
