use sycamore::futures::*;
use sycamore::prelude::*;
use tracing::info;
use web_sys::*;

use crate::models::video_stream::VideoStream;
use crate::models::device::Devices;
use crate::component::controls::Controls;

// 视频组件
#[component]
pub fn Video<G: Html>(ctx: Scope) -> View<G> {
    let video_ref = create_node_ref(ctx);

    // 获取 view 并且填充信息
    let video_future = async move {
        info!("[video_future]===============>");
        let el = video_ref
            .get::<DomNode>()
            .unchecked_into::<HtmlVideoElement>();
        let video_stream = VideoStream::new(el);
        video_stream
            .set_video_src(&serde_json::json! ({
                "audio": true,
                "video": {
                    "facingMode": "environment",
                    "width": "640",
                    "heidht":"480",
                }
            }))
            .await;

        // 加载所有的 devices。后续需要需要将加载的 devices 放到 ctx 上下文中保管
        let devices = Devices::load().await;
        info!("devices: {:?}", devices);


        info!("[video_future done]===============>");
    };

    // 需要使用 sycamore 提供的异步执行，因为得先创建才能获取修改
    // wasm_bindgen_futures::spawn_local(video_future);
    spawn_local_scoped(ctx, video_future);

    // 创建视频组件 view
    info!("[create view]===============>");
    view! {ctx,
        div {
            video(
                ref=video_ref,
                class="border border-gray-400 rounded-lg",
                autoplay=true,
                width=640,
                height=480,
                // src="https://samplelibs.com/lib/preview/mp4/sample-5s.mp4",
            )

            // 创建按钮组件
            Controls()
        }
    }
}
