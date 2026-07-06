use serde::{Deserialize, Deserializer, Serialize};

/// 自定义反序列化函数，将整数 0/1 转换为布尔值
fn bool_from_int<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let value: Option<i32> = Option::deserialize(deserializer)?;
    Ok(value.map(|v| v != 0))
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SysUser {
    pub id: i32,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub nick_name: Option<String>,
    #[serde(default)]
    pub phone: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub header_img: Option<String>,
    #[serde(default)]
    pub side_mode: Option<String>,
    #[serde(default, deserialize_with = "bool_from_int")]
    pub enable: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUserInsertDTO {
    pub username: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysUserUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_img: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable: Option<bool>,
}
