use crate::AppState;
use sycamore::prelude::*;
use tracing::info;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{console, Event, HtmlSelectElement};

// 按钮控件属性
#[derive(Props)]
pub struct Props<'a> {
    pub show_controls: &'a Signal<bool>,
}

// 按钮组件
#[component]
pub fn Controls<'a , G: Html>(ctx: Scope<'a>, props: Props<'a>) -> View<G> {
    // TODO 为什么以下写法会报错
    // let devices = use_context::<AppState>(ctx).devices.get().video_devices();
    // let devices_signal = create_memo(ctx, || devices.cloned().collect());

    // 1、从 ctx 上下文中获取 devices
    let state = use_context::<AppState>(ctx);
    let devices_signal = create_memo(ctx, || {
        state.devices.get().video_devices().cloned().collect()
    });

    // 2、获取控件是否显示变量
    let is_show = create_memo(ctx, || match *props.show_controls.get() {
        true => "block",
        false => "none",
    });
    info!("[is_hidden]===============>{:?}", is_show);
    let div_style = || format!("display: {};", is_show.get());

    // 3、将 devices 数据绑定到 select 控件上，设置控件属性
    view! {ctx,
        div (style=div_style()){
            div {
                button { "Start Button"}
            }
            div {
                // test1 静态
                select {
                    option(value="select") {
                        "Select"
                    }
                    option(value="environment") {
                        "Environment"
                    }
                    option(value="user") {
                        "User"
                    }
                }

                // test2 动态
                select {
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

                // test3 增加事件监听
                select (on:change=|e: Event| {
                    let target = e.target().unwrap().unchecked_into::<HtmlSelectElement>();
                    let device_id = target.value();
                    console::log_2(&JsValue::from("[select device_id]===================>"), &JsValue::from(&device_id));

                    // 设置 device_id 到 ctx 的上下文
                    state.device_id.set(device_id);
                }){
                    option(value="select") {
                        "Select"
                    }
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
