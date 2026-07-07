use serde::{Deserialize, Serialize};

/// 角色 (与后端 sys_role::Model 对应)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SysRole {
    pub id: i32,
    #[serde(default)]
    pub en_name: Option<String>,
    #[serde(default)]
    pub cn_name: Option<String>,
    #[serde(default)]
    pub parent_id: Option<u64>,
    #[serde(default)]
    pub created_ad: Option<String>,
    #[serde(default)]
    pub updated_ad: Option<String>,
}

/// 新增角色 DTO (与后端 SysRoleInsertDTO 对应)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysRoleInsertDTO {
    pub en_name: String,
    pub cn_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<u64>,
}

/// 更新角色 DTO (与后端 SysRoleUpdateDTO 对应)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysRoleUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<u64>,
}
