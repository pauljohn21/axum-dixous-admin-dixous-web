use serde::{Deserialize, Serialize};

use super::menu::SysMenu;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginDTO {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResp {
    pub token: String,
}

/// 用户信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoResp {
    pub username: String,
    #[serde(default)]
    pub menus: Vec<SysMenu>,
}
