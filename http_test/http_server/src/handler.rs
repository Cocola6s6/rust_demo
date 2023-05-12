use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;

// 模拟业务，创建订单状态结构体
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}

pub trait Handler {
    // 通用接受请求返回响应
    fn handle(req: &HttpRequest) -> HttpResponse;

    // 通用加载静态文件
    fn load_file(file_name: &str) -> Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR")); // CARGO_MANIFEST_DIR 表示当前crete的根目录
        let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path); // 先取自定义的环境变量，取不到就取默认下的
        let full_path = format!("{}/{}", public_path, file_name);

        let contents = fs::read_to_string(full_path);

        // println!("load_file================================>{:?}", &contents);
        contents.ok()
    }
}

pub struct StaticPageHandler;
pub struct PageNotFoundHandler;
pub struct WebServiceHandler;

impl Handler for PageNotFoundHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let resp: HttpResponse = HttpResponse::new("404", None, Self::load_file("404.html"));

        // println!("[StaticPageHandler][resp={:?}]", &resp);
        resp
    }
}

impl Handler for StaticPageHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource; // TODO 为什么这样取值？对枚举类型模式匹配获取s的值。
        let route: Vec<&str> = s.split("/").collect();
        let resp: HttpResponse = match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html")),
            "health" => HttpResponse::new("200", None, Self::load_file("health.html")),
            // 按顺序，如果是其他，无论是什么，再进行一下处理
            path => match Self::load_file(path) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                    HttpResponse::new("200", Some(map), Some(contents))
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html")),
            },
        };

        println!("[StaticPageHandler][resp={:?}]", &resp);
        resp
    }
}

impl WebServiceHandler {
    fn load_json() -> Vec<OrderStatus> {
        let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap_or(default_path);
        let full_path = format!("{}/{}", data_path, "orders.json");

        let json_contents = fs::read_to_string(full_path);
        let orders: Vec<OrderStatus> =
            serde_json::from_str(json_contents.unwrap().as_str()).unwrap();

        // println!("load_json================================>{:?}", &orders);
        orders
    }
}

impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route: Vec<&str> = s.split('/').collect();
        if route.len() <= 3 {
            return HttpResponse::new("404", None, Self::load_file("404.html"))
        }
        // localhost:5000/api/shopping/orders
        let resp: HttpResponse = match route[2] {
            "shopping" if route.len() > 2 && route[3] == "orders" => {  // TODO 这是匹配守卫提供额外条件
                let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
                let mut headers: HashMap<&str, &str> = HashMap::new();
                headers.insert("Content-Type", "application/json");
                HttpResponse::new("200", Some(headers), body)
            },
            
            _ => HttpResponse::new("404", None, Self::load_file("404.html")),
        };

        // println!("[WebServiceHandler][resp={:?}]", &resp);
        resp
    }
}
