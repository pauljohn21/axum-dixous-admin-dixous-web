//! 滑块验证码组件库
//!
//! 提供一个纯 Dioxus 滑块验证码组件，无外部业务依赖。
//!
//! # 性能优化
//!
//! 拖动期间使用 `web_sys` 直接操作 DOM 元素的 `style` 属性，
//! 完全绕过 Dioxus 虚拟 DOM，避免每次 `mousemove` 触发组件重新渲染。
//! 仅在拖动结束时更新 Signal 状态。

use std::sync::atomic::{AtomicU32, Ordering};

use dioxus::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{Document, MouseEvent as WebMouseEvent};

/// 滑块按钮宽度 (px)
const BUTTON_WIDTH: i32 = 40;
/// 最大偏移量 = 轨道宽度 - 按钮宽度
const MAX_OFFSET: i32 = 280;
/// 验证阈值 — 拖到最右端附近算通过
const VERIFY_THRESHOLD: i32 = 270;

/// 滑块验证码组件
#[component]
pub fn SliderCaptcha(
    placeholder: String,
    success_text: String,
    on_success: EventHandler<()>,
) -> Element {
    let is_verified = use_signal(|| false);
    let slider_offset = use_signal(|| 0i32);

    // 生成唯一 ID，避免多实例冲突
    static COUNTER: AtomicU32 = AtomicU32::new(0);
    let instance_id = use_hook(|| {
        let n = COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("slider-captcha-{n}")
    });
    let btn_id = format!("{}-btn", instance_id);
    let progress_id = format!("{}-progress", instance_id);

    // 闭包内用的克隆（闭包是 move，不能与 rsx 共享所有权）
    let btn_id_closure = btn_id.clone();
    let progress_id_closure = progress_id.clone();

    // 鼠标按下: 开始拖动 — 挂载 document 级原生事件
    let on_mouse_down = move |e: MouseEvent| {
        if is_verified() {
            return;
        }
        let client_x = e.client_coordinates().x as i32;
        let initial_offset = slider_offset();
        let btn_id_btn = btn_id_closure.clone();
        let progress_id_btn = progress_id_closure.clone();
        let btn_id_mu = btn_id_closure.clone();
        let progress_id_mu = progress_id_closure.clone();

        let Some(window) = web_sys::window() else {
            return;
        };
        let Some(document) = window.document() else {
            return;
        };

        // —— mousemove 闭包：直接操作 DOM，不写 Signal —— //
        let doc_mm = document.clone();
        let mousemove_closure = Closure::wrap(Box::new(move |e: &WebMouseEvent| {
            let delta = e.client_x() - client_x;
            let new_offset = (initial_offset + delta).clamp(0, MAX_OFFSET);
            // 直接更新 DOM — 零 Dioxus 重新渲染
            set_slider_dom(&doc_mm, &btn_id_btn, &progress_id_btn, new_offset, false);
        }) as Box<dyn FnMut(&WebMouseEvent)>);

        let mm_fn: js_sys::Function = mousemove_closure.as_ref().unchecked_ref::<js_sys::Function>().clone();
        let _ = document.add_event_listener_with_callback("mousemove", mm_fn.as_ref());
        // 持有闭包防止 GC 回收（在 mouseup 中移除监听后自动释放）
        let mm_jsv: JsValue = mousemove_closure.into_js_value();

        // —— mouseup 闭包：结束拖动，更新 Signal，移除监听 —— //
        let mut is_verified_sig = is_verified;
        let mut slider_offset_sig = slider_offset;
        let on_success_clone = on_success.clone();
        let doc_mu = document.clone();

        let mouseup_closure = Closure::once(Box::new(move || {
            // 移除 mousemove 监听
            let _ = doc_mu.remove_event_listener_with_callback("mousemove", mm_fn.as_ref());

            // 从 DOM 的 data-offset 属性读取最终偏移量
            let final_offset = doc_mu
                .get_element_by_id(&btn_id_mu)
                .and_then(|el| el.get_attribute("data-offset"))
                .and_then(|s| s.parse::<i32>().ok())
                .unwrap_or(0);

            if final_offset >= VERIFY_THRESHOLD {
                // 验证通过
                slider_offset_sig.set(MAX_OFFSET);
                is_verified_sig.set(true);
                set_slider_dom(&doc_mu, &btn_id_mu, &progress_id_mu, MAX_OFFSET, true);
                on_success_clone.call(());
            } else {
                // 回弹
                slider_offset_sig.set(0);
                set_slider_dom(&doc_mu, &btn_id_mu, &progress_id_mu, 0, false);
            }
        }) as Box<dyn FnOnce()>);

        let mu_fn: js_sys::Function = mouseup_closure.as_ref().unchecked_ref::<js_sys::Function>().clone();
        let opts = web_sys::AddEventListenerOptions::new();
        opts.set_once(true);
        let _ = document.add_event_listener_with_callback_and_add_event_listener_options(
            "mouseup",
            mu_fn.as_ref(),
            &opts,
        );
        mouseup_closure.forget();

        // mm_jsv 被 addEventListener 持有引用，不会被 GC
        // 当 mouseup 中 removeEventListener 后自动释放
        let _ = mm_jsv;
    };

    // 计算渲染样式
    let offset_px = format!("{}px", slider_offset());
    let filled_width = format!("{}px", slider_offset() + BUTTON_WIDTH);
    let progress_bg = if is_verified() {
        "linear-gradient(135deg, #67c23a, #85ce61)"
    } else {
        "linear-gradient(135deg, #409eff, #66b1ff)"
    };
    let slider_bg = if is_verified() { "#67c23a" } else { "#409eff" };
    let cursor = if is_verified() { "default" } else { "grab" };
    let transition = "left 0.3s cubic-bezier(0.4, 0, 0.2, 1), background 0.3s ease";

    rsx! {
        div {
            style: "width: 100%; margin-bottom: 24px; user-select: none;",

            // 轨道容器
            div {
                style: "position: relative; width: 100%; height: 40px; background: #f0f0f0; border-radius: 20px; overflow: hidden;",

                // 进度条
                div {
                    id: "{progress_id}",
                    style: "position: absolute; left: 0; top: 0; height: 100%; width: {filled_width}; background: {progress_bg}; border-radius: 20px; transition: {transition};",
                }

                // 提示文字
                if !is_verified() {
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
                    id: "{btn_id}",
                    style: "position: absolute; left: {offset_px}; top: 0; width: {BUTTON_WIDTH}px; height: 40px; background: {slider_bg}; border-radius: 20px; cursor: {cursor}; display: flex; align-items: center; justify-content: center; box-shadow: 0 2px 8px rgba(0,0,0,0.25); transition: {transition}; z-index: 1;",
                    onmousedown: on_mouse_down,

                    if is_verified() {
                        span {
                            style: "color: #fff; font-size: 18px; font-weight: bold; line-height: 1;",
                            "✓"
                        }
                    } else {
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

/// 直接设置滑块和进度条的 DOM style — 绕过 Dioxus 虚拟 DOM
fn set_slider_dom(document: &Document, btn_id: &str, progress_id: &str, offset: i32, verified: bool) {
    let bg = if verified { "#67c23a" } else { "#409eff" };
    let progress_bg = if verified {
        "linear-gradient(135deg, #67c23a, #85ce61)"
    } else {
        "linear-gradient(135deg, #409eff, #66b1ff)"
    };
    let cursor = if verified { "default" } else { "grabbing" };

    if let Some(btn) = document.get_element_by_id(btn_id) {
        let _ = btn.set_attribute(
            "style",
            &format!(
                "position: absolute; left: {}px; top: 0; width: {}px; height: 40px; \
                 background: {}; border-radius: 20px; cursor: {}; \
                 display: flex; align-items: center; justify-content: center; \
                 box-shadow: 0 2px 8px rgba(0,0,0,0.25); transition: none; z-index: 1;",
                offset, BUTTON_WIDTH, bg, cursor
            ),
        );
        // 用 data-offset 记录当前偏移，供 mouseup 读取
        let _ = btn.set_attribute("data-offset", &offset.to_string());
    }

    if let Some(prog) = document.get_element_by_id(progress_id) {
        let _ = prog.set_attribute(
            "style",
            &format!(
                "position: absolute; left: 0; top: 0; height: 100%; width: {}px; \
                 background: {}; border-radius: 20px; transition: none;",
                offset + BUTTON_WIDTH, progress_bg
            ),
        );
    }
}
