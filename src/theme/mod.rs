//! 主题管理模块 — 亮色/暗色主题切换
//!
//! 架构:
//! - `GLOBAL_THEME` — 非响应式全局存储 (供 main() 初始化时读写，在虚拟 DOM 创建前使用)
//! - `THEME` — `GlobalSignal<ThemeMode>` 响应式全局信号 (渲染期订阅，事件处理器中写入)
//! - `current_theme()` — 渲染期调用，返回当前主题 (响应式)
//! - `set_theme()` — 切换主题并持久化到 localStorage
//!
//! 使用 `GlobalSignal` 替代 `use_context` + `use_context_provider`，
//! 避免在事件处理器中调用 hook 导致的 hooks 顺序错乱问题。

use std::sync::{LazyLock, RwLock};

use dioxus::prelude::*;

/// 主题模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl std::str::FromStr for ThemeMode {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "light" => Ok(ThemeMode::Light),
            "dark" => Ok(ThemeMode::Dark),
            _ => Err("不支持的主题模式，支持: 'light' 或 'dark'"),
        }
    }
}

impl ThemeMode {
    /// 返回用于 CSS data-theme 属性的字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        }
    }


}

const THEME_KEY: &str = "admin_theme";

/// 非响应式全局存储 (供 init/set 时读写)
static GLOBAL_THEME: LazyLock<RwLock<ThemeMode>> =
    LazyLock::new(|| RwLock::new(ThemeMode::Light));

/// 初始化主题 (从 localStorage 读取) — 在 main() 中调用
pub fn init_theme() {
    if let Some(stored) = crate::storage::get(THEME_KEY) {
        let mode = stored.parse::<ThemeMode>().unwrap_or(ThemeMode::Light);
        *GLOBAL_THEME.write().unwrap() = mode;
    } else {
        // 检测系统偏好
        if let Some(window) = web_sys::window() {
            if let Ok(Some(media)) = window.match_media("(prefers-color-scheme: dark)") {
                if media.matches() {
                    *GLOBAL_THEME.write().unwrap() = ThemeMode::Dark;
                }
            }
        }
    }
}

/// 响应式全局 Signal — 可在任何组件渲染期或事件处理器中使用，无需 use_context
/// 首次访问时从 GLOBAL_THEME 读取初始值
static THEME: GlobalSignal<ThemeMode> = Signal::global(|| *GLOBAL_THEME.read().unwrap());

/// 获取当前主题 (响应式 — 渲染期调用会订阅 Signal)
pub fn current_theme() -> ThemeMode {
    THEME()
}

/// 切换主题并持久化
pub fn set_theme(mode: ThemeMode) {
    *GLOBAL_THEME.write().unwrap() = mode;
    crate::storage::set(THEME_KEY, mode.as_str());
    *THEME.write() = mode;
}

/// 在亮色/暗色之间切换
pub fn toggle_theme() {
    let next = match current_theme() {
        ThemeMode::Light => ThemeMode::Dark,
        ThemeMode::Dark => ThemeMode::Light,
    };
    set_theme(next);
}

// ========== CSS 变量定义 ==========

/// 生成主题 CSS — 包含亮色和暗色两套 CSS 变量
/// 通过 [data-theme="dark"] 选择器覆盖 :root 变量
pub fn theme_css() -> String {
    r#"
:root {
    /* ===== 亮色主题 (默认) ===== */

    /* 品牌色 */
    --el-color-primary: #409eff;
    --el-color-primary-light-3: #79bbff;
    --el-color-primary-light-5: #a0cfff;
    --el-color-primary-light-7: #c6e2ff;
    --el-color-primary-light-8: #d9ecff;
    --el-color-primary-light-9: #ecf5ff;
    --el-color-primary-dark-2: #337ecc;

    /* 功能色 */
    --el-color-success: #67c23a;
    --el-color-success-light-9: #f0f9eb;
    --el-color-warning: #e6a23c;
    --el-color-warning-light-9: #fdf6ec;
    --el-color-danger: #f56c6c;
    --el-color-danger-light-9: #fef0f0;
    --el-color-info: #909399;

    /* 文字色 */
    --el-text-color-primary: #303133;
    --el-text-color-regular: #606266;
    --el-text-color-secondary: #909399;
    --el-text-color-placeholder: #a8abb2;
    --el-text-color-disabled: #c0c4cc;

    /* 边框色 */
    --el-border-color: #dcdfe6;
    --el-border-color-light: #e4e7ed;
    --el-border-color-lighter: #ebeef5;
    --el-border-color-extra-light: #f2f6fc;
    --el-border-color-dark: #d4d7de;

    /* 填充色 */
    --el-fill-color: #f0f2f5;
    --el-fill-color-light: #f5f7fa;
    --el-fill-color-lighter: #fafafa;
    --el-fill-color-extra-light: #fafcff;
    --el-fill-color-blank: #ffffff;

    /* 背景色 */
    --el-bg-color: #ffffff;
    --el-bg-color-page: #f0f2f5;
    --el-bg-color-overlay: #ffffff;

    /* 侧边栏 */
    --el-sidebar-bg: #304156;
    --el-sidebar-bg-deep: #263445;
    --el-sidebar-bg-active: #1f2d3d;
    --el-sidebar-text: #bfcbd9;
    --el-sidebar-text-active: #ffffff;
    --el-sidebar-arrow: #8a9bb0;

    /* 头部 */
    --el-header-bg: #ffffff;
    --el-header-text: #5a5e66;
    --el-header-shadow: 0 1px 4px rgba(0,21,41,0.08);

    /* 阴影 */
    --el-box-shadow: 0 2px 12px rgba(0,0,0,0.1);
    --el-box-shadow-light: 0 2px 4px rgba(0,0,0,0.04);
    --el-box-shadow-lighter: 0 2px 12px rgba(0,0,0,0.08);

    /* 遮罩 */
    --el-overlay-color: rgba(0,0,0,0.5);

    /* 过渡 */
    --el-transition-duration: 0.3s;
}

[data-theme="dark"] {
    /* ===== 暗色主题 ===== */

    /* 品牌色 (保持不变) */
    --el-color-primary: #409eff;
    --el-color-primary-light-3: #3375b9;
    --el-color-primary-light-5: #2a5f99;
    --el-color-primary-light-7: #214979;
    --el-color-primary-light-8: #1c3d66;
    --el-color-primary-light-9: #182f4d;
    --el-color-primary-dark-2: #66b1ff;

    /* 功能色 */
    --el-color-success: #67c23a;
    --el-color-success-light-9: #1a2e14;
    --el-color-warning: #e6a23c;
    --el-color-warning-light-9: #2e2410;
    --el-color-danger: #f56c6c;
    --el-color-danger-light-9: #2e1515;
    --el-color-info: #73767a;

    /* 文字色 */
    --el-text-color-primary: #e5eaf3;
    --el-text-color-regular: #cfd3dc;
    --el-text-color-secondary: #a3a6ad;
    --el-text-color-placeholder: #8d9095;
    --el-text-color-disabled: #6c6e72;

    /* 边框色 */
    --el-border-color: #4c4d4f;
    --el-border-color-light: #414243;
    --el-border-color-lighter: #363637;
    --el-border-color-extra-light: #2b2b2c;
    --el-border-color-dark: #58585a;

    /* 填充色 */
    --el-fill-color: #303030;
    --el-fill-color-light: #262727;
    --el-fill-color-lighter: #1d1e1f;
    --el-fill-color-extra-light: #1a1a1a;
    --el-fill-color-blank: #141414;

    /* 背景色 */
    --el-bg-color: #141414;
    --el-bg-color-page: #141414;
    --el-bg-color-overlay: #1d1e1f;

    /* 侧边栏 */
    --el-sidebar-bg: #1d1e1f;
    --el-sidebar-bg-deep: #181818;
    --el-sidebar-bg-active: #262727;
    --el-sidebar-text: #cfd3dc;
    --el-sidebar-text-active: #ffffff;
    --el-sidebar-arrow: #8d9095;

    /* 头部 */
    --el-header-bg: #1d1e1f;
    --el-header-text: #cfd3dc;
    --el-header-shadow: 0 1px 4px rgba(0,0,0,0.3);

    /* 阴影 */
    --el-box-shadow: 0 2px 12px rgba(0,0,0,0.4);
    --el-box-shadow-light: 0 2px 4px rgba(0,0,0,0.2);
    --el-box-shadow-lighter: 0 2px 12px rgba(0,0,0,0.3);

    /* 遮罩 */
    --el-overlay-color: rgba(0,0,0,0.7);
}

/* 主题切换过渡 — 仅作用于布局元素，避免影响交互组件性能 */
body, .sidebar-menu-item, .login-container > div, [data-theme] > div {
    transition: background-color var(--el-transition-duration),
                border-color var(--el-transition-duration),
                color var(--el-transition-duration);
}
"#.to_string()
}