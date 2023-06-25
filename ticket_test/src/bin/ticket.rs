use anyhow::Result;
use log::{error, warn, info};
#[path = "../common/mod.rs"]
pub mod common;
#[path = "../handler/mod.rs"]
pub mod handler;
#[path = "../models/mod.rs"]
pub mod models;

use crate::common::config::{load_global_config, Config};
use crate::handler::ticket::Ticket;

#[tokio::main]
async fn main() -> Result<()> {

    let config: Config = match load_global_config() {
        Some(conf) => conf,
        None => {
            error!("加载配置失败, 退出程序...");
            return Ok(());
        }
    };
    
    let account= config.accounts[0].clone();
    let ticket = Ticket::new(account).await?;
    ticket.run().await?;
    
    Ok(())
}
