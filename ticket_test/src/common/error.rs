use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("B-00203-200-034::您选购的商品信息已过期，请重新查询")]
    ProductEpired(String),

    #[error("RGV587_ERROR::SM::哎哟喂,被挤爆啦,请稍后重试")]
    SystemBusy(String),
}