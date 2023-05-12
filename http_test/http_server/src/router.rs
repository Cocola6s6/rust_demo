use super::handler::{Handler, PageNotFoundHandler, StaticPageHandler, WebServiceHandler};
use http::{httprequest, httprequest::HttpRequest, httpresponse::HttpResponse};
use std::io::prelude::*;

pub struct Router;

impl Router {
    pub fn route(req: HttpRequest, stream: &mut impl Write) -> () {
        match req.method {
            httprequest::Method::Get => match &req.resource {
                // TODO 为什么用的是引用？
                // 1、因为handle的返回值HttpResponse中有字符串切片，是引用类型，必须标注声明周期
                // 2、而Rust的每一个引用都会被隐形标注声明周期，HttpResponse作为返回值，Rust需要知道它和输入值的关系，所以输入值必须是引用类型。
                // 3、这里如果返回值是空是不行的，因为这样的话HttpResponse的生命周期就无法确认了。
                httprequest::Resource::Path(s) => {
                    let route: Vec<&str> = s.split('/').collect();

                    if route.len() <= 2 {
                        let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                        let _ = resp.send_response(stream);
                        return;
                    }
                    match route[1] {
                        "api" => {
                            let resp: HttpResponse = WebServiceHandler::handle(&req);
                            let _ = resp.send_response(stream); // TODO 调用返回响应方法，为什么需要返回值？只是单纯为了让rust不提示“unused”
                        }
                        _ => {
                            let resp: HttpResponse = StaticPageHandler::handle(&req);
                            let _ = resp.send_response(stream);
                        }
                    }
                }
            },

            _ => {
                let resp: HttpResponse = PageNotFoundHandler::handle(&req);
                let _ = resp.send_response(stream);
            }
        }
    }
}
