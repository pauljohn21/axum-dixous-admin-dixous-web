pub mod auth;
pub mod user;
pub mod role;
pub mod menu;
pub mod sys_api;
pub mod dictionary;

use serde::{Deserialize, Serialize};
use gloo_net::http::Request;

const BASE_URL: &str = "http://localhost:8888";

/// 统一响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct R<T> {
    pub code: u16,
    pub message: String,
    pub data: Option<T>,
}

/// 分页请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            page: Some(1),
            page_size: Some(10),
            keyword: None,
        }
    }
}

/// 分页响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageResponse<T> {
    pub list: Vec<T>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
}

// ===== Token 管理（内存，不使用 localStorage） =====
//
// 使用 thread_local + RefCell 存储在内存中：
// - XSS 攻击无法通过 JS 读取内存中的 token（localStorage 可被任意 JS 读取）
// - 页面刷新后 token 丢失，用户需重新登录，符合安全最佳实践

thread_local! {
    static TOKEN: std::cell::RefCell<Option<String>> = const { std::cell::RefCell::new(None) };
}

/// 获取当前 token
pub fn get_token() -> Option<String> {
    TOKEN.with(|t| t.borrow().clone())
}

/// 设置 token
pub fn set_token(token: &str) {
    TOKEN.with(|t| *t.borrow_mut() = Some(token.to_string()));
}

/// 清除 token
pub fn clear_token() {
    TOKEN.with(|t| *t.borrow_mut() = None);
}

// ===== HTTP 请求封装 =====

fn build_url(path: &str) -> String {
    format!("{}{}", BASE_URL, path)
}

fn add_auth_header(builder: gloo_net::http::RequestBuilder) -> gloo_net::http::RequestBuilder {
    if let Some(token) = get_token() {
        builder.header("Authorization", &format!("Bearer {}", token))
    } else {
        builder
    }
}

pub async fn get<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, String> {
    let url = build_url(path);
    let builder = add_auth_header(Request::get(&url));
    let resp = builder.send().await.map_err(|e| e.to_string())?;
    if resp.status() == 401 {
        clear_token();
        return Err("未授权，请重新登录".into());
    }
    let body: R<T> = resp.json().await.map_err(|e| e.to_string())?;
    if body.code == 200 {
        body.data.ok_or_else(|| "响应数据为空".to_string())
    } else {
        Err(body.message)
    }
}

pub async fn get_with_query<T: serde::de::DeserializeOwned>(
    path: &str,
    query: &str,
) -> Result<T, String> {
    let url = if query.is_empty() {
        build_url(path)
    } else {
        format!("{}?{}", build_url(path), query)
    };
    let builder = add_auth_header(Request::get(&url));
    let resp = builder.send().await.map_err(|e| e.to_string())?;
    if resp.status() == 401 {
        clear_token();
        return Err("未授权，请重新登录".into());
    }
    let body: R<T> = resp.json().await.map_err(|e| e.to_string())?;
    if body.code == 200 {
        body.data.ok_or_else(|| "响应数据为空".to_string())
    } else {
        Err(body.message)
    }
}

pub async fn post<T: serde::de::DeserializeOwned, B: serde::Serialize>(
    path: &str,
    body: &B,
) -> Result<T, String> {
    let body_str = serde_json::to_string(body).map_err(|e| e.to_string())?;
    let url = build_url(path);
    let builder = add_auth_header(
        Request::post(&url).header("Content-Type", "application/json")
    );
    let request = builder.body(body_str).map_err(|e| e.to_string())?;
    let resp = request.send().await.map_err(|e| e.to_string())?;
    if resp.status() == 401 {
        clear_token();
        return Err("未授权，请重新登录".into());
    }
    let body: R<T> = resp.json().await.map_err(|e| e.to_string())?;
    if body.code == 200 {
        body.data.ok_or_else(|| "响应数据为空".to_string())
    } else {
        Err(body.message)
    }
}

pub async fn put<T: serde::de::DeserializeOwned, B: serde::Serialize>(
    path: &str,
    body: &B,
) -> Result<T, String> {
    let body_str = serde_json::to_string(body).map_err(|e| e.to_string())?;
    let url = build_url(path);
    let builder = add_auth_header(
        Request::put(&url).header("Content-Type", "application/json")
    );
    let request = builder.body(body_str).map_err(|e| e.to_string())?;
    let resp = request.send().await.map_err(|e| e.to_string())?;
    if resp.status() == 401 {
        clear_token();
        return Err("未授权，请重新登录".into());
    }
    let body: R<T> = resp.json().await.map_err(|e| e.to_string())?;
    if body.code == 200 {
        body.data.ok_or_else(|| "响应数据为空".to_string())
    } else {
        Err(body.message)
    }
}

pub async fn delete<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, String> {
    let url = build_url(path);
    let builder = add_auth_header(Request::delete(&url));
    let resp = builder.send().await.map_err(|e| e.to_string())?;
    if resp.status() == 401 {
        clear_token();
        return Err("未授权，请重新登录".into());
    }
    let body: R<T> = resp.json().await.map_err(|e| e.to_string())?;
    if body.code == 200 {
        body.data.ok_or_else(|| "响应数据为空".to_string())
    } else {
        Err(body.message)
    }
}

pub fn build_page_query(page: Option<u32>, page_size: Option<u32>, keyword: Option<&str>) -> String {
    let mut params = Vec::new();
    if let Some(p) = page {
        params.push(format!("page={}", p));
    }
    if let Some(ps) = page_size {
        params.push(format!("page_size={}", ps));
    }
    if let Some(kw) = keyword {
        if !kw.is_empty() {
            params.push(format!("keyword={}", urlencoding::encode(kw)));
        }
    }
    params.join("&")
}
