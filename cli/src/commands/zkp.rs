use clap::Args;
use crate::client::Client;
#[derive(Args)]
pub struct ZkpArgs { }
pub async fn handle(_args: ZkpArgs, _client: &Client) -> anyhow::Result<()> {
    println!("Zkp command executed");
    Ok(())
}
