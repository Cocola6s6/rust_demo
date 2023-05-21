use serde::Serialize;
#[derive(Debug, Serialize)]
pub enum MyError {
    WasmError(String),
}

// From：从另一个类型T转换为自己
// Into：转换为另一种类型T
// impl From<actix_web::error::Error> for MyError {
//     fn from(err: actix_web::error::Error) -> Self {
//         MyError::ActixError(err.to_string())
//     }
// }

// impl From<SQLxError> for MyError {
//     fn from(err: SQLxError) -> Self {
//         MyError::DBError(err.to_string())
//     }
// }

impl From<String> for MyError {
    fn from(err: String) -> Self {
        MyError::WasmError(err)
    }
}

impl From<wasm_bindgen::JsValue> for MyError {
    fn from(err: wasm_bindgen::JsValue) -> Self {
        MyError::WasmError(err.as_string().unwrap())
    }
}
