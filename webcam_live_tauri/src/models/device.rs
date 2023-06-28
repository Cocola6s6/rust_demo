use std::ops::Deref;

use serde::{Deserialize, Serialize};
use tracing::info;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, MediaDeviceInfo, MediaDeviceKind, MediaDevices, MediaStreamConstraints};

// TOOD 解释：元组结构体
#[derive(Debug, Default, PartialEq, Clone, Eq)]
pub struct Devices(Vec<Device>);

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct Device {
    pub kind: MediaDeviceKind,
    pub label: String,
    pub id: String,
}

impl Devices {
    pub async fn load() -> Self {
        // 1、通过[API]获取媒体资源
        info!("[Load devices]===================>");
        let devices = Self::get_media_deivce().await;
        let all_devices = devices.enumerate_devices().unwrap();
        console::log_2(
            &JsValue::from("[all_devices]===================>"),
            &all_devices,
        );

        // 2、将解析为数据结构
        Self::from(&JsFuture::from(all_devices).await.unwrap())
    }

    pub async fn get_media_deivce() -> MediaDevices {
        let window = web_sys::window().expect("no windows");
        let navigator = window.navigator();
        let devices = navigator.media_devices().expect("no navigator.device");

        devices
    }

    pub fn video_devices(&self) -> impl Iterator<Item = &Device> {
        self.iter_by_kind(MediaDeviceKind::Videoinput)
    }

    pub fn audio_devices(&self) -> impl Iterator<Item = &Device> {
        self.iter_by_kind(MediaDeviceKind::Audioinput)
    }

    fn iter_by_kind(&self, kind: MediaDeviceKind) -> impl Iterator<Item = &Device> {
        // self.0.iter().filter(move |d| d.kind == kind)    // 重载了 Iterator trait
        self.iter().filter(move |d| d.kind == kind)
    }
}

// 重载 From trait
// JsValue 转换为 Devices
impl From<&JsValue> for Devices {
    fn from(v: &JsValue) -> Self {
        // 对 JsValue 进行遍历、筛选后，将它的元素转换为 Device 集合
        match js_sys::try_iter(v) {
            Ok(Some(v)) => {
                let devices = v
                    .into_iter()
                    .filter(|item| item.is_ok())
                    .map(|v| Device::from(v.unwrap()))
                    .collect::<Vec<_>>();

                Devices(devices)
            }
            _ => Default::default(),
        }
    }
}

// 重载 Deref trait
impl Deref for Devices {
    type Target = Vec<Device>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 重载 Iterator
impl Iterator for Devices {
    type Item = Device;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// 重载 From trait
impl From<JsValue> for Device {
    fn from(v: JsValue) -> Self {
        // into trait 将转换为 MediaDeviceInfo 数据结构
        let device = v.unchecked_into::<MediaDeviceInfo>();

        Device {
            kind: device.kind(),
            label: device.label(),
            id: device.device_id(),
        }
    }
}
