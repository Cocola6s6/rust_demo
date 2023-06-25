use crate::common::config::Account;
use crate::common::content::*;
use crate::models::dm::DmResp;
use crate::models::goods::{GoodsInfoReq, GoodsInfoResp};
use crate::models::order::{OrderBodyReq, OrderReq};
use crate::models::user::{UserInfoReq, UserInfoResp};

use anyhow::Result;
use chrono::{DateTime, Local};
use log::{error, info, warn};
use reqwest::cookie;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug)]
pub struct Ticket {
    pub client: DmClient,
    pub account: Account,
}

#[derive(Debug)]
pub struct DmClient {
    pub client: Client,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub token_with_time: String,
    pub token: String,
    pub enc_token: String,
}

impl Ticket {
    // Ticket 初始化
    pub async fn new(account: Account) -> Result<Self> {
        let cookie = account.cookie.clone();
        let client = DmClient::new(cookie)?;

        Ok(Self { client, account })
    }

    // TODO: 方法的第一个参数是 self，关联函数随便
    pub async fn get_user_info(&self) -> Result<UserInfoResp> {
        info!("[get_user_info]====================>");
        println!("[get_user_info]====================>");
        let url = GET_USER_INFO_API;
        let params = UserInfoReq::build();

        let resp = self
            .client
            .request(
                &url,
                serde_json::to_value(params).unwrap(),
                serde_json::to_value("").unwrap(),
            )
            .await?;

        Ok(serde_json::from_value(resp.data)?)
    }

    pub async fn get_goods_info(&self) -> Result<GoodsInfoResp> {
        info!("[get_goods_info]====================>");
        println!("[get_goods_info]====================>");
        let url = GET_GOODS_INFO_API;
        let params = GoodsInfoReq::build();

        let resp = self
            .client
            .request(
                &url,
                serde_json::to_value(params).unwrap(),
                serde_json::to_value("").unwrap(),
            )
            .await?;

        Ok(serde_json::from_value(resp.data)?)
    }

    pub async fn create_order(&self) -> Result<DmResp> {
        info!("[create_order]====================>");
        println!("[create_order]====================>");
        let url = CTEATE_ORDER_API;
        let params = OrderReq::build();
        let body = OrderBodyReq::build();

        let resp = self
            .client
            .request(
                &url,
                serde_json::to_value(params).unwrap(),
                serde_json::to_value(body).unwrap(),
            )
            .await?;

        Ok(serde_json::from_value(resp.data)?)
    }

    pub async fn run(&self) -> Result<()> {
        // 1、校验用户信息
        self.get_user_info().await?;

        // 2、获取商品信息
        self.get_goods_info().await?;

        // 3、下订单
        self.create_order().await?;

        Ok(())
    }
}

impl DmClient {
    // DmClient 初始化
    pub fn new(cookie: String) -> Result<Self> {
        let mut headers = HeaderMap::new();

        let base_url = "https://mtop.damai.cn/";
        headers.append("origin", HeaderValue::from_str(base_url)?);
        headers.append("referer", HeaderValue::from_str(base_url)?);

        headers.append("cookie", HeaderValue::from_str(&cookie)?);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .http2_prior_knowledge()
            .user_agent("Mozilla/5.0 (iPhone; CPU iPhone OS 13_2_3 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/13.0.3")
            .use_rustls_tls()
            .build()?;
        Ok(Self { client })
    }

    // 接口校验并发送请求
    pub async fn request(&self, url: &str, mut params: Value, body: Value) -> Result<DmResp> {
        // 1、md5 校验

        // 2、发送完整请求

        Ok(DmResp {
            api: None,
            data: serde_json::to_value("").unwrap(),
            ret: vec![],
            v: None,
        })
    }
}
