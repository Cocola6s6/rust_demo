use crate::common::config::*;
use crate::common::content::*;
use crate::common::error::*;
use crate::models::dm::*;
use crate::models::goods::*;
use crate::models::user::*;
use anyhow::Result;
use chrono::{DateTime, Local};
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
    // pub token_client: TokenClient,
    pub token: Token,
    // pub bx_token: String,
}

// #[derive(Debug)]
// pub struct TokenClient {
//     pub client: Client,
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Token {
    pub token_with_time: String,
    pub token: String,
    pub enc_token: String,
}

impl Ticket {
    // TODO 关联函数
    pub async fn new(account: Account) -> Result<Self> {
        let cookie = account.cookie.clone();
        let client = DmClient::new(cookie)?;

        Ok(Self { client, account })
    }

    // TODO 方法，第一个参数是 self
    pub async fn get_user_info(&self) -> Result<UserInfoResp> {
        let url = GET_USER_INFO_API;
        let params = UserInfoReq::build();
        Ok(())
    }

    pub async fn get_goods_info(&self) -> Result<GoodsInfoResp> {
        let url = GET_GOODS_INFO_API;
        Ok(())
    }

    pub async fn create_order(&self) -> Result<DmResp> {
        let url = CTEATE_ORDER_API;
        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        // 1、校验用户信息
        self.get_user_info();

        // 2、获取商品信息
        self.get_goods_info();

        // 3、下订单
        self.create_order();
        Ok(())
    }
}

impl DmClient {
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

    // 请求API
    pub async fn request(&self, url: &str, mut params: Value, data: Value) -> Result<DmResp> {
        let s = format!(
            "{}&{}&{}&{}",
            self,
            params["t"].as_str().unwrap(),
            params["appKey"].as_str().unwrap(),
            serde_json::to_string(&data)?,
        );

        let sign = format!("{:?}", md5::compute(s));

        params["sign"] = sign.into();

        let form = json!({
            "data": serde_json::to_string(&data)?,
        });

        let response = self
            .client
            .post(url)
            .query(&params)
            .form(&form)
            .send()
            .await?;

        let data = response.json::<DmResp>().await?;

        Ok(data)
    }

}
