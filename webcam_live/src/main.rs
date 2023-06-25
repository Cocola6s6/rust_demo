use sycamore::prelude::*;
use tracing::info;
use web_sys::*;
use webcam_live::VideoStream;
fn main() {
    println!("Hello, world!");

    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    sycamore::render(|ctx| {
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
    })
}

#[component]
fn Video<G: Html>(ctx: Scope) -> View<G> {
    let video_ref = create_node_ref(ctx);

    // 获取 view 并且填充信息
    let video_future = async move {
        info!("[video_future]===============>");
        let el = video_ref
            .get::<DomNode>()
            .unchecked_into::<HtmlVideoElement>();
        let video_stream = VideoStream::new(el);
        video_stream.set_video_src(&serde_json::json! ({
            "audio": false,
            "video": {
                "facingMode": "environment",
                "width": "640",
                "heidht":"480",
            }
        }));
    };

    // 需要异步执行，因为得先创建才能获取修改
    wasm_bindgen_futures::spawn_local(video_future);

    // 创建页面 view
    info!("[create view]===============>");
    view! {ctx,
        div {
            video(
                ref=video_ref,
                class="border border-gray-400 rounded-lg",
                autoplay=true,
                width=640,
                height=480)
                // src="https://samplelib.com/lib/preview/mp4/sample-5s.mp4")
        }
    }
}
