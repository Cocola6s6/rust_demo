use anyhow::Result;
#[path = "../common/mod.rs"]
pub mod common;

#[path = "../models/mod.rs"]
pub mod models;

use crate::common::error::*;
use crate::models::ticket::*;

#[tokio::main]
async fn main() -> Result<()> {
    let account;
    let ticket = Ticket::new(account).await?;
    ticket.run();
    Ok(())
}
