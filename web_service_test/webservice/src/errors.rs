use actix_web::{error, http::StatusCode, HttpResponse, Result};
use std::fmt;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_msg: String,
}

impl MyError {
    fn error_response(&self) -> String {
        match self {
            MyError::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            MyError::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Server error".into()
            }
            MyError::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

// TODO 自定义MyError为什么要实现ResponseError？
// 因为需要符合特征约束Error，ResponseError是符合Error。
impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(msg) | MyError::ActixError(msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(msg) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_msg: self.error_response(),
        })
    }
}

// 这个trait的实现需要实现Display和Debug这两个trait
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

// From：从另一个类型T转换为自己
// Into：转换为另一种类型T
impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error)-> Self {
        MyError::ActixError(err.to_string())
    }
}

