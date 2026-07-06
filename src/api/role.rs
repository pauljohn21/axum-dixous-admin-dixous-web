use crate::http::{build_page_query, delete, get_with_query, post, put};
use crate::models::common::PageResponse;
use crate::models::role::{SysRole, SysRoleInsertDTO, SysRoleUpdateDTO};

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
