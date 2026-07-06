use crate::http::{build_page_query, delete, get_with_query, post, put};
use crate::models::common::PageResponse;
use crate::models::menu::{SysMenu, SysMenuInsertDTO, SysMenuUpdateDTO};

pub async fn list(
    page: Option<u32>,
    page_size: Option<u32>,
    keyword: Option<&str>,
) -> Result<PageResponse<SysMenu>, String> {
    let query = build_page_query(page, page_size, keyword);
    get_with_query("/api/menu/list", &query).await
}

pub async fn create(data: SysMenuInsertDTO) -> Result<(), String> {
    post("/api/menu", &data).await
}

pub async fn update(id: i32, data: SysMenuUpdateDTO) -> Result<SysMenu, String> {
    put(&format!("/api/menu/{}", id), &data).await
}

pub async fn delete_menu(id: i32) -> Result<(), String> {
    delete(&format!("/api/menu/{}", id)).await
}
