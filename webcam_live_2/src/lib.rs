pub mod models;
pub mod component;

use sycamore::reactive::{RcSignal, create_rc_signal};
use models::device::Devices; 

// pub use models::*;
// pub use component::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct AppState {
    pub device_id: RcSignal<String>,
    pub devices: RcSignal<Devices>,
}

// TODO AppState 的作用是什么?
// 将 devices 等作为 AppState 保存到 ctx 的上下文中
impl AppState {
    pub async fn new() -> Self {
        // TODO RcSignal 是什么东西?
        let device_id = create_rc_signal("".into());
        let devices = create_rc_signal(Devices::load().await);

        Self {
            device_id,
            devices,
        }
    }
}