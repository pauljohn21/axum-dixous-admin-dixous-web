//! Dioxus 路由守卫库
//!
//! 提供类似 Vue Router `beforeEach` 的全局路由守卫功能。
//!
//! # 快速使用
//!
//! ```rust,ignore
//! use dioxus::prelude::*;
//! use dioxus::router::RouterConfig;
//! use route_guard::RouteGuard;
//!
//! #[derive(Clone, Routable, PartialEq)]
//! enum Route {
//!     #[route("/login")]
//!     Login {},
//!     #[route("/")]
//!     Home {},
//! }
//!
//! fn route_config() -> RouterConfig<Route> {
//!     RouteGuard::<Route>::new()
//!         .is_authenticated(|| { /* 检查是否已登录 */ true })
//!         .requires_auth(|route| !matches!(route, Route::Login {}))
//!         .login_route(Route::Login {})
//!         .home_route(Route::Home {})
//!         .config()
//! }
//! ```

use dioxus::router::{GenericRouterContext, RouterConfig, Routable};

/// 路由守卫构建器
///
/// 通过 builder 模式配置鉴权规则，最终生成 [`RouterConfig`]。
///
/// 守卫在路由变化后、组件渲染前执行，因此不会有页面闪烁。
pub struct RouteGuard<R: Routable> {
    is_authenticated: Box<dyn Fn() -> bool>,
    requires_auth: Box<dyn Fn(&R) -> bool>,
    login_route: Option<R>,
    home_route: Option<R>,
}

impl<R: Routable + Clone + PartialEq + 'static> RouteGuard<R> {
    /// 创建一个空的守卫配置
    ///
    /// 默认行为：
    /// - `is_authenticated` 始终返回 `false`
    /// - `requires_auth` 始终返回 `true`
    /// - 无 login_route / home_route（不触发重定向）
    pub fn new() -> Self {
        Self {
            is_authenticated: Box::new(|| false),
            requires_auth: Box::new(|_| true),
            login_route: None,
            home_route: None,
        }
    }

    /// 设置认证检查函数
    ///
    /// 返回 `true` 表示已登录，`false` 表示未登录。
    ///
    /// ```rust,ignore
    /// RouteGuard::<Route>::new()
    ///     .is_authenticated(|| storage::get_token().is_some())
    /// ```
    pub fn is_authenticated<F: Fn() -> bool + 'static>(mut self, f: F) -> Self {
        self.is_authenticated = Box::new(f);
        self
    }

    /// 设置需要认证的路由判断函数
    ///
    /// 返回 `true` 表示该路由需要登录才能访问。
    ///
    /// ```rust,ignore
    /// RouteGuard::<Route>::new()
    ///     .requires_auth(|route| !matches!(route, Route::Login {}))
    /// ```
    pub fn requires_auth<F: Fn(&R) -> bool + 'static>(mut self, f: F) -> Self {
        self.requires_auth = Box::new(f);
        self
    }

    /// 设置未登录时的重定向目标（通常是登录页）
    ///
    /// ```rust,ignore
    /// RouteGuard::<Route>::new()
    ///     .login_route(Route::Login {})
    /// ```
    pub fn login_route(mut self, route: R) -> Self {
        self.login_route = Some(route);
        self
    }

    /// 设置已登录用户访问登录页时的重定向目标（通常是首页）
    ///
    /// ```rust,ignore
    /// RouteGuard::<Route>::new()
    ///     .home_route(Route::Dashboard {})
    /// ```
    pub fn home_route(mut self, route: R) -> Self {
        self.home_route = Some(route);
        self
    }

    /// 构建 [`RouterConfig`]，将守卫注入路由器
    ///
    /// 守卫逻辑：
    /// 1. 未登录 + 访问受保护路由 → 重定向到 `login_route`
    /// 2. 已登录 + 访问登录页 → 重定向到 `home_route`
    /// 3. 其他情况 → 正常导航
    pub fn config(self) -> RouterConfig<R> {
        let is_authenticated = self.is_authenticated;
        let requires_auth = self.requires_auth;
        let login_route = self.login_route;
        let home_route = self.home_route;

        RouterConfig::<R>::default().on_update(move |ctx: GenericRouterContext<R>| {
            let current: R = ctx.current();
            let has_auth = is_authenticated();

            // 未登录访问受保护路由 → 跳转登录页
            if requires_auth(&current) && !has_auth {
                if let Some(ref login) = login_route {
                    return Some(login.clone().into());
                }
            }

            // 已登录访问登录页 → 跳转首页
            if has_auth {
                if let Some(ref home) = home_route {
                    // 仅当当前路由等于登录路由时才重定向
                    if Some(&current) == login_route.as_ref() {
                        return Some(home.clone().into());
                    }
                }
            }

            None
        })
    }
}

impl<R: Routable + Clone + PartialEq + 'static> Default for RouteGuard<R> {
    fn default() -> Self {
        Self::new()
    }
}
