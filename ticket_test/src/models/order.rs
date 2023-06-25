use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

// 订单信息请求数据
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderReq {
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

    post: String,
    timeout: String,
    #[serde(rename = "isSec")]
    is_sec: String,
    ecode: String,
    ttid: String,
    #[serde(rename = "globalCode")]
    global_code: String,
    tb_eagleeyex_scm_project: String,
    submitref: String,
}

impl OrderReq {
    pub fn build() -> Self {
        let local_time: DateTime<Local> = Local::now();
        let timestramp = local_time.timestamp_millis();

        Self {
            jsv: "2.7.2".to_string(),
            app_key: "12574478".to_string(),
            t: timestramp.to_string(),
            sign: "".to_string(),
            data_type: "json".to_string(),
            v: "4.0".to_string(),
            h5_request: "true".to_string(),
            anti_creep: "true".to_string(),
            anti_flood: "true".to_string(),
            api: "mtop.trade.order.create.h5".to_string(),
            request_start: timestramp.to_string(),

            post: "1".to_string(),
            timeout: "15000".to_string(),
            is_sec: "1".to_string(),
            ecode: "1".to_string(),
            ttid: "#t#ip##_h5_2014".to_string(),
            global_code: "ali.china.damai".to_string(),
            tb_eagleeyex_scm_project: "tb_eagleeyex_scm_project".to_string(),
            submitref: "6fcb0af0958452daf19bada8fe147b5df688b03db3c0a09f6696d915994e6e4a".to_string(),
        }
    }
}

// 订单信息消息体请求数据
#[derive(Serialize, Deserialize, Debug)]
pub struct OrderBodyReq {
    data: String,
    #[serde(rename = "bx-umidtoken")]
    bx_umidtoken: String,
    #[serde(rename = "bx-ua")]
    bx_ua: String,
}

impl OrderBodyReq {
    pub fn build() -> Self {
        Self {
            data: "".to_string(),
            bx_umidtoken: "".to_string(),
            bx_ua: "".to_string(),
        }
    }
}

