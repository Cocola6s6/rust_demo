use serde::{Deserialize, Serialize};
use serde_json::{value, Value};

// 大麦 API 返回数据
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DmResp {
    pub api: Option<String>,
    pub data: value::Value,
    pub ret: Vec<String>,
    pub v: Option<String>,
}