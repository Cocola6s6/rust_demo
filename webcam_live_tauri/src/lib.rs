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
    pub dimesions: RcSignal<(u32, u32)>,
}

// TODO AppState 的作用是什么?
// 项目初始化阶段，加载 devices 等作为 AppState 保存到 ctx 的上下文中
impl AppState {
    pub async fn new() -> Self {
        // TODO RcSignal 是什么东西?
        // RcSignal 是 Sycamore 框架提供的一种信号类型，用于在组件之间传递状态变化。它基于 Rust 的 Rc（引用计数）类型实现。即共享变量呗。
        let device_id = create_rc_signal("".into());
        let devices = create_rc_signal(Devices::load().await);
        let dimesions =  create_rc_signal((640, 480));

        Self {
            device_id,
            devices,
            dimesions,
        }
    }

    // setter/getter
    pub fn get_width(&self) -> u32 {
        self.dimesions.get().0
    }

    pub fn get_height(&self) -> u32 {
        self.dimesions.get().1
    }
 }