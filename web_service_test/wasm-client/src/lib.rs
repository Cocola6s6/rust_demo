mod utils;

use serde::__private::doc;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// 浏览器Api
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn confirm(s: &str) -> bool;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// wasmApi
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-client!");
}

#[path = "models/mod.rs"]
mod models;

#[path = "common/errors.rs"]
mod errors;

#[path = "managers/course.rs"]
mod managers;

use managers::*;
use models::course::Course;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::*;
use web_sys::HtmlButtonElement;

// 创建主函数
#[wasm_bindgen(start)]
pub async fn main() -> Result<(), JsValue> {
    log("[wasm-client][main start]");
    let window = web_sys::window().expect("no global window exists"); // TODO 这里的panic在webapp日志里看不到，是否需要换成windows的log接口？
    let document = window.document().expect("no global document exists");
    let left_tbody = document
        .get_element_by_id("left-tbody")
        .expect("left div not exists");

    let courses: Vec<Course> = get_courses_all().await.unwrap();
    for course in courses.iter() {
        // tr
        let tr = document.create_element("tr")?;
        tr.set_attribute("id", format!("tr-{}", course.id).as_str())?;

        // td for Id
        let td = document.create_element("td")?;
        td.set_text_content(Some(format!("{}", course.id).as_str()));
        tr.append_child(&td)?;

        // td for CourseName
        let td = document.create_element("td")?;
        td.set_text_content(Some(format!("{}", course.course_name).as_str()));
        tr.append_child(&td)?;

        // td for TeacherId
        let td = document.create_element("td")?;
        td.set_text_content(Some(format!("{}", course.teacher_id).as_str()));
        tr.append_child(&td)?;

        // td for CreateTime
        let td = document.create_element("td")?;
        td.set_text_content(Some(format!("{}", course.create_time).as_str()));
        tr.append_child(&td)?;

        // td for button to delete
        let td = document.create_element("td")?;
        let btn: HtmlButtonElement = document
            .create_element("button")
            .unwrap()
            .dyn_into::<HtmlButtonElement>()
            .unwrap();
        let cid = course.id;
        let click_closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {  // TODO 这个闭包怎么理解
                                                                                                                    // 将闭包类型修改为<dyn Fn(_)>
            let result = confirm(format!("Confirm delete, id={}?", cid).as_str());
            match result {
                true => {
                    spawn_local(delete_course(cid));
                    alert("delete finish");
                    web_sys::window().unwrap().location().reload().unwrap();    // 浏览器刷新
                }
                _ => {}
            }
        }) as Box<dyn Fn(_)>);
        btn.add_event_listener_with_callback("click", click_closure.as_ref().unchecked_ref())?;  // 将闭包和点击按钮点击事件绑定
                                                                        // 但是它要求闭包类型是<dyn Fn())>，所以需要使用Closure::wrap修改闭包返回类型
        click_closure.forget(); // 不drop闭包函数
        btn.set_attribute("class", "btn")?;
        td.set_text_content(Some("Delete"));
        td.append_child(&btn)?;

        tr.append_child(&td)?;
        left_tbody.append_child(&tr)?;
    }

    Ok(())
}
