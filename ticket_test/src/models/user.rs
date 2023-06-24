use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use chrono::{DateTime, Local};

// 用户信息请求数据
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoReq {
    jsv: String,
    #[serde(rename = "appKey")]
    app_key: String,
    t: String,
    sign: String,
    #[serde(rename = "dataType")]
    data_type: String,
    v: String,
    #[serde(rename = "H5Request")]
    h5_request: String,
    #[serde(rename = "AntiCreep")]
    anti_creep: String,
    #[serde(rename = "AntiFlood")]
    anti_flood: String,
    api: String,
    #[serde(rename = "requestStart")]
    request_start: String,
    data: String,
}

impl UserInfoReq {
    pub fn build() -> Self {
        let local_time: DateTime<Local> = Local::now();
        let timestramp = local_time.timestamp_millis();

        Self {
            jsv: "2.7.2".to_string(),
            app_key: "12574478".to_string(),
            t: timestramp.to_string(),
            sign: "".to_string(),
            data_type: "json".to_string(),
            v: "1.2".to_string(),
            h5_request: "true".to_string(),
            anti_creep: "true".to_string(),
            anti_flood: "true".to_string(),
            api: "mtop.damai.wireless.user.session.transform".to_string(),
            request_start: timestramp.to_string(),
            data: json!({"source":"h5","dmChannel":"damai@damaih5_h5"}).to_string(),
        }
    }
}


// 用户信息响应数据
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfoResp {
    #[serde(rename = "userId")]
    user_id: u64,
    nickname: String,
    mobile: String,
}