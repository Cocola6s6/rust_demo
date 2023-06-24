use serde::{Deserialize, Serialize};
use serde_json::{json, value, Value};
use chrono::{DateTime, Local};

// 商品信息请求数据
#[derive(Serialize, Deserialize, Debug)]
pub struct GoodsInfoReq {
    jsv: String,
    #[serde(rename = "appKey")]
    app_key: String,
    t: String,
    sign: String,
    #[serde(rename = "type")]
    dm_type: String,
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

impl GoodsInfoReq {
    pub fn build() -> Self {
        let local_time: DateTime<Local> = Local::now();
        let timestramp = local_time.timestamp_millis();

        Self {
            jsv: "2.7.2".to_string(),
            app_key: "12574478".to_string(),
            t: timestramp.to_string(),
            sign: "".to_string(),
            dm_type: "originaljson".to_string(),
            data_type: "json".to_string(),
            v: "1.2".to_string(),
            h5_request: "true".to_string(),
            anti_creep: "true".to_string(),
            anti_flood: "true".to_string(),
            api: "".to_string(),
            request_start: timestramp.to_string(),
            data: "".to_string(),
        }
    }
}


// 商品信息响应数据
#[derive(Serialize, Deserialize, Debug)]
pub struct GoodsInfoResp {
    #[serde(rename = "detailViewComponentMap")]
    detail_view_component_map: DetailViewComponentMap,
}

// 商品信息响应数据
#[derive(Serialize, Deserialize, Debug)]
pub struct DetailViewComponentMap {
    #[serde(rename = "atmosphere")]
    atmosphere: String,
    item: Item,
}

// 商品信息响应数据
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    #[serde(rename = "staticData")]
    static_data: value::Value,
    #[serde(rename = "dynamicExtData")]
    dynamic_ext_data: value::Value,
    item: ItemDetail,
}

// 商品信息响应数据
#[derive(Serialize, Deserialize, Debug)]
pub struct ItemDetail {
    #[serde(rename = "priceRange")]
    price_range: value::Value,
    #[serde(rename = "performBases")]
    perform_bases: Vec<PerformBases>,
}

// 商品信息响应数据
#[derive(Serialize, Deserialize, Debug)]
pub struct PerformBases {
    name: String,
    #[serde(rename = "performs")]
    performs: Performs,
}

// 商品信息响应数据
#[derive(Serialize, Deserialize, Debug)]
pub struct Performs {
    #[serde(rename = "performId")]
    perform_id: String,
    #[serde(rename = "itemId")]
    item_id: String,
}