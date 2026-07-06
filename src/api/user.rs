use serde::{Deserialize, Serialize};

use super::{get_with_query, post, put, delete, build_page_query, PageResponse};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SysUser {
    pub id: i32,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub nick_name: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub header_img: Option<String>,
    #[serde(default)]
    pub side_mode: Option<String>,
    #[serde(default)]
    pub enable: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUserInsertDTO {
    pub username: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUserUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_img: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}

pub async fn list(
    page: Option<u32>,
    page_size: Option<u32>,
    keyword: Option<&str>,
) -> Result<PageResponse<SysUser>, String> {
    let query = build_page_query(page, page_size, keyword);
    get_with_query("/api/user/list", &query).await
}

pub async fn get_by_id(id: i32) -> Result<SysUser, String> {
    get_with_query(&format!("/api/user/{}", id), "").await
}

pub async fn create(data: SysUserInsertDTO) -> Result<(), String> {
    post("/api/user/register", &data).await
}

pub async fn update(id: i32, data: SysUserUpdateDTO) -> Result<SysUser, String> {
    put(&format!("/api/user/{}", id), &data).await
}

pub async fn delete_user(id: i32) -> Result<(), String> {
    delete(&format!("/api/user/{}", id)).await
}
