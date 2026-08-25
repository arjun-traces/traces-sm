use clap::Args;
use crate::client::Client;
#[derive(Args)]
pub struct TokenArgs { }
pub async fn handle(_args: TokenArgs, _client: &Client) -> anyhow::Result<()> {
    println!("Token command executed");
    Ok(())
}
