use web_sys::Storage;

use crate::config::{TOKEN_KEY, USERNAME_KEY};

/// 获取 localStorage 对象
fn local_storage() -> Option<Storage> {
    let window = web_sys::window()?;
    window.local_storage().ok()?
}

/// 存储数据到 localStorage
pub fn set(key: &str, value: &str) {
    if let Some(storage) = local_storage() {
        let _ = storage.set_item(key, value);
    }
}

/// 从 localStorage 读取数据
pub fn get(key: &str) -> Option<String> {
    local_storage()?.get_item(key).ok()?
}

/// 从 localStorage 移除指定 key
pub fn remove(key: &str) {
    if let Some(storage) = local_storage() {
        let _ = storage.remove_item(key);
    }
}

/// 清空 localStorage
pub fn clear() {
    if let Some(storage) = local_storage() {
        let _ = storage.clear();
    }
}

// ===== Token 便捷方法 =====

/// 获取当前 token
pub fn get_token() -> Option<String> {
    get(TOKEN_KEY)
}

/// 设置 token
pub fn set_token(token: &str) {
    set(TOKEN_KEY, token);
}

/// 清除 token
pub fn clear_token() {
    remove(TOKEN_KEY);
}

// ===== Username 便捷方法 =====

/// 获取当前用户名
pub fn get_username() -> Option<String> {
    get(USERNAME_KEY)
}

/// 设置用户名
pub fn set_username(username: &str) {
    set(USERNAME_KEY, username);
}

/// 清除用户名
pub fn clear_username() {
    remove(USERNAME_KEY);
}
