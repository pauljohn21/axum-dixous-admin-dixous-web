//! 滑块验证码组件库
//!
//! 提供一个纯 Dioxus 滑块验证码组件，无外部业务依赖。
//!
//! # 快速使用
//!
//! ```rust,ignore
//! use dioxus::prelude::*;
//! use slider_captcha::SliderCaptcha;
//!
//! #[component]
//! fn App() -> Element {
//!     let mut verified = use_signal(|| false);
//!     rsx! {
//!         SliderCaptcha {
//!             placeholder: "请拖动滑块验证".to_string(),
//!             success_text: "验证通过".to_string(),
//!             on_success: move |_| { verified.set(true); }
//!         }
//!     }
//! }
//! ```

use dioxus::prelude::*;

/// 滑块验证码组件
///
/// 用户需拖动滑块从左到右完成验证。
/// 验证通过后通过 `on_success` 回调通知父组件。
///
/// # Props
/// - `placeholder` — 未验证时轨道中央提示文字
/// - `success_text` — 验证通过后轨道中央显示文字
/// - `on_success` — 验证通过回调
///
/// # 状态
/// - 初始: 灰色轨道，蓝色滑块按钮，显示 placeholder
/// - 拖动中: 蓝色进度条跟随滑块
/// - 已验证: 绿色轨道，绿色滑块，显示 success_text
/// - 未到终点松开: 滑块回弹到起点
#[component]
pub fn SliderCaptcha(
    placeholder: String,
    success_text: String,
    on_success: EventHandler<()>,
) -> Element {
    let mut is_dragging = use_signal(|| false);
    let mut slider_offset = use_signal(|| 0i32);
    let mut is_verified = use_signal(|| false);
    let mut start_mouse_x = use_signal(|| 0i32);
    let mut start_offset = use_signal(|| 0i32);

    // 滑块按钮宽度 (px)
    const BUTTON_WIDTH: i32 = 40;
    // 最大偏移量 = 轨道宽度 - 按钮宽度 (登录框内容区 320px)
    const MAX_OFFSET: i32 = 280;
    // 验证阈值 — 拖到最右端附近算通过
    const VERIFY_THRESHOLD: i32 = 270;

    // 鼠标按下: 开始拖动
    let on_mouse_down = move |e: MouseEvent| {
        if is_verified() {
            return;
        }
        start_mouse_x.set(e.client_coordinates().x as i32);
        start_offset.set(slider_offset());
        is_dragging.set(true);
    };

    // 鼠标移动: 更新偏移量
    let on_mouse_move = move |e: MouseEvent| {
        if !is_dragging() || is_verified() {
            return;
        }
        let delta = e.client_coordinates().x as i32 - start_mouse_x();
        let new_offset = (start_offset() + delta).clamp(0, MAX_OFFSET);
        slider_offset.set(new_offset);
    };

    // 完成拖动: 验证或回弹
    let mut finish_drag = move || {
        if !is_dragging() || is_verified() {
            return;
        }
        is_dragging.set(false);
        if slider_offset() >= VERIFY_THRESHOLD {
            slider_offset.set(MAX_OFFSET);
            is_verified.set(true);
            on_success.call(());
        } else {
            slider_offset.set(0);
        }
    };

    let on_mouse_up = move |_| {
        finish_drag();
    };

    let on_mouse_leave = move |_| {
        finish_drag();
    };

    // 计算样式
    let offset_px = format!("{}px", slider_offset());
    let filled_width = format!("{}px", slider_offset() + BUTTON_WIDTH);
    let progress_bg = if is_verified() {
        "linear-gradient(135deg, #67c23a, #85ce61)"
    } else {
        "linear-gradient(135deg, #409eff, #66b1ff)"
    };
    let slider_bg = if is_verified() { "#67c23a" } else { "#409eff" };
    let cursor = if is_verified() {
        "default"
    } else if is_dragging() {
        "grabbing"
    } else {
        "grab"
    };
    let slider_transition = if is_dragging() {
        "none"
    } else {
        "left 0.3s cubic-bezier(0.4, 0, 0.2, 1), background 0.3s ease"
    };
    let progress_transition = if is_dragging() {
        "none"
    } else {
        "width 0.3s cubic-bezier(0.4, 0, 0.2, 1), background 0.3s ease"
    };

    rsx! {
        div {
            style: "width: 100%; margin-bottom: 24px; user-select: none;",

            // 轨道容器
            div {
                style: "position: relative; width: 100%; height: 40px; background: #f0f0f0; border-radius: 20px; overflow: hidden;",
                onmousemove: on_mouse_move,
                onmouseup: on_mouse_up,
                onmouseleave: on_mouse_leave,

                // 进度条 (已填充部分)
                div {
                    style: "position: absolute; left: 0; top: 0; height: 100%; width: {filled_width}; background: {progress_bg}; border-radius: 20px; transition: {progress_transition};",
                }

                // 提示文字 — 未验证且未拖动时显示
                if !is_verified() && !is_dragging() {
                    span {
                        style: "position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%); font-size: 13px; color: #909399; pointer-events: none; white-space: nowrap;",
                        "{placeholder}"
                    }
                }

                // 验证通过文字
                if is_verified() {
                    span {
                        style: "position: absolute; left: 50%; top: 50%; transform: translate(-50%, -50%); font-size: 13px; color: #fff; pointer-events: none; font-weight: 500; white-space: nowrap;",
                        "{success_text}"
                    }
                }

                // 滑块按钮
                div {
                    style: "position: absolute; left: {offset_px}; top: 0; width: {BUTTON_WIDTH}px; height: 40px; background: {slider_bg}; border-radius: 20px; cursor: {cursor}; display: flex; align-items: center; justify-content: center; box-shadow: 0 2px 8px rgba(0,0,0,0.25); transition: {slider_transition}; z-index: 1;",
                    onmousedown: on_mouse_down,

                    if is_verified() {
                        // 验证通过图标
                        span {
                            style: "color: #fff; font-size: 18px; font-weight: bold; line-height: 1;",
                            "✓"
                        }
                    } else {
                        // 右箭头图标
                        span {
                            style: "color: #fff; font-size: 16px; font-weight: bold; line-height: 1;",
                            "⇒"
                        }
                    }
                }
            }
        }
    }
}
