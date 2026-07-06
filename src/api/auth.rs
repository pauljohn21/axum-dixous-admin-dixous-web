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

/// 菜单模型 — 对应后端 sys_menu::Model
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SysMenu {
    pub id: i32,
    #[serde(default)]
    pub menu_level: Option<u64>,
    #[serde(default)]
    pub parent_id: Option<u64>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub hidden: Option<u8>,
    #[serde(default)]
    pub component: Option<String>,
    #[serde(default)]
    pub sort: Option<i64>,
    #[serde(default)]
    pub active_name: Option<String>,
    #[serde(default)]
    pub keep_alive: Option<i8>,
    #[serde(default)]
    pub default_menu: Option<i8>,
    #[serde(default)]
    pub title: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub close_tab: Option<i8>,
}

/// 菜单树节点 — 带子菜单
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MenuTreeNode {
    #[serde(flatten)]
    pub menu: SysMenu,
    pub children: Vec<MenuTreeNode>,
}

/// 用户信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfoResp {
    pub username: String,
    #[serde(default)]
    pub menus: Vec<SysMenu>,
}

/// 将扁平菜单列表构建为树形结构
/// parent_id 为 0 或 None 的菜单作为根节点
pub fn build_menu_tree(menus: &[SysMenu]) -> Vec<MenuTreeNode> {
    use std::collections::HashMap;

    // 1. 按 id 建索引
    let mut node_map: HashMap<i32, MenuTreeNode> = menus
        .iter()
        .map(|m| (m.id, MenuTreeNode { menu: m.clone(), children: vec![] }))
        .collect();

    // 2. 收集所有子节点到对应父节点
    let mut roots: Vec<MenuTreeNode> = Vec::new();
    let child_ids: Vec<i32> = node_map.keys().copied().collect();

    for &id in &child_ids {
        if let Some(node) = node_map.get(&id) {
            let parent_id = node.menu.parent_id.unwrap_or(0);
            if parent_id == 0 {
                // 根节点
                continue;
            }
            // 有父节点 — 添加到父节点的 children
            if node_map.contains_key(&(parent_id as i32)) {
                // 先取出当前节点，避免借用冲突
                if let Some(child) = node_map.get_mut(&id) {
                    let child_clone = child.clone();
                    if let Some(parent) = node_map.get_mut(&(parent_id as i32)) {
                        parent.children.push(child_clone);
                    }
                }
            } else {
                // 父节点不在列表中，当作根节点
            }
        }
    }

    // 3. 收集根节点（parent_id 为 0 或父节点不在列表中的）
    for &id in &child_ids {
        if let Some(node) = node_map.get(&id) {
            let parent_id = node.menu.parent_id.unwrap_or(0);
            if parent_id == 0 || !node_map.contains_key(&(parent_id as i32)) {
                if let Some(root) = node_map.get(&id) {
                    roots.push(root.clone());
                }
            }
        }
    }

    // 4. 按 sort 排序
    roots.sort_by_key(|n| n.menu.sort.unwrap_or(0));
    for root in &mut roots {
        sort_children(root);
    }

    roots
}

fn sort_children(node: &mut MenuTreeNode) {
    node.children.sort_by_key(|c| c.menu.sort.unwrap_or(0));
    for child in &mut node.children {
        sort_children(child);
    }
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
