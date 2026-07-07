//! Dioxus 国际化基础库
//!
//! 提供 Locale 枚举、全局状态管理和 Dioxus Signal 响应式支持。
//! localStorage 持久化通过闭包注入，不耦合具体存储实现。
//!
//! 使用 `GlobalSignal` 替代 `use_context` + `use_context_provider`，
//! 避免在事件处理器中调用 hook 导致的 hooks 顺序错乱问题。
//!
//! # 快速使用
//!
//! ```rust,ignore
//! use dioxus::prelude::*;
//! use dioxus_i18n::{Locale, init_locale, set_locale, current_locale};
//!
//! fn main() {
//!     // 注入存储读取闭包
//!     init_locale(|| storage::get("locale"));
//!     dioxus::launch(App);
//! }
//!
//! #[component]
//! fn App() -> Element {
//!     rsx! { p { "{current_locale().label()}" } }
//! }
//!
//! fn switch() {
//!     // 注入存储写入闭包
//!     set_locale(Locale::EnUS, |s| storage::set("locale", s));
//! }
//! ```

use std::sync::{LazyLock, RwLock};

use dioxus::prelude::*;

/// 支持的语言
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Locale {
    ZhCN,
    EnUS,
}

impl Locale {
    /// 语言的人类可读标签
    pub fn label(&self) -> &'static str {
        match self {
            Locale::ZhCN => "中文",
            Locale::EnUS => "English",
        }
    }


    /// 转为字符串标识
    pub fn as_str(&self) -> &'static str {
        match self {
            Locale::ZhCN => "zh-CN",
            Locale::EnUS => "en-US",
        }
    }
}

/// 非响应式全局存储（供 `init_locale()` 读写，在虚拟 DOM 创建前使用）
static GLOBAL_LOCALE: LazyLock<RwLock<Locale>> =
    LazyLock::new(|| RwLock::new(Locale::ZhCN));

/// 响应式全局 Signal — 可在任何组件渲染期或事件处理器中使用，无需 use_context
/// 首次访问时从 GLOBAL_LOCALE 读取初始值
static LOCALE: GlobalSignal<Locale> = Signal::global(|| *GLOBAL_LOCALE.read().unwrap());

/// 初始化语言
///
/// 接受一个闭包从持久化存储中读取已保存的语言偏好。
/// 若闭包返回 `None` 或读取失败，则使用默认值 `ZhCN`。
///
/// ```rust,ignore
/// init_locale(|| crate::storage::get("admin_locale"));
/// ```
pub fn init_locale<F: FnOnce() -> Option<String>>(get_stored: F) {
    if let Some(stored) = get_stored() {
        *GLOBAL_LOCALE.write().unwrap() = stored.parse::<Locale>().unwrap_or(Locale::ZhCN);
    }
}

/// 获取当前语言（响应式 — 渲染期调用会订阅 Signal）
pub fn current_locale() -> Locale {
    LOCALE()
}

/// 切换语言并持久化
///
/// 接受一个闭包将语言标识写入持久化存储。
///
/// ```rust,ignore
/// set_locale(Locale::EnUS, |s| crate::storage::set("admin_locale", s));
/// ```
pub fn set_locale<F: FnOnce(&str)>(locale: Locale, persist: F) {
    *GLOBAL_LOCALE.write().unwrap() = locale;
    persist(locale.as_str());
    *LOCALE.write() = locale;
}

impl std::str::FromStr for Locale {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "zh-CN" => Ok(Locale::ZhCN),
            "en-US" => Ok(Locale::EnUS),
            _ => Err("不支持的语言标识，支持: 'zh-CN' 或 'en-US'"),
        }
    }
}
