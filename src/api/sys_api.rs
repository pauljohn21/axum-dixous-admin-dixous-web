use crate::http::{build_page_query, delete, get_with_query, post, put};
use crate::models::common::PageResponse;
use crate::models::sys_api::{SysApi, SysApiInsertDTO, SysApiUpdateDTO};

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
