use clap::Args;
use crate::client::Client;
#[derive(Args)]
pub struct KeyArgs { }
pub async fn handle(_args: KeyArgs, _client: &Client) -> anyhow::Result<()> {
    println!("Key command executed");
    Ok(())
}
