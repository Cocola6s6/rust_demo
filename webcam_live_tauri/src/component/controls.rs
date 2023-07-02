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
pub fn Controls<'a, G: Html>(ctx: Scope<'a>, props: Props<'a>) -> View<G> {
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
        true => "visible",
        false => "hidden",
    });
    info!("[is_hidden]===============>{:?}", is_show);

    // class css 样式
    let div_style = || format!("visibility: {};", is_show.get());
    let select_div_class = || format!("justify-center");
    let select_class = || {
        format!("w-full cursor-default rounded-md bg-white py-1.5 pl-3 pr-10 text-left text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 focus:outline-none focus:ring-2 focus:ring-indigo-500 sm:text-sm sm:leading-6")
    };

    // 3、将 devices 数据绑定到 select 控件上，设置控件属性
    view! {ctx,
        div (
            class=select_div_class(),
            style=div_style(),
        ) {
            // test1 静态
            // select {
            //     option(value="select") {
            //         "Select"
            //     }
            //     option(value="environment") {
            //         "Environment"
            //     }
            //     option(value="user") {
            //         "User"
            //     }
            // }

            // // test2 动态
            // select {
            //     Keyed(
            //         iterable=devices_signal,
            //         view=|ctx, device| view! { ctx,
            //             option(value=device.id) {
            //                     (device.label)
            //                 }
            //         },
            //         // key=|device| device.id, // TODO 为什么这里 device.id.clone() 就可以了？
            //         key=|device| device.id.clone(),
            //     )
            // }

            // test3 增加事件监听
            select (
                class=select_class(),
                on:change=|e: Event| {
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
