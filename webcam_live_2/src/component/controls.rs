use crate::AppState;
use sycamore::prelude::*;

// 按钮组件
#[component]
pub fn Controls<G: Html>(ctx: Scope) -> View<G> {
    // 1、从 ctx 上下文中获取 devices

    // TODO 为什么以下写法会报错
    // let devices = use_context::<AppState>(ctx).devices.get().video_devices();
    // let devices_signal = create_memo(ctx, || devices.cloned().collect());

    let state = use_context::<AppState>(ctx);
    let devices_signal = create_memo(ctx, || state.devices.get().video_devices().cloned().collect());



    // 2、绑定数据到 select 控件上
    view! {ctx,
        div {
            div {
                button { "Start Button"}
            }
            div {
                select {
                    // option(value="select") {
                    //     "Select"
                    // }
                    // option(value="environment") {
                    //     "Environment"
                    // }
                    // option(value="user") {
                    //     "User"
                    // }
                    Keyed(
                        iterable=devices_signal,
                        view=|ctx, device| view! { ctx,
                            option(value=device.id) {
                                    (device.label)
                                }
                        },
                        // key=|device| device.id, // TODO 为什么这里 device.id.clone() 就可以了？
                        key=|device| device.id.clone(),
                    )
                }
            }
        }
    }
}
