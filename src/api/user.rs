use crate::http::{build_page_query, delete_void, get, get_with_query, post_void, put, put_void};
use crate::models::common::PageResponse;
use crate::models::user::{SysUser, SysUserInsertDTO, SysUserUpdateDTO};
use serde::{Deserialize, Serialize};

pub async fn list(
    page: Option<u32>,
    page_size: Option<u32>,
    keyword: Option<&str>,
) -> Result<PageResponse<SysUser>, String> {
    let query = build_page_query(page, page_size, keyword);
    get_with_query("/api/user/list", &query).await
}

#[allow(dead_code)]
pub async fn get_by_id(id: i32) -> Result<SysUser, String> {
    get_with_query(&format!("/api/user/{}", id), "").await
}

pub async fn create(data: SysUserInsertDTO) -> Result<(), String> {
    post_void("/api/user/register", &data).await
}

pub async fn update(id: i32, data: SysUserUpdateDTO) -> Result<SysUser, String> {
    put(&format!("/api/user/{}", id), &data).await
}

pub async fn delete_user(id: i32) -> Result<(), String> {
    delete_void(&format!("/api/user/{}", id)).await
}

/// 修改密码
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordDTO {
    pub old_password: String,
    pub new_password: String,
}

pub async fn change_password(old_password: String, new_password: String) -> Result<(), String> {
    let dto = ChangePasswordDTO { old_password, new_password };
    put_void("/api/user/change_password", &dto).await
}

/// 仪表盘统计数据
#[derive(Debug, Clone, Deserialize)]
pub struct DashboardStats {
    pub user_count: u64,
    pub role_count: u64,
    pub menu_count: u64,
    pub api_count: u64,
}

pub async fn dashboard_stats() -> Result<DashboardStats, String> {
    get("/api/dashboard/stats").await
}
