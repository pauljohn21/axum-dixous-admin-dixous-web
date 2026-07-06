use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysMenuInsertDTO {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SysMenuUpdateDTO {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub component: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
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
                continue;
            }
            if node_map.contains_key(&(parent_id as i32)) {
                if let Some(child) = node_map.get_mut(&id) {
                    let child_clone = child.clone();
                    if let Some(parent) = node_map.get_mut(&(parent_id as i32)) {
                        parent.children.push(child_clone);
                    }
                }
            }
        }
    }

    // 3. 收集根节点
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
