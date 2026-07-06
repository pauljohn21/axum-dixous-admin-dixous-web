use dioxus::prelude::*;
use dioxus::router::use_route;

use crate::icons::element::*;
use crate::icons::{IconProps, IconSize};
use crate::models::menu::{MenuTreeNode, SysMenu};
use crate::router::Route;

/// 菜单项 CSS — 在 admin_layout 中注入全局
pub const MENU_CSS: &str = r#"
.sidebar-menu-item {
    display: flex;
    align-items: center;
    padding: 10px 20px;
    color: #bfcbd9;
    cursor: pointer;
    transition: all 0.2s;
    white-space: nowrap;
    border-left: 3px solid transparent;
    text-decoration: none;
    user-select: none;
}
.sidebar-menu-item:hover {
    background: #263445 !important;
    color: #fff !important;
}
.sidebar-menu-item-active {
    background: #1f2d3d !important;
    border-left-color: #409eff !important;
    color: #fff !important;
}
"#;

/// 将菜单 path 映射到 Route 枚举
fn menu_path_to_route(path: &str) -> Option<Route> {
    let path = path.trim_start_matches('/');
    match path {
        "" | "dashboard" => Some(Route::Dashboard {}),
        "user" | "users" => Some(Route::UserManage {}),
        "role" | "roles" => Some(Route::RoleManage {}),
        "menu" | "menus" => Some(Route::MenuManage {}),
        "api" | "apis" => Some(Route::ApiManage {}),
        "dictionary" | "dictionaries" => Some(Route::DictManage {}),
        _ => None,
    }
}

/// 判断某菜单的 route 是否与当前路由匹配
fn is_active_route(route: &Option<Route>, current: &Route) -> bool {
    if let Some(r) = route {
        r == current
    } else {
        false
    }
}

/// 递归检查子菜单中是否有匹配当前路由的
fn has_active_child(node: &MenuTreeNode, current: &Route) -> bool {
    for child in &node.children {
        let path = child.menu.path.clone().unwrap_or_default();
        if let Some(r) = menu_path_to_route(&path) {
            if r == *current {
                return true;
            }
        }
        if has_active_child(child, current) {
            return true;
        }
    }
    false
}

/// 图标组件类型别名
type IconComponent = fn(IconProps) -> Element;

/// 将 Element Plus 图标类名映射为对应的图标组件
///
/// 返回一个闭包，该闭包接受 IconProps 并返回 Element
fn map_icon_class_to_component(icon_class: &str) -> Option<IconComponent> {
    match icon_class {
        // 仪表盘/首页
        "odometer" | "speedometer" | "dashboard" => Some(Odometer as IconComponent),
        // 用户相关
        "user" => Some(User as IconComponent),
        "user-filled" => Some(UserFilled as IconComponent),
        "avatar" | "user-solid" | "peoples" => Some(UserFilled as IconComponent),
        // 角色/权限
        "lock" => Some(Lock as IconComponent),
        "key" => Some(Key as IconComponent),
        // 菜单
        "tickets" => Some(Tickets as IconComponent),
        "menu" => Some(Menu as IconComponent),
        "list" => Some(List as IconComponent),
        // API/接口
        "platform" => Some(Platform as IconComponent),
        "connection" => Some(Connection as IconComponent),
        // 字典/文档
        "dict" | "dictionary" | "book" | "document" => Some(Document as IconComponent),
        "folder" => Some(Folder as IconComponent),
        // 操作记录/历史
        "operation" | "history" | "time" | "pie-chart" => Some(PieChart as IconComponent),
        // 消息/通知
        "message" => Some(Message as IconComponent),
        "bell" => Some(Bell as IconComponent),
        "bell-filled" => Some(BellFilled as IconComponent),
        "notification" => Some(Notification as IconComponent),
        // 工具/设置
        "tools" | "tool" => Some(Tools as IconComponent),
        "setting" | "settings" => Some(Setting as IconComponent),
        // 系统配置
        "system" | "monitor" => Some(Monitor as IconComponent),
        "cloudy" => Some(Cloudy as IconComponent),
        // 关于/信息
        "info-filled" | "info" | "about" => Some(InfoFilled as IconComponent),
        // 搜索
        "search" => Some(Search as IconComponent),
        // 删除
        "delete" => Some(Delete as IconComponent),
        "delete-filled" => Some(DeleteFilled as IconComponent),
        // 编辑
        "edit" => Some(Edit as IconComponent),
        "edit-pen" => Some(EditPen as IconComponent),
        // 检查/确认
        "check" | "checked" => Some(Check as IconComponent),
        "success-filled" => Some(SuccessFilled as IconComponent),
        // 关闭
        "close" => Some(Close as IconComponent),
        "close-bold" => Some(CloseBold as IconComponent),
        "circle-close" => Some(CircleClose as IconComponent),
        "circle-close-filled" => Some(CircleCloseFilled as IconComponent),
        // 添加
        "plus" | "circle-plus" => Some(Plus as IconComponent),
        "circle-plus-filled" => Some(CirclePlusFilled as IconComponent),
        // 箭头
        "arrow-left" => Some(ArrowLeft as IconComponent),
        "arrow-right" => Some(ArrowRight as IconComponent),
        "arrow-up" => Some(ArrowUp as IconComponent),
        "arrow-down" => Some(ArrowDown as IconComponent),
        "arrow-left-bold" => Some(ArrowLeftBold as IconComponent),
        "arrow-right-bold" => Some(ArrowRightBold as IconComponent),
        "arrow-up-bold" => Some(ArrowUpBold as IconComponent),
        "arrow-down-bold" => Some(ArrowDownBold as IconComponent),
        "d-arrow-left" => Some(DArrowLeft as IconComponent),
        "d-arrow-right" => Some(DArrowRight as IconComponent),
        "caret-left" => Some(CaretLeft as IconComponent),
        "caret-right" => Some(CaretRight as IconComponent),
        "caret-top" => Some(CaretTop as IconComponent),
        "caret-bottom" => Some(CaretBottom as IconComponent),
        // 更多
        "more" => Some(More as IconComponent),
        "more-filled" => Some(MoreFilled as IconComponent),
        // 查看
        "view" => Some(View as IconComponent),
        "hide" => Some(Hide as IconComponent),
        // 上传/下载
        "upload" => Some(Upload as IconComponent),
        "upload-filled" => Some(UploadFilled as IconComponent),
        "download" => Some(Download as IconComponent),
        // 文件夹
        "folder-add" => Some(FolderAdd as IconComponent),
        "folder-delete" => Some(FolderDelete as IconComponent),
        "folder-opened" => Some(FolderOpened as IconComponent),
        // 文档
        "document-add" => Some(DocumentAdd as IconComponent),
        "document-delete" => Some(DocumentDelete as IconComponent),
        "document-copy" => Some(DocumentCopy as IconComponent),
        "document-checked" => Some(DocumentChecked as IconComponent),
        // 警告
        "warning" => Some(Warning as IconComponent),
        "warning-filled" | "warn-triangle-filled" => Some(WarningFilled as IconComponent),
        // 问题
        "question-filled" => Some(QuestionFilled as IconComponent),
        "circle-check" => Some(CircleCheck as IconComponent),
        "circle-check-filled" => Some(CircleCheckFilled as IconComponent),
        // 失败/移除
        "failed" => Some(Failed as IconComponent),
        "remove" => Some(Remove as IconComponent),
        "remove-filled" => Some(RemoveFilled as IconComponent),
        // 排序/排名
        "rank" => Some(Rank as IconComponent),
        "sort" => Some(Sort as IconComponent),
        "filter" => Some(Filter as IconComponent),
        // 分享
        "share" => Some(Share as IconComponent),
        // 打印
        "printer" => Some(Printer as IconComponent),
        // 附件
        "paperclip" => Some(Paperclip as IconComponent),
        // 链接
        "link" => Some(LinkIcon as IconComponent),
        // 日历
        "calendar" => Some(Calendar as IconComponent),
        // 时钟
        "clock" => Some(Clock as IconComponent),
        "timer" => Some(Timer as IconComponent),
        // 消息框
        "message-box" => Some(MessageBox as IconComponent),
        "chat-dot-round" => Some(ChatDotRound as IconComponent),
        "chat-line-round" => Some(ChatLineRound as IconComponent),
        // 数据
        "histogram" => Some(Histogram as IconComponent),
        "data-line" => Some(DataLine as IconComponent),
        "data-board" => Some(DataBoard as IconComponent),
        "data-analysis" => Some(DataAnalysis as IconComponent),
        "trend-charts" => Some(TrendCharts as IconComponent),
        // 财务
        "wallet" => Some(Wallet as IconComponent),
        "wallet-filled" => Some(WalletFilled as IconComponent),
        "money" => Some(Money as IconComponent),
        "coin" => Some(Coin as IconComponent),
        "credit-card" => Some(CreditCard as IconComponent),
        // 商品
        "goods" => Some(Goods as IconComponent),
        "goods-filled" => Some(GoodsFilled as IconComponent),
        "shopping-cart" => Some(ShoppingCart as IconComponent),
        "shopping-cart-full" => Some(ShoppingCartFull as IconComponent),
        "shop" => Some(Shop as IconComponent),
        "sell" => Some(Sell as IconComponent),
        "present" => Some(Present as IconComponent),
        // 盒子
        "box" => Some(BoxIcon as IconComponent),
        // 建筑
        "office-building" => Some(OfficeBuilding as IconComponent),
        "school" => Some(School as IconComponent),
        // 位置
        "location" => Some(Location as IconComponent),
        "location-filled" => Some(LocationFilled as IconComponent),
        "map-location" => Some(MapLocation as IconComponent),
        "place" => Some(Place as IconComponent),
        "coordinate" => Some(Coordinate as IconComponent),
        // 全屏
        "full-screen" => Some(FullScreen as IconComponent),
        "expand" => Some(Expand as IconComponent),
        "fold" => Some(Fold as IconComponent),
        "open" => Some(Open as IconComponent),
        // 开关
        "turn-off" => Some(TurnOff as IconComponent),
        "unlock" => Some(Unlock as IconComponent),
        "switch" => Some(Switch as IconComponent),
        "switch-button" => Some(SwitchButton as IconComponent),
        "switch-filled" => Some(SwitchFilled as IconComponent),
        // 刷新
        "refresh" => Some(Refresh as IconComponent),
        "refresh-left" => Some(RefreshLeft as IconComponent),
        "refresh-right" => Some(RefreshRight as IconComponent),
        // 方向
        "top" => Some(Top as IconComponent),
        "bottom" => Some(Bottom as IconComponent),
        "right" => Some(Right as IconComponent),
        "top-left" => Some(TopLeft as IconComponent),
        "top-right" => Some(TopRight as IconComponent),
        "bottom-left" => Some(BottomLeft as IconComponent),
        "bottom-right" => Some(BottomRight as IconComponent),
        // 缩放
        "zoom-in" => Some(ZoomIn as IconComponent),
        "zoom-out" => Some(ZoomOut as IconComponent),
        // 加载
        "loading" => Some(Loading as IconComponent),
        // 星星
        "star" => Some(Star as IconComponent),
        "star-filled" => Some(StarFilled as IconComponent),
        // 默认
        _ => None,
    }
}

/// 根据菜单信息获取对应的图标组件
///
/// 返回一个 Element，可以直接在 RSX 中使用
fn get_menu_icon_element(menu: &SysMenu) -> Element {
    // 优先使用后端返回的 icon 字段
    let icon_class = menu.icon.as_deref().filter(|s| !s.is_empty());

    let icon_fn = if let Some(icon) = icon_class {
        map_icon_class_to_component(icon)
    } else {
        // 根据 path 兜底
        let path = menu.path.as_deref().unwrap_or("");
        match path.trim_start_matches('/') {
            "" | "dashboard" => Some(Odometer as IconComponent),
            "user" | "users" => Some(User as IconComponent),
            "role" | "roles" => Some(Lock as IconComponent),
            "menu" | "menus" => Some(Menu as IconComponent),
            "api" | "apis" => Some(Connection as IconComponent),
            "dictionary" | "dictionaries" => Some(Document as IconComponent),
            _ => Some(Document as IconComponent),
        }
    };

    if let Some(icon_fn) = icon_fn {
        icon_fn(IconProps {
            size: IconSize::from(18),
            attributes: vec![],
        })
    } else {
        // 如果没有匹配的图标，返回一个默认的 Document 图标
        Document(IconProps {
            size: IconSize::from(18),
            attributes: vec![],
        })
    }
}

/// 菜单项组件 — 递归渲染菜单树
#[component]
pub fn MenuItem(
    node: MenuTreeNode,
    depth: usize,
    collapsed: bool,
    expanded_keys: Signal<Vec<i32>>,
) -> Element {
    let menu = &node.menu;
    let title = menu
        .title
        .clone()
        .unwrap_or_else(|| menu.name.clone().unwrap_or_default());
    let has_children = !node.children.is_empty();
    let is_expanded = expanded_keys().contains(&menu.id);
    let menu_id = menu.id;
    let indent = format!("padding-left: {}px;", 16 + depth * 20);
    let navigator = navigator();
    let current_route = use_route::<Route>();

    // 判断当前菜单是否激活
    let path = menu.path.clone().unwrap_or_default();
    let route = menu_path_to_route(&path);
    let is_active = is_active_route(&route, &current_route);
    let child_active = has_active_child(&node, &current_route);

    if has_children && !collapsed {
        // 父节点 — 可展开/折叠
        let arrow = if is_expanded { "▼" } else { "▶" };
        let mut expanded_keys_clone = expanded_keys;
        let highlight = is_expanded || child_active;
        let class = if highlight {
            "sidebar-menu-item sidebar-menu-item-active"
        } else {
            "sidebar-menu-item"
        };

        rsx! {
            div {
                // 父节点标题
                div {
                    class: "{class}",
                    style: "{indent}",
                    onclick: move |_| {
                        let mut keys = expanded_keys_clone();
                        if keys.contains(&menu_id) {
                            keys.retain(|&k| k != menu_id);
                        } else {
                            keys.push(menu_id);
                        }
                        expanded_keys_clone.set(keys);
                    },
                    span {
                        style: "margin-right: 10px; flex-shrink: 0; display: inline-flex; align-items: center;",
                        {get_menu_icon_element(menu)}
                    }
                    span {
                        style: "font-size: 14px; flex: 1; overflow: hidden; text-overflow: ellipsis;",
                        "{title}"
                    }
                    span {
                        style: "font-size: 12px; color: #8a9bb0; flex-shrink: 0;",
                        "{arrow}"
                    }
                }
                // 子菜单（展开时显示）
                if is_expanded {
                    for child in &node.children {
                        if child.menu.hidden.unwrap_or(0) == 0 {
                            MenuItem {
                                node: child.clone(),
                                depth: depth + 1,
                                collapsed: collapsed,
                                expanded_keys: expanded_keys,
                            }
                        }
                    }
                }
            }
        }
    } else if has_children && collapsed {
        // 折叠状态下，父菜单 — 点击展开侧边栏并展开该菜单
        let class = if child_active {
            "sidebar-menu-item sidebar-menu-item-active"
        } else {
            "sidebar-menu-item"
        };
        let mut expanded_keys_clone = expanded_keys;

        rsx! {
            div {
                class: "{class}",
                style: "{indent}",
                title: "{title}",
                onclick: move |_| {
                    let mut keys = expanded_keys_clone();
                    if !keys.contains(&menu_id) {
                        keys.push(menu_id);
                    }
                    expanded_keys_clone.set(keys);
                },
                span {
                    style: "flex-shrink: 0; display: inline-flex; align-items: center;",
                    {get_menu_icon_element(menu)}
                }
            }
        }
    } else {
        // 叶子菜单 — 可点击导航
        let path = menu.path.clone().unwrap_or_default();
        let route = menu_path_to_route(&path);

        if let Some(r) = route {
            let route_clone = r.clone();
            let class = if is_active {
                "sidebar-menu-item sidebar-menu-item-active"
            } else {
                "sidebar-menu-item"
            };

            rsx! {
                div {
                    class: "{class}",
                    style: "{indent}",
                    onclick: move |_| {
                        navigator.push(route_clone.clone());
                    },
                    span {
                        style: "margin-right: 10px; flex-shrink: 0; display: inline-flex; align-items: center;",
                        {get_menu_icon_element(menu)}
                    }
                    if !collapsed {
                        span {
                            style: "font-size: 14px; overflow: hidden; text-overflow: ellipsis;",
                            "{title}"
                        }
                    }
                }
            }
        } else {
            // 没有对应路由的菜单 — 仅显示文本
            rsx! {
                div {
                    class: "sidebar-menu-item",
                    style: "{indent}",
                    title: "{title}",
                    span {
                        style: "margin-right: 10px; flex-shrink: 0; display: inline-flex; align-items: center;",
                        {get_menu_icon_element(menu)}
                    }
                    if !collapsed {
                        span {
                            style: "font-size: 14px; overflow: hidden; text-overflow: ellipsis;",
                            "{title}"
                        }
                    }
                }
            }
        }
    }
}
