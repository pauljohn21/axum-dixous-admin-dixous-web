use dioxus::prelude::*;
use dioxus::router::Link;
use dioxus_element_plug::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // 生成 Element Plus 完整样式
    let styles = CompleteStyleManager::new().generate_complete_styles();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        // 注入生成的 CSS 样式
        style { "{styles}" }
        Router::<Route> {}
    }
}

/// 路由枚举
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

/// Home page - 使用 Element Plus 组件展示
#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut input_value = use_signal(|| String::new());

    rsx! {
        div {
            class: "min-h-screen bg-gray-50 p-8",

            // 页面标题
            h1 {
                class: "text-3xl font-bold text-gray-800 mb-8",
                "Dioxus + Element Plus 0.2.0 Demo"
            }

            // 卡片容器
            Card {
                class: "mb-6",
                header: Some("计数器示例".to_string()),

                div {
                    class: "flex items-center gap-4",
                    p {
                        class: "text-lg",
                        "点击次数: {count}"
                    }
                    Button {
                        variant: ButtonVariant::Primary,
                        size: Some(ButtonSize::Medium),
                        on_click: move |_| count += 1,
                        "增加"
                    }
                    Button {
                        variant: ButtonVariant::Default,
                        size: Some(ButtonSize::Medium),
                        on_click: move |_| count.set(0),
                        "重置"
                    }
                }
            }

            // 输入框示例
            Card {
                class: "mb-6",
                header: Some("输入框示例".to_string()),

                div {
                    class: "space-y-4",
                    Input {
                        value: Some(input_value()),
                        placeholder: Some("请输入内容...".to_string()),
                        size: InputSize::Medium,
                        on_input: move |e: Event<FormData>| {
                            input_value.set(e.data().value());
                        }
                    }
                    p {
                        class: "text-gray-600",
                        "当前输入: {input_value}"
                    }
                }
            }

            // 导航链接
            Card {
                header: Some("页面导航".to_string()),

                div {
                    class: "flex gap-4",
                    Link {
                        to: Route::Blog { id: 1 },
                        Button {
                            variant: ButtonVariant::Primary,
                            "查看博客 #1"
                        }
                    }
                    Link {
                        to: Route::Blog { id: 2 },
                        Button {
                            variant: ButtonVariant::Default,
                            "查看博客 #2"
                        }
                    }
                }
            }
        }
    }
}

/// Blog page - 使用 Element Plus 组件
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            class: "min-h-screen bg-gray-50 p-8",

            Card {
                class: "max-w-2xl mx-auto",
                header: Some(format!("博客文章 #{}", id)),

                div {
                    class: "space-y-4",

                    p {
                        class: "text-gray-700 leading-relaxed",
                        "在博客 #{id} 中，我们展示了 Dioxus 路由如何工作，以及 URL 参数如何作为 props 传递给路由组件。"
                    }

                    div {
                        class: "flex gap-4 pt-4",
                        Link {
                            to: Route::Blog { id: id - 1 },
                            Button {
                                variant: ButtonVariant::Default,
                                "上一篇"
                            }
                        }
                        Link {
                            to: Route::Home {},
                            Button {
                                variant: ButtonVariant::Primary,
                                "返回首页"
                            }
                        }
                        Link {
                            to: Route::Blog { id: id + 1 },
                            Button {
                                variant: ButtonVariant::Default,
                                "下一篇"
                            }
                        }
                    }
                }
            }
        }
    }
}
