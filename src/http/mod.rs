use gloo_net::http::Request;

use crate::config::BASE_URL;
use crate::models::common::R;
use crate::storage;

#[derive(Debug)]
pub enum ApiError {
    Unauthorized(String),
    Forbidden(String),
    NotFound(String),
    ServerError(String),
    NetworkError(String),
    ParseError(String),
    EmptyResponse,
    BusinessError(String),
}

impl From<ApiError> for String {
    fn from(err: ApiError) -> Self {
        match err {
            ApiError::Unauthorized(msg) => msg,
            ApiError::Forbidden(msg) => msg,
            ApiError::NotFound(msg) => msg,
            ApiError::ServerError(msg) => msg,
            ApiError::NetworkError(msg) => msg,
            ApiError::ParseError(msg) => msg,
            ApiError::EmptyResponse => "服务器返回空响应".to_string(),
            ApiError::BusinessError(msg) => msg,
        }
    }
}

pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

fn build_url(path: &str) -> String {
    format!("{}{}", BASE_URL, path)
}

fn add_auth_header(builder: gloo_net::http::RequestBuilder) -> gloo_net::http::RequestBuilder {
    if let Some(token) = storage::get_token() {
        builder.header("Authorization", &format!("Bearer {}", token))
    } else {
        builder
    }
}

fn build_request(method: Method, url: &str) -> gloo_net::http::RequestBuilder {
    match method {
        Method::Get => Request::get(url),
        Method::Post => Request::post(url),
        Method::Put => Request::put(url),
        Method::Delete => Request::delete(url),
    }
}

async fn request<T: serde::de::DeserializeOwned, B: serde::Serialize>(
    method: Method,
    path: &str,
    body: Option<&B>,
) -> Result<T, ApiError> {
    let url = build_url(path);

    let resp = if let Some(data) = body {
        let body_str = serde_json::to_string(data).map_err(|e| ApiError::ParseError(e.to_string()))?;
        add_auth_header(build_request(method, &url))
            .header("Content-Type", "application/json")
            .body(body_str)
            .map_err(|e| ApiError::ParseError(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("请求失败: {}", e)))?
    } else {
        add_auth_header(build_request(method, &url))
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("请求失败: {}", e)))?
    };

    if resp.status() == 401 {
        storage::clear_token();
        return Err(ApiError::Unauthorized("未授权，请重新登录".into()));
    }
    if resp.status() == 403 {
        return Err(ApiError::Forbidden("禁止访问".into()));
    }
    if resp.status() == 404 {
        return Err(ApiError::NotFound(format!("API 不存在: {}", url)));
    }
    if resp.status() >= 500 {
        return Err(ApiError::ServerError(format!("服务器错误: HTTP {}", resp.status())));
    }

    let text = resp.text().await.map_err(|e| ApiError::NetworkError(format!("读取响应失败: {}", e)))?;
    if text.is_empty() {
        return Err(ApiError::EmptyResponse);
    }

    let body: R<T> = serde_json::from_str(&text).map_err(|e| ApiError::ParseError(format!("解析响应失败: {} - 原始响应: {}", e, text)))?;
    if body.code == 200 {
        body.data.ok_or_else(|| ApiError::BusinessError("响应数据为空".to_string()))
    } else {
        Err(ApiError::BusinessError(body.message))
    }
}

async fn request_void<B: serde::Serialize>(
    method: Method,
    path: &str,
    body: Option<&B>,
) -> Result<(), ApiError> {
    let url = build_url(path);

    let resp = if let Some(data) = body {
        let body_str = serde_json::to_string(data).map_err(|e| ApiError::ParseError(e.to_string()))?;
        add_auth_header(build_request(method, &url))
            .header("Content-Type", "application/json")
            .body(body_str)
            .map_err(|e| ApiError::ParseError(e.to_string()))?
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("请求失败: {}", e)))?
    } else {
        add_auth_header(build_request(method, &url))
            .send()
            .await
            .map_err(|e| ApiError::NetworkError(format!("请求失败: {}", e)))?
    };

    if resp.status() == 401 {
        storage::clear_token();
        return Err(ApiError::Unauthorized("未授权，请重新登录".into()));
    }
    if resp.status() == 403 {
        return Err(ApiError::Forbidden("禁止访问".into()));
    }
    if resp.status() == 404 {
        return Err(ApiError::NotFound(format!("API 不存在: {}", url)));
    }
    if resp.status() >= 500 {
        return Err(ApiError::ServerError(format!("服务器错误: HTTP {}", resp.status())));
    }

    let text = resp.text().await.map_err(|e| ApiError::NetworkError(format!("读取响应失败: {}", e)))?;
    if text.is_empty() {
        return Err(ApiError::EmptyResponse);
    }

    let body: R<serde_json::Value> = serde_json::from_str(&text).map_err(|e| ApiError::ParseError(format!("解析响应失败: {} - 原始响应: {}", e, text)))?;
    if body.code == 200 {
        Ok(())
    } else {
        Err(ApiError::BusinessError(body.message))
    }
}

pub async fn get<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, String> {
    request::<T, ()>(Method::Get, path, None).await.map_err(Into::into)
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
    request::<T, ()>(Method::Get, &url, None).await.map_err(Into::into)
}

pub async fn post<T: serde::de::DeserializeOwned, B: serde::Serialize>(
    path: &str,
    body: &B,
) -> Result<T, String> {
    request::<T, B>(Method::Post, path, Some(body)).await.map_err(Into::into)
}

pub async fn put<T: serde::de::DeserializeOwned, B: serde::Serialize>(
    path: &str,
    body: &B,
) -> Result<T, String> {
    request::<T, B>(Method::Put, path, Some(body)).await.map_err(Into::into)
}

pub async fn delete<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, String> {
    request::<T, ()>(Method::Delete, path, None).await.map_err(Into::into)
}

/// 发送 POST 请求，不期望返回数据（后端返回 R<()> 时 data 为 null）
pub async fn post_void<B: serde::Serialize>(path: &str, body: &B) -> Result<(), String> {
    request_void(Method::Post, path, Some(body)).await.map_err(Into::into)
}

/// 发送 DELETE 请求，不期望返回数据
pub async fn delete_void(path: &str) -> Result<(), String> {
    request_void::<()>(Method::Delete, path, None).await.map_err(Into::into)
}

/// 发送 PUT 请求，不期望返回数据（后端返回 R<()> 时 data 为 null）
pub async fn put_void<B: serde::Serialize>(path: &str, body: &B) -> Result<(), String> {
    request_void(Method::Put, path, Some(body)).await.map_err(Into::into)
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
