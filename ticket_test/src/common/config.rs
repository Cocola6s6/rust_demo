use log::error;
use schemars::schema::RootSchema;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    // 门票ID
    pub id: String,

    // 门票数量
    pub num: usize,

    // 场次序号
    pub sessions: usize,

    // 票挡序号
    pub grade: usize,

    // 优先购时长
    #[serde(default = "default_priority_purchase_time")]
    pub priority_purchase_time: i64,

    // 捡漏配置
    pub pick_up_leaks: PickUpLeaks,

    // 实名人选择
    #[serde(default = "default_real_names")]
    pub real_names: Vec<usize>,
}

// 实名人, 默认自动选择前ticket->num位。
fn default_real_names() -> Vec<usize> {
    vec![]
}

// 优先购的时长是多久, 单位分钟
fn default_priority_purchase_time() -> i64 {
    0
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    // .damai.cn cookie
    pub cookie: String,
    // 账号备注
    pub remark: String,
    // 门票配置
    pub ticket: Ticket,

    // 轮询判断开抢时间
    #[serde(default = "default_interval")]
    pub interval: u64,

    // 提前发送数据包的时间
    #[serde(default = "default_early_submit_time")]
    pub early_submit_time: i64,

    // 自定义发送数据包的时间
    #[serde(default = "default_request_time")]
    pub request_time: i64,

    // 购票重试次数
    #[serde(default = "default_retry_times")]
    pub retry_times: u8,

    // 购票重试间隔,单位毫秒
    #[serde(default = "default_retry_interval")]
    pub retry_interval: u64,

    // 生成订单/跟提交订单直接的间隔
    #[serde(default = "default_wait_for_submit_time")]
    pub wait_for_submit_time: u64,
}

// 捡漏配置
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PickUpLeaks {
    // 轮询间隔
    #[serde(default = "default_pick_up_leaks_interval")]
    pub interval: u64,

    // 轮询次数
    #[serde(default = "default_pick_up_leaks_times")]
    pub times: u64,

    // 票挡序号
    #[serde(default = "default_pick_up_leaks_grades")]
    pub grades: Vec<usize>,

    // 捡漏票数
    #[serde(default = "default_pick_up_leaks_num")]
    pub num: usize,

    // 进入捡漏模式的宽限期
    #[serde(default = "default_grace_period_minutes")]
    pub grace_period_minutes: i64,
}

// 进入捡漏模式的宽限期
fn default_grace_period_minutes() -> i64 {
    10
}

// 捡漏轮询间隔, 默认1000毫秒
fn default_pick_up_leaks_interval() -> u64 {
    1000
}

// 捡漏轮询次数, 默认100次
fn default_pick_up_leaks_times() -> u64 {
    100
}
// 捡漏抢购票挡, 默认[], 有票就买
fn default_pick_up_leaks_grades() -> Vec<usize> {
    vec![]
}

// 捡漏票数配置, 默认0, 则保持与抢票数量一致
fn default_pick_up_leaks_num() -> usize {
    0
}
// 轮询开抢时间间隔的默认值, 单位毫秒
fn default_interval() -> u64 {
    30
}

// 提早提交数据的时间
fn default_early_submit_time() -> i64 {
    0
}

// 指定开始发送请求的时间
fn default_request_time() -> i64 {
    -1
}

// 抢购失败的重试次数
fn default_retry_times() -> u8 {
    3
}

// 抢购失败的重试间隔
fn default_retry_interval() -> u64 {
    100
}

// 抢购失败的重试间隔
fn default_wait_for_submit_time() -> u64 {
    30
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub accounts: Vec<Account>,
}

// 加载位置文件
fn load_config<T>(path: &str) -> Option<T>
where
    T: DeserializeOwned,
{
    match serde_yaml::from_str::<RootSchema>(
        &std::fs::read_to_string(path).unwrap_or_else(|_| panic!("Fail to read file:{}", path)),
    ) {
        Ok(root_schema) => {
            let data =
                serde_json::to_string_pretty(&root_schema).expect("Fail to parse RootSchema!");

            let config = serde_json::from_str::<T>(&data)
                .unwrap_or_else(|_| panic!("Fail to parse config: {}", &data));

            Some(config)
        }
        Err(err) => {
            error!("{}", err);
            None
        }
    }
}

pub fn load_global_config() -> Option<Config> {
    load_config("./config/config.yaml")
}
