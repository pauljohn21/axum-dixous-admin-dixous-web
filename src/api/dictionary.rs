use crate::http::{build_page_query, delete, get_with_query, post, put};
use crate::models::common::PageResponse;
use crate::models::dictionary::{SysDictionary, SysDictionaryInsertDTO, SysDictionaryUpdateDTO};

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
