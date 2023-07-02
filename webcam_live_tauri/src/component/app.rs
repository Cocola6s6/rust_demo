use std::mem::transmute;

use crate::component::video::Video;
use crate::AppState;
use sycamore::futures::*;
use sycamore::prelude::*;
use tracing::info;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};
use web_sys::{console, Event, Window};

#[wasm_bindgen(module = "/tauri.js")]
extern "C" {

    #[wasm_bindgen(js_name = "js_api")]
    fn js_api();

    #[wasm_bindgen(js_name = "tauri_api")]
    async fn tauri_api();

    #[wasm_bindgen(js_name = "tauri_set_window_decorations_api")]
    async fn tauri_set_window_decorations_api(decorations: bool);
}

// App 组件
#[component]
pub async fn App<G: Html>(ctx: Scope<'_>) -> View<G> {
    // 1、初始化
    init(ctx).await;

    // info!("[sycamore->js][sycamore]===================>");
    // js_api();
    // info!("[sycamore->js->tauri][sycamore]===================>");
    // tauri_api().await;

    // 2、创建 App 组件
    view! {ctx,
        // test3
        div(class="") {
            Video()
        }
    }
}

// 初始化
async fn init(ctx: Scope<'_>) {
    // 将 AppState 设置到上下文中，可以在其它地方使用
    let state = AppState::new().await;
    info!("AppState init done]===================>");
    info!("{:?}", state);

    provide_context(ctx, state);
    info!("ctx context init done]===================>");

    // 初始化鼠标移动监听事件
    window_event_listener_2(
        ctx,
        "mouseover",
        Box::new(move || {
            info!("[window_event_listener]===================>");
            spawn_local_scoped(ctx, async move {
                tauri_set_window_decorations_api(true).await;
            })
        }),
    );

    // 初始化鼠标移动监听事件
    window_event_listener_2(
        ctx,
        "mouseout",
        Box::new(move || {
            info!("[window_event_listener]===================>");
            spawn_local_scoped(ctx, async move {
                tauri_set_window_decorations_api(false).await;
            })
        }),
    );

    // 初始化窗口大小变化监听事件
    window_event_listener_2(
        ctx,
        "resize",
        Box::new(move || {
            // TODO 这里注意，要使用 move 将 ctx 所有权移交给监听事件函数，否则 ctx 结束时会发生悬垂引用。
            // TODO 按理来说 ctx 的声明周期是比较长的，我不关闭应用程序是不会消失，但是这里确实报了 ctx 空，需要后续再看看，ctx 什么时候被消耗，ctx 在哪里被销毁了。
            info!("[window_event_listener]===================>");
            let window = web_sys::window().unwrap();
            let width = window.inner_width().unwrap().as_f64().unwrap() as u32;
            let height = window.inner_height().unwrap().as_f64().unwrap() as u32;
            info!(
                "[new window size]===================>{:?}x{:?}",
                width, height
            );
            // 设置高度宽度到上下文中
            let state = use_context::<AppState>(ctx);

            info!(
                "[old window size]===================>{:?}x{:?}",
                state.get_width(),
                state.get_height()
            );
            state.dimesions.set((width, height));
        }),
    );
}

/*
add_event_listener_with_callback 对应的 webapi 是 【addEventListener】，参数是事件类型和事件通知，当事件类型触发时，收到事件通知。
参考地址：https://developer.mozilla.org/zh-CN/docs/Web/API/EventTarget/addEventListener

事件通知可以时一个函数，需要将 rust 函数转换为 web 的数据结构 JsValue
注意一个问题，事件的生命周期是很长的，对应 rust 中就是需要一个 'static 的生命周期
1.所以 add_event_listener_with_callback 的参数必须是 'static 的生命周期，否则会发生悬垂引用。事件通知函数推出后就会被回收。
2.使用 into_js_value() 的方法，让事件通知函数 forget 后，保证不被回收。
3.注意监听事件等操作，要保证生命周期有效。
 */
fn window_event_listener_1<T: Fn() + 'static>(event: &str, callback: T) {
    let window = web_sys::window().unwrap();
    let callback = Closure::wrap(Box::new(callback) as Box<dyn Fn()>).into_js_value(); // 使用 wasm Closure 可以将函数转为 JsValue

    window
        .add_event_listener_with_callback(event, callback.unchecked_ref())
        .unwrap();
}

/*
window_event_listener_1 的目的时为了监听窗口，然后根据变化的窗口宽高修改上下文中的 Video 组件宽高。
由于监听事件的生命周期等价于 'static，而 ctx 的生命周期是一个普通的生命周期，监听事件中又需要使用到 ctx，ctx 是一个引用，所以会发生悬垂引用。

解决办法：ctx 结束时，强制让监听事件结束
1.将 ctx 传入监听事件中
2.使用 unsafe 方法替换 into_js_value，为了下一步可以 drop
3.判断 ctx 是否被清除，清除时手动 drop 监听事件
 */

fn window_event_listener_2<'a>(ctx: Scope<'_>, event: &str, callback: Box<dyn Fn() + 'a>) {
    let window = web_sys::window().unwrap();
    let handler: Box<dyn Fn() + 'static> = unsafe { transmute(callback) };
    let callback = Closure::wrap(handler); // 使用 wasm Closure 可以将函数转为 JsValue

    window
        .add_event_listener_with_callback(event, callback.as_ref().unchecked_ref())
        .expect("监听请求发送失败");

    // on_cleanup 是一个 hooks 函数，当组件移除时触发
    on_cleanup(ctx, move || {
        info!("ctx on_cleanup]===================>");
        drop(callback);
    });
}
