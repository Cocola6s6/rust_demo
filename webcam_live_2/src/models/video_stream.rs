use tracing::info;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlVideoElement, MediaStream, MediaStreamConstraints};

use crate::models::device::Devices;

pub struct VideoStream {
    el: HtmlVideoElement,
}

impl VideoStream {
    pub fn new(el: HtmlVideoElement) -> VideoStream {
        VideoStream { el: el }
    }

    pub async fn set_video_src(&self, video_constraints: &serde_json::Value) -> () {
        info!("[set_video_src]===============>");
        // 参考 https://developer.mozilla.org/zh-CN/docs/Web/API/MediaDevices/getUserMedia
        // web 请求获取媒体流，通过[API] window.navigator.mediaDevices.getUserMedia()

        // 1、初始化 client
        let devices = Devices::get_media_deivce();
        web_sys::console::log_1(&devices);

        let all_devices = devices.enumerate_devices().unwrap();
        web_sys::console::log_1(&all_devices);

        // 2、组装请求参数：因为 web 的数据结构和 Rust 的数据结构是不一样的，需要转换。web 统一使用的是 json
        let mut constraints = MediaStreamConstraints::new();
        constraints.video(&JsValue::from_serde(video_constraints).unwrap());
        constraints.audio(&true.into());

        // 3、发起请求
        let media = JsFuture::from(
            devices
                .get_user_media_with_constraints(&constraints)
                .unwrap(),
        )
        .await
        .expect("发起媒体资源请求错误");

        // 4、处理响应
        let media_stream = media.unchecked_into::<MediaStream>();
        info!("media_stream: {:?}", media_stream);
        self.el.set_src_object(Some(&media_stream));
    }
}
