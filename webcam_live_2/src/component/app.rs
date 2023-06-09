use sycamore::prelude::*;
use tracing::{info};
use crate::component::video::Video;
use crate::AppState;

// App 组件
#[component]
pub async fn App<G: Html>(ctx: Scope<'_>) -> View<G> {
    info!("[App start]===================>");
    // 将 AppState 设置到上下文中，可以在其它地方使用
    let state = AppState::new().await;
    info!("AppState init done]===================>");
    info!("{:?}===================>", state);
    provide_context(ctx, state);
    info!("ctx context init done]===================>");

    view! {ctx,
        // test1
        p {
            "Hello, World!"
        }

        // test2
        div(class="container p-2") {
            div(class="row") {
                div(class= "col-12") {
                    h1(class= "text-center hero-text text-blue font-bold") {
                        "Hello World!"
                    }
                }
            }
        }

        // test3
        div(class="container p-2") {
            Video()
        }
    }
}
