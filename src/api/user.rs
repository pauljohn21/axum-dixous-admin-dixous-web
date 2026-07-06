use crate::http::{build_page_query, delete, get_with_query, post, put};
use crate::models::common::PageResponse;
use crate::models::user::{SysUser, SysUserInsertDTO, SysUserUpdateDTO};

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
