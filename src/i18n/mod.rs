use std::sync::{LazyLock, RwLock};

use dioxus::prelude::*;

/// 支持的语言
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    ZhCN,
    EnUS,
}

impl Locale {
    pub fn label(&self) -> &'static str {
        match self {
            Locale::ZhCN => "中文",
            Locale::EnUS => "English",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "en-US" => Locale::EnUS,
            _ => Locale::ZhCN,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Locale::ZhCN => "zh-CN",
            Locale::EnUS => "en-US",
        }
    }
}

const LOCALE_KEY: &str = "admin_locale";

/// 非响应式全局存储（供 t() 读取）
static GLOBAL_LOCALE: LazyLock<RwLock<Locale>> =
    LazyLock::new(|| RwLock::new(Locale::ZhCN));

/// 初始化语言（从 localStorage 读取）
pub fn init_locale() {
    if let Some(stored) = crate::storage::get(LOCALE_KEY) {
        *GLOBAL_LOCALE.write().unwrap() = Locale::from_str(&stored);
    }
}

/// 在 App 根组件中调用 — 提供响应式 Locale Signal
pub fn provide_locale() {
    let initial = *GLOBAL_LOCALE.read().unwrap();
    let signal = use_signal(move || initial);
    use_context_provider(|| signal);
}

/// 获取当前响应式 Locale Signal（组件渲染期可用）
pub fn locale_signal() -> Signal<Locale> {
    use_context::<Signal<Locale>>()
}

/// 获取当前语言（响应式 — 渲染期调用会订阅 Signal）
pub fn current_locale() -> Locale {
    let sig = locale_signal();
    sig()
}

/// 切换语言并持久化
pub fn set_locale(locale: Locale) {
    *GLOBAL_LOCALE.write().unwrap() = locale;
    crate::storage::set(LOCALE_KEY, locale.as_str());
    locale_signal().set(locale);
}

/// 翻译 key 枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TKey {
    // 通用
    Search, Cancel, Confirm, Edit, Delete, Loading, NoData,
    Enabled, Disabled, PrevPage, NextPage, TotalRecords, BackHome, PageNotFound,
    // 登录页
    AdminSystem, Username, Password, UsernamePlaceholder, PasswordPlaceholder,
    UsernamePasswordRequired, Login, LoggingIn,
    // 布局
    Logout,
    // 仪表盘
    Welcome, WelcomeMessage, WelcomeDescription, TotalUsers, TotalRoles, TotalMenus, TotalApis,
    SystemInfo, BackendFramework, FrontendFramework, UiLibrary, Database, AuthFramework, ApiDocs,
    // 用户管理
    UserManage, AddUser, EditUser, SearchUserPlaceholder, Nickname, Phone, Email, Status, Action,
    UsernameRequired, PasswordRequired, NicknamePlaceholder, PhonePlaceholder, EmailPlaceholder,
    // 角色管理
    RoleManage, AddRole, EditRole, RoleName, RoleKeyword, RoleDesc,
    SearchRolePlaceholder, RoleNamePlaceholder, RoleKeywordPlaceholder, RoleDescPlaceholder,
    // API管理
    ApiManage, AddApi, EditApi, ApiPath, ApiMethod, ApiGroup, ApiDescription,
    SearchApiPlaceholder, ApiPathPlaceholder, ApiMethodPlaceholder, ApiGroupPlaceholder, ApiDescPlaceholder,
    // 菜单管理
    MenuManage, AddMenu, EditMenu, MenuName, MenuTitle, MenuPath, MenuIcon, MenuSort,
    SearchMenuPlaceholder, MenuNamePlaceholder, MenuTitlePlaceholder, MenuPathPlaceholder, MenuIconPlaceholder, MenuSortPlaceholder,
    // 字典管理
    DictManage, AddDict, EditDict, DictName, DictCode, DictDesc,
    SearchDictPlaceholder, DictNamePlaceholder, DictCodePlaceholder, DictDescPlaceholder, DescPlaceholder, Description,
}

/// 翻译函数（响应式 — 渲染期调用会订阅 locale Signal）
pub fn t(key: TKey) -> String {
    match current_locale() {
        Locale::ZhCN => t_zh(key),
        Locale::EnUS => t_en(key),
    }
}

/// 带参数的翻译 — 用于分页信息
pub fn t_paging(total: u64, current: u32, total_pages: u64) -> String {
    t(TKey::TotalRecords)
        .replace("{total}", &total.to_string())
        .replace("{current}", &current.to_string())
        .replace("{total_pages}", &total_pages.to_string())
}

fn t_zh(key: TKey) -> String {
    match key {
        TKey::Search => "搜索", TKey::Cancel => "取消", TKey::Confirm => "确定",
        TKey::Edit => "编辑", TKey::Delete => "删除", TKey::Loading => "加载中...",
        TKey::NoData => "暂无数据", TKey::Enabled => "启用", TKey::Disabled => "禁用",
        TKey::PrevPage => "上一页", TKey::NextPage => "下一页",
        TKey::TotalRecords => "共 {total} 条记录，第 {current}/{total_pages} 页",
        TKey::BackHome => "返回首页", TKey::PageNotFound => "页面未找到",
        TKey::AdminSystem => "后台管理系统", TKey::Username => "用户名", TKey::Password => "密码",
        TKey::UsernamePlaceholder => "请输入用户名", TKey::PasswordPlaceholder => "请输入密码",
        TKey::UsernamePasswordRequired => "用户名和密码不能为空", TKey::Login => "登 录", TKey::LoggingIn => "登录中...",
        TKey::Logout => "退出登录",
        TKey::Welcome => "欢迎", TKey::WelcomeMessage => "欢迎使用 Axum Admin 管理系统",
        TKey::WelcomeDescription => "基于 Axum + Dioxus + Element Plus 构建的后台管理系统",
        TKey::TotalUsers => "用户总数", TKey::TotalRoles => "角色总数", TKey::TotalMenus => "菜单总数", TKey::TotalApis => "API总数",
        TKey::SystemInfo => "系统信息", TKey::BackendFramework => "后端框架", TKey::FrontendFramework => "前端框架",
        TKey::UiLibrary => "UI组件库", TKey::Database => "数据库", TKey::AuthFramework => "权限框架", TKey::ApiDocs => "API文档",
        TKey::UserManage => "用户管理", TKey::AddUser => "+ 新增用户", TKey::EditUser => "编辑用户",
        TKey::SearchUserPlaceholder => "搜索用户名/昵称/手机号", TKey::Nickname => "昵称", TKey::Phone => "手机号",
        TKey::Email => "邮箱", TKey::Status => "状态", TKey::Action => "操作",
        TKey::UsernameRequired => "用户名 *", TKey::PasswordRequired => "密码 *",
        TKey::NicknamePlaceholder => "请输入昵称", TKey::PhonePlaceholder => "请输入手机号", TKey::EmailPlaceholder => "请输入邮箱",
        TKey::RoleManage => "角色管理", TKey::AddRole => "+ 新增角色", TKey::EditRole => "编辑角色",
        TKey::RoleName => "角色名称", TKey::RoleKeyword => "关键词", TKey::RoleDesc => "描述",
        TKey::SearchRolePlaceholder => "搜索角色名称", TKey::RoleNamePlaceholder => "请输入角色名称",
        TKey::RoleKeywordPlaceholder => "请输入角色关键词", TKey::RoleDescPlaceholder => "请输入角色描述",
        TKey::ApiManage => "API管理", TKey::AddApi => "+ 新增API", TKey::EditApi => "编辑API",
        TKey::ApiPath => "路径", TKey::ApiMethod => "方法", TKey::ApiGroup => "分组", TKey::ApiDescription => "描述",
        TKey::SearchApiPlaceholder => "搜索API路径/描述", TKey::ApiPathPlaceholder => "如 /api/user/list",
        TKey::ApiMethodPlaceholder => "GET/POST/PUT/DELETE", TKey::ApiGroupPlaceholder => "如 用户管理", TKey::ApiDescPlaceholder => "请输入API描述",
        TKey::MenuManage => "菜单管理", TKey::AddMenu => "+ 新增菜单", TKey::EditMenu => "编辑菜单",
        TKey::MenuName => "名称", TKey::MenuTitle => "标题", TKey::MenuPath => "路径", TKey::MenuIcon => "图标", TKey::MenuSort => "排序",
        TKey::SearchMenuPlaceholder => "搜索菜单名称", TKey::MenuNamePlaceholder => "请输入菜单名称",
        TKey::MenuTitlePlaceholder => "请输入菜单标题", TKey::MenuPathPlaceholder => "请输入路由路径",
        TKey::MenuIconPlaceholder => "请输入图标", TKey::MenuSortPlaceholder => "请输入排序值",
        TKey::DictManage => "字典管理", TKey::AddDict => "+ 新增字典", TKey::EditDict => "编辑字典",
        TKey::DictName => "字典名称", TKey::DictCode => "编码", TKey::DictDesc => "描述",
        TKey::SearchDictPlaceholder => "搜索字典名称/编码", TKey::DictNamePlaceholder => "请输入字典名称",
        TKey::DictCodePlaceholder => "请输入字典编码", TKey::DictDescPlaceholder => "请输入描述", TKey::DescPlaceholder => "请输入描述",
        TKey::Description => "描述",
    }.into()
}

fn t_en(key: TKey) -> String {
    match key {
        TKey::Search => "Search", TKey::Cancel => "Cancel", TKey::Confirm => "Confirm",
        TKey::Edit => "Edit", TKey::Delete => "Delete", TKey::Loading => "Loading...",
        TKey::NoData => "No Data", TKey::Enabled => "Enabled", TKey::Disabled => "Disabled",
        TKey::PrevPage => "Previous", TKey::NextPage => "Next",
        TKey::TotalRecords => "Total {total} records, Page {current}/{total_pages}",
        TKey::BackHome => "Back Home", TKey::PageNotFound => "Page Not Found",
        TKey::AdminSystem => "Admin System", TKey::Username => "Username", TKey::Password => "Password",
        TKey::UsernamePlaceholder => "Enter username", TKey::PasswordPlaceholder => "Enter password",
        TKey::UsernamePasswordRequired => "Username and password cannot be empty", TKey::Login => "Login", TKey::LoggingIn => "Logging in...",
        TKey::Logout => "Logout",
        TKey::Welcome => "Welcome", TKey::WelcomeMessage => "Welcome to Axum Admin System",
        TKey::WelcomeDescription => "Backend management system built with Axum + Dioxus + Element Plus",
        TKey::TotalUsers => "Total Users", TKey::TotalRoles => "Total Roles", TKey::TotalMenus => "Total Menus", TKey::TotalApis => "Total APIs",
        TKey::SystemInfo => "System Info", TKey::BackendFramework => "Backend Framework", TKey::FrontendFramework => "Frontend Framework",
        TKey::UiLibrary => "UI Library", TKey::Database => "Database", TKey::AuthFramework => "Auth Framework", TKey::ApiDocs => "API Docs",
        TKey::UserManage => "User Management", TKey::AddUser => "+ Add User", TKey::EditUser => "Edit User",
        TKey::SearchUserPlaceholder => "Search username/nickname/phone", TKey::Nickname => "Nickname", TKey::Phone => "Phone",
        TKey::Email => "Email", TKey::Status => "Status", TKey::Action => "Action",
        TKey::UsernameRequired => "Username *", TKey::PasswordRequired => "Password *",
        TKey::NicknamePlaceholder => "Enter nickname", TKey::PhonePlaceholder => "Enter phone", TKey::EmailPlaceholder => "Enter email",
        TKey::RoleManage => "Role Management", TKey::AddRole => "+ Add Role", TKey::EditRole => "Edit Role",
        TKey::RoleName => "Role Name", TKey::RoleKeyword => "Keyword", TKey::RoleDesc => "Description",
        TKey::SearchRolePlaceholder => "Search role name", TKey::RoleNamePlaceholder => "Enter role name",
        TKey::RoleKeywordPlaceholder => "Enter role keyword", TKey::RoleDescPlaceholder => "Enter role description",
        TKey::ApiManage => "API Management", TKey::AddApi => "+ Add API", TKey::EditApi => "Edit API",
        TKey::ApiPath => "Path", TKey::ApiMethod => "Method", TKey::ApiGroup => "Group", TKey::ApiDescription => "Description",
        TKey::SearchApiPlaceholder => "Search API path/description", TKey::ApiPathPlaceholder => "e.g. /api/user/list",
        TKey::ApiMethodPlaceholder => "GET/POST/PUT/DELETE", TKey::ApiGroupPlaceholder => "e.g. User Management", TKey::ApiDescPlaceholder => "Enter API description",
        TKey::MenuManage => "Menu Management", TKey::AddMenu => "+ Add Menu", TKey::EditMenu => "Edit Menu",
        TKey::MenuName => "Name", TKey::MenuTitle => "Title", TKey::MenuPath => "Path", TKey::MenuIcon => "Icon", TKey::MenuSort => "Sort",
        TKey::SearchMenuPlaceholder => "Search menu name", TKey::MenuNamePlaceholder => "Enter menu name",
        TKey::MenuTitlePlaceholder => "Enter menu title", TKey::MenuPathPlaceholder => "Enter route path",
        TKey::MenuIconPlaceholder => "Enter icon", TKey::MenuSortPlaceholder => "Enter sort value",
        TKey::DictManage => "Dictionary Management", TKey::AddDict => "+ Add Dictionary", TKey::EditDict => "Edit Dictionary",
        TKey::DictName => "Dictionary Name", TKey::DictCode => "Code", TKey::DictDesc => "Description",
        TKey::SearchDictPlaceholder => "Search dictionary name/code", TKey::DictNamePlaceholder => "Enter dictionary name",
        TKey::DictCodePlaceholder => "Enter dictionary code", TKey::DictDescPlaceholder => "Enter description", TKey::DescPlaceholder => "Enter description",
        TKey::Description => "Description",
    }.into()
}
