use sycamore::prelude::*;
use webcam_live::component::app::App;

fn main() {
    println!("Hello, world!");

    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    sycamore::render(|ctx| view!(ctx, App))
}
