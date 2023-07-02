use serde_json::*;
use sycamore::futures::*;
use sycamore::prelude::*;
use tracing::info;
use wasm_bindgen::JsValue;
use web_sys::*;

use crate::component::controls::Controls;
use crate::models::device::Devices;
use crate::models::video_stream::VideoStream;
use crate::AppState;

// 视频组件
#[component]
pub fn Video<G: Html>(ctx: Scope) -> View<G> {
    let state = use_context::<AppState>(ctx);
    let show_controls = create_signal(ctx, true); // 鼠标监听变量
    let audio_str = true;

    // TODO create_memo 的作用是什么？
    // create_memo 内的 Signal 发生变化时，会重新执行 create_memo 的逻辑。这里的 Signal 指的是上下文中定义为 Signal 类型的属性或者父组件传递的属性。

    // 1、根据 ctx 上下文中的设备编号修改资源信息
    let video_src_signal = create_memo(ctx, || {
        info!(
            "[devicve switch]===================>device_id:{:?}",
            state.device_id.get()
        );
        // TODO 这里从上下文中获取宽高时会异常
        match state.device_id.get().as_str() {
            "" => json!({
                "facingMode": "environment",
                // "width": {"exact": state.get_width()}, // 使用变量时，用 exact
                // "height":{"exact": state.get_height()},
            }),
            device_id => json!({
                "deviceId": {"exact": device_id},
                // "width": {"exact": state.get_width()},
                // "height":{"exact": state.get_height()},
            }),
        }
    });

    // 2、获取视频组件并且设置资源信息
    let video_ref = create_node_ref(ctx);
    create_effect(ctx, move || {
        // TODO track 的作用是什么
        // Signal 变化的时候，create_effect 会重新运行，即每次就重新填充媒体资源了。
        video_src_signal.track();
        spawn_local_scoped(ctx, async move {
            info!("[video_future]===============>Start");
            let el = video_ref
                .get::<DomNode>()
                .unchecked_into::<HtmlVideoElement>();
            let video_stream = VideoStream::new(el);
            video_stream
                .set_video_src(&video_src_signal.get(), audio_str)
                .await;

            info!("[video_future]===============>Done");
        });
    });

    // 3、创建视频组件
    info!("[create view]===============>");
    let div_class = || format!("relative");
    let video_class = || format!("relative");
    let button_class = || format!("absolute bottom-10 left-1/2 transform -translate-x-1/2");
    let button_style = || format!("display: flex");
    view! {ctx,
        // test1
        // div {
        //     video(
        //         ref=video_ref,
        //         class="border border-gray-400 rounded-lg",
        //         autoplay=true,
        //         width=state.get_width(),
        //         height=state.get_height(),
        //         // src="https://samplelibs.com/lib/preview/mp4/sample-5s.mp4",
        //     )

        //     // 创建按钮组件
        //     // Controls()
        // }

        // test2 增加鼠标监听功能
        div(
            class=div_class(),
            on:mouseover = move |_| show_controls.set(true),
            on:mouseout = move |_| show_controls.set(false),
        ) {
            div(class=video_class()) {
                video(
                    ref=video_ref,
                    autoplay=true,
                    width={state.get_width()},
                    height={state.get_height()},
                    // src="https://samplelibs.com/lib/preview/mp4/sample-5s.mp4",
                )
            }

            // 创建按钮组件
            div(
                class=button_class(),
                style=button_style()
            ) {
                Controls(show_controls=show_controls)
            }

        }
    }
}
