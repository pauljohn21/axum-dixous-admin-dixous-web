use gloo_net::http::Request;

use crate::config::BASE_URL;
use crate::models::common::R;
use crate::storage;

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

pub async fn get<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, String> {
    let url = build_url(path);
    let builder = add_auth_header(Request::get(&url));
    let resp = builder.send().await.map_err(|e| format!("请求失败: {}", e))?;
    if resp.status() == 401 {
        storage::clear_token();
        return Err("未授权，请重新登录".into());
    }
    if resp.status() == 404 {
        return Err(format!("API 不存在: {}", url));
    }
    if resp.status() >= 500 {
        return Err(format!("服务器错误: HTTP {}", resp.status()));
    }
    let text = resp.text().await.map_err(|e| format!("读取响应失败: {}", e))?;
    if text.is_empty() {
        return Err("服务器返回空响应".to_string());
    }
    let body: R<T> = serde_json::from_str(&text).map_err(|e| format!("解析响应失败: {} - 原始响应: {}", e, text))?;
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
    let resp = builder.send().await.map_err(|e| format!("请求失败: {}", e))?;
    if resp.status() == 401 {
        storage::clear_token();
        return Err("未授权，请重新登录".into());
    }
    if resp.status() == 404 {
        return Err(format!("API 不存在: {}", url));
    }
    if resp.status() >= 500 {
        return Err(format!("服务器错误: HTTP {}", resp.status()));
    }
    let text = resp.text().await.map_err(|e| format!("读取响应失败: {}", e))?;
    if text.is_empty() {
        return Err("服务器返回空响应".to_string());
    }
    let body: R<T> = serde_json::from_str(&text).map_err(|e| format!("解析响应失败: {} - 原始响应: {}", e, text))?;
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
    let resp = request.send().await.map_err(|e| format!("请求失败: {}", e))?;
    if resp.status() == 401 {
        storage::clear_token();
        return Err("未授权，请重新登录".into());
    }
    if resp.status() == 404 {
        return Err(format!("API 不存在: {}", url));
    }
    if resp.status() >= 500 {
        return Err(format!("服务器错误: HTTP {}", resp.status()));
    }
    let text = resp.text().await.map_err(|e| format!("读取响应失败: {}", e))?;
    if text.is_empty() {
        return Err("服务器返回空响应".to_string());
    }
    let body: R<T> = serde_json::from_str(&text).map_err(|e| format!("解析响应失败: {} - 原始响应: {}", e, text))?;
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
    let resp = request.send().await.map_err(|e| format!("请求失败: {}", e))?;
    if resp.status() == 401 {
        storage::clear_token();
        return Err("未授权，请重新登录".into());
    }
    if resp.status() == 404 {
        return Err(format!("API 不存在: {}", url));
    }
    if resp.status() >= 500 {
        return Err(format!("服务器错误: HTTP {}", resp.status()));
    }
    let text = resp.text().await.map_err(|e| format!("读取响应失败: {}", e))?;
    if text.is_empty() {
        return Err("服务器返回空响应".to_string());
    }
    let body: R<T> = serde_json::from_str(&text).map_err(|e| format!("解析响应失败: {} - 原始响应: {}", e, text))?;
    if body.code == 200 {
        body.data.ok_or_else(|| "响应数据为空".to_string())
    } else {
        Err(body.message)
    }
}

pub async fn delete<T: serde::de::DeserializeOwned>(path: &str) -> Result<T, String> {
    let url = build_url(path);
    let builder = add_auth_header(Request::delete(&url));
    let resp = builder.send().await.map_err(|e| format!("请求失败: {}", e))?;
    if resp.status() == 401 {
        storage::clear_token();
        return Err("未授权，请重新登录".into());
    }
    if resp.status() == 404 {
        return Err(format!("API 不存在: {}", url));
    }
    if resp.status() >= 500 {
        return Err(format!("服务器错误: HTTP {}", resp.status()));
    }
    let text = resp.text().await.map_err(|e| format!("读取响应失败: {}", e))?;
    if text.is_empty() {
        return Err("服务器返回空响应".to_string());
    }
    let body: R<T> = serde_json::from_str(&text).map_err(|e| format!("解析响应失败: {} - 原始响应: {}", e, text))?;
    if body.code == 200 {
        body.data.ok_or_else(|| "响应数据为空".to_string())
    } else {
        Err(body.message)
    }
}

/// 发送 POST 请求，不期望返回数据（后端返回 R<()> 时 data 为 null）
pub async fn post_void<B: serde::Serialize>(path: &str, body: &B) -> Result<(), String> {
    let body_str = serde_json::to_string(body).map_err(|e| e.to_string())?;
    let url = build_url(path);
    let builder = add_auth_header(
        Request::post(&url).header("Content-Type", "application/json")
    );
    let request = builder.body(body_str).map_err(|e| e.to_string())?;
    let resp = request.send().await.map_err(|e| format!("请求失败: {}", e))?;
    if resp.status() == 401 {
        storage::clear_token();
        return Err("未授权，请重新登录".into());
    }
    if resp.status() == 404 {
        return Err(format!("API 不存在: {}", url));
    }
    if resp.status() >= 500 {
        return Err(format!("服务器错误: HTTP {}", resp.status()));
    }
    let text = resp.text().await.map_err(|e| format!("读取响应失败: {}", e))?;
    if text.is_empty() {
        return Err("服务器返回空响应".to_string());
    }
    let body: R<serde_json::Value> = serde_json::from_str(&text).map_err(|e| format!("解析响应失败: {} - 原始响应: {}", e, text))?;
    if body.code == 200 {
        Ok(())
    } else {
        Err(body.message)
    }
}

/// 发送 DELETE 请求，不期望返回数据
pub async fn delete_void(path: &str) -> Result<(), String> {
    let url = build_url(path);
    let builder = add_auth_header(Request::delete(&url));
    let resp = builder.send().await.map_err(|e| format!("请求失败: {}", e))?;
    if resp.status() == 401 {
        storage::clear_token();
        return Err("未授权，请重新登录".into());
    }
    if resp.status() == 404 {
        return Err(format!("API 不存在: {}", url));
    }
    if resp.status() >= 500 {
        return Err(format!("服务器错误: HTTP {}", resp.status()));
    }
    let text = resp.text().await.map_err(|e| format!("读取响应失败: {}", e))?;
    if text.is_empty() {
        return Err("服务器返回空响应".to_string());
    }
    let body: R<serde_json::Value> = serde_json::from_str(&text).map_err(|e| format!("解析响应失败: {} - 原始响应: {}", e, text))?;
    if body.code == 200 {
        Ok(())
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
