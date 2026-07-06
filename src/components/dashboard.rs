use dioxus::prelude::*;
use dioxus_element_plug::prelude::*;

/// 仪表盘页面
#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div {
            style: "max-width: 1200px;",

            // 欢迎卡片
            Card {
                class: "mb-6",
                header: Some("欢迎".to_string()),
                

                div {
                    style: "padding: 20px 0;",
                    h2 {
                        style: "font-size: 24px; color: #303030; margin: 0 0 8px 0;",
                        "欢迎使用 Axum Admin 管理系统"
                    }
                    p {
                        style: "font-size: 14px; color: #909399; margin: 0;",
                        "基于 Axum + Dioxus + Element Plus 构建的后台管理系统"
                    }
                }
            }

            // 统计卡片
            div {
                style: "display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 20px; margin-bottom: 20px;",

                // 用户数
                div {
                    style: "background: white; border-radius: 8px; padding: 24px; box-shadow: 0 2px 12px rgba(0,0,0,0.08);",
                    div {
                        style: "display: flex; align-items: center; justify-content: space-between;",
                        div {
                            p { style: "font-size: 14px; color: #909399; margin: 0 0 8px 0;", "用户总数" }
                            p { style: "font-size: 28px; font-weight: 700; color: #409eff; margin: 0;", "128" }
                        }
                        div {
                            style: "width: 48px; height: 48px; background: #ecf5ff; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 24px;",
                            "👤"
                        }
                    }
                }

                // 角色数
                div {
                    style: "background: white; border-radius: 8px; padding: 24px; box-shadow: 0 2px 12px rgba(0,0,0,0.08);",
                    div {
                        style: "display: flex; align-items: center; justify-content: space-between;",
                        div {
                            p { style: "font-size: 14px; color: #909399; margin: 0 0 8px 0;", "角色总数" }
                            p { style: "font-size: 28px; font-weight: 700; color: #67c23a; margin: 0;", "8" }
                        }
                        div {
                            style: "width: 48px; height: 48px; background: #f0f9eb; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 24px;",
                            "👥"
                        }
                    }
                }

                // 菜单数
                div {
                    style: "background: white; border-radius: 8px; padding: 24px; box-shadow: 0 2px 12px rgba(0,0,0,0.08);",
                    div {
                        style: "display: flex; align-items: center; justify-content: space-between;",
                        div {
                            p { style: "font-size: 14px; color: #909399; margin: 0 0 8px 0;", "菜单总数" }
                            p { style: "font-size: 28px; font-weight: 700; color: #e6a23c; margin: 0;", "32" }
                        }
                        div {
                            style: "width: 48px; height: 48px; background: #fdf6ec; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 24px;",
                            "📋"
                        }
                    }
                }

                // API数
                div {
                    style: "background: white; border-radius: 8px; padding: 24px; box-shadow: 0 2px 12px rgba(0,0,0,0.08);",
                    div {
                        style: "display: flex; align-items: center; justify-content: space-between;",
                        div {
                            p { style: "font-size: 14px; color: #909399; margin: 0 0 8px 0;", "API总数" }
                            p { style: "font-size: 28px; font-weight: 700; color: #f56c6c; margin: 0;", "56" }
                        }
                        div {
                            style: "width: 48px; height: 48px; background: #fef0f0; border-radius: 8px; display: flex; align-items: center; justify-content: center; font-size: 24px;",
                            "🔌"
                        }
                    }
                }
            }

            // 系统信息
            Card {
                header: Some("系统信息".to_string()),
                

                div {
                    style: "padding: 12px 0;",

                    div {
                        style: "display: flex; padding: 12px 0; border-bottom: 1px solid #ebeef5;",
                        span { style: "width: 160px; color: #909399; font-size: 14px;", "后端框架" }
                        span { style: "color: #303030; font-size: 14px;", "Axum 0.8 (Rust)" }
                    }
                    div {
                        style: "display: flex; padding: 12px 0; border-bottom: 1px solid #ebeef5;",
                        span { style: "width: 160px; color: #909399; font-size: 14px;", "前端框架" }
                        span { style: "color: #303030; font-size: 14px;", "Dioxus 0.7 (Rust/WASM)" }
                    }
                    div {
                        style: "display: flex; padding: 12px 0; border-bottom: 1px solid #ebeef5;",
                        span { style: "width: 160px; color: #909399; font-size: 14px;", "UI组件库" }
                        span { style: "color: #303030; font-size: 14px;", "Element Plus (dioxus-element-plug)" }
                    }
                    div {
                        style: "display: flex; padding: 12px 0; border-bottom: 1px solid #ebeef5;",
                        span { style: "width: 160px; color: #909399; font-size: 14px;", "数据库" }
                        span { style: "color: #303030; font-size: 14px;", "MySQL (SeaORM)" }
                    }
                    div {
                        style: "display: flex; padding: 12px 0; border-bottom: 1px solid #ebeef5;",
                        span { style: "width: 160px; color: #909399; font-size: 14px;", "权限框架" }
                        span { style: "color: #303030; font-size: 14px;", "Casbin RBAC" }
                    }
                    div {
                        style: "display: flex; padding: 12px 0;",
                        span { style: "width: 160px; color: #909399; font-size: 14px;", "API文档" }
                        a {
                            href: "http://localhost:8888",
                            target: "_blank",
                            style: "color: #409eff; font-size: 14px; text-decoration: none;",
                            "Swagger UI"
                        }
                    }
                }
            }
        }
    }
}
