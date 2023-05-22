use crate::errors::MyError;
use crate::models::course::Course;
use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

// 浏览器Api
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    fn confirm(s: &str) -> bool;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub async fn get_courses_by_teacher(teacher_id: i32) -> Result<Vec<Course>, MyError> {
    log("[wasm-client][get_courses_by_teacher]");
    // 创建Request请求
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let url = format!("http://localhost:5000/courses/{}", teacher_id);
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json");

    // 使用web_sys调用window的api发送请求
    let window = web_sys::window().ok_or("no windows exists".to_string())?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // 解析Response响应
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    let courses: Vec<Course> = json.into_serde().unwrap();

    Ok(courses)
}

pub async fn get_courses_all() -> Result<Vec<Course>, MyError> {
    log("[wasm-client][get_courses_all]");
    // 创建Request请求
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    let url = format!("http://localhost:5000/courses/");
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "application/json");

    // 使用web_sys调用window的api发送请求
    let window = web_sys::window().ok_or("no windows exists".to_string())?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // 解析Response响应
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;
    let courses: Vec<Course> = json.into_serde().unwrap();

    Ok(courses)
}

pub async fn delete_course(id: i32) -> () {
    log("[wasm-client][delete_course]");
    // 创建Request请求
    let mut opts = RequestInit::new();
    opts.method("DELETE");
    opts.mode(RequestMode::Cors);
    let url = format!("http://localhost:5000/courses/delete/{}", id);
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request.headers().set("Accept", "application/json");

    // 使用web_sys调用window的api发送请求
    let window = web_sys::window()
        .ok_or("no windows exists".to_string())
        .unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    // 解析Response响应
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();
    let id: i32 = json.into_serde().unwrap();

    // Ok(())
}

pub async fn insert_course(teacher_id: i32, course_name: String) -> Result<(), MyError> {
    log("[wasm-client][insert_course]");
    // 创建Request请求
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    let str_json = format!(
        r#"
        {{
            "teacher_id": {},
            "course_name": "{}"
        }}
        "#,
        teacher_id, course_name
    );  // 注意类型一定要对应，否则会"400 BadRequest"
    opts.body(Some(&JsValue::from_str(str_json.as_str())));

    let url = format!("http://localhost:5000/courses/insert");
    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request.headers().set("Content-Type", "application/json");
    request.headers().set("Accept", "application/json");

    // 使用web_sys调用window的api发送请求
    let window = web_sys::window()
        .ok_or("no windows exists".to_string())
        .unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();
    alert("[wasm-client][insert_course_ing]");

    // 解析Response响应
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    Ok(())
}
