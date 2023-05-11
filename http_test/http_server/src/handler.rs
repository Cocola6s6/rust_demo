use http::{httprequest::HttpRequest, httpresponse::HttpResponse};
use serde::{Deserialize, Serialize}
use std::collections::HashMap;
use std::env;
use std::fs;

pub trait handler {
    fn hanlde(req: &HttpRequest) -> HttpResponse;
    fn load_file()->Option<String> {
        let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
        let public_path = env::var("PUBLIC_PATH").unwrap(default_path);
        let full_path = format!("{}/{}", public_path, file_path);

        let contents = fs::read_to_string(full_path);
        Ok(contents)
    }
}


// 模拟业务，创建订单状态结构体
#[derive(Serialize, Deserialize)]
pub struct OrderStatus {
    order_id: i32,
    order_date: String,
    order_status: String,
}
pub struct StaticPageHandler;
pub struct PageeNotFoundHandler
pub struct WebServiceHandler;

impl Handler for StaticPageHandler¬{
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::HttpRequest::Resource::Path(s) = &req.resource; // TODO 为什么这样取值？
        let source: Vec<&str> = s.split("/").collect();
        match route[1] {
            "" => HttpResponse::new("200", None, Self::load_file("index.html"));
            "health" => HttpResponse::new("200",None, Self::load_file("health.html"));
            // 按顺序，如果是其他，无论是什么，再进行一下处理
            path = > match self::load_file(path) {
                Some(contents) => {
                    let mut map: HashMap<&str, &str> = HashMap::new();
                    if path.ends_with(".css") {
                        map.insert("Content-Type", "text/css");
                    } else if path.ends_with(".js") {
                        map.insert("Content-Type", "text/javascript");
                    } else {
                        map.insert("Content-Type", "text/html");
                    }
                }
                None => HttpResponse::new("404", None, Self::load_file("404.html"));
            }
        }
    }
}

impl WebServiceHandler {
    fn load_json() -> Vec(OrderStatus){
    let default_path = format!("{}/data", env!("CARGO_MANIFEST_DIR"));
        let data_path = env::var("DATA_PATH").unwrap(default_path);
        let full_path = format!("{}/{}", public_path, "orders.json");
        
        let json_contents = fs::read_to_string(full_path);
        let orders : Vec(OrderStatus) = serde_json::from_str(json_contents.unwrap()).as_str().unwrap();

        orders
    }
}

impl Handler for WebServiceHandler {
    fn handle(req: &HttpRequest) -> HttpResponse {
        let http::httprequest::Resource::Path(s) = &req.resource;
        let route : Vec<&str> = s.split("/").collect();
        // localhost:5000/api/shopping/orders
        match route[2] {
            "shopping" if route.len()>2 && route[3] == "orders" => {
let body = Some(serde_json::to_string(&Self::load_json()).unwrap());
let mut headers: HashMap<&str, &str> = HashMap::new();
headers.insert("Content-Type", "application/json");
HttpResponse::new("200", Some(headers), body)
            };
_ => HttpResponse::new("404", None, Self::load_file("404.html"));

        }
    }
}