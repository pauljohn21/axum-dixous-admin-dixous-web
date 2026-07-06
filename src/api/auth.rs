use serde::{Deserialize, Serialize};

use super::{get, post};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginDTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResp {
    pub token: String,
}

/// 菜单模型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SysMenu {
    pub id: i32,
    #[serde(default)]
    pub parent_id: Option<i32>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub component: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub sort: Option<i32>,
    #[serde(default)]
    pub hidden: Option<bool>,
    #[serde(default)]
    pub title: Option<String>,
}

/// 用户信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoResp {
    pub username: String,
    #[serde(default)]
    pub menus: Vec<SysMenu>,
}

/// 登录
pub async fn login(username: String, password: String) -> Result<LoginResp, String> {
    let dto = LoginDTO { username, password };
    post("/api/user/login", &dto).await
}

/// 获取当前用户信息（含菜单）
pub async fn get_user_info() -> Result<UserInfoResp, String> {
    get("/api/user/info").await
}
