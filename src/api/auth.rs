use crate::http::{get, post};
use crate::models::auth::{LoginDTO, LoginResp, UserInfoResp};

/// 登录
pub async fn login(username: String, password: String) -> Result<LoginResp, String> {
    let dto = LoginDTO { username, password };
    post("/api/user/login", &dto).await
}

/// 获取当前用户信息（含菜单）
pub async fn get_user_info() -> Result<UserInfoResp, String> {
    get("/api/user/info").await
}
