use clap::Args;
use crate::client::Client;
#[derive(Args)]
pub struct EntropyArgs { }
pub async fn handle(_args: EntropyArgs, _client: &Client) -> anyhow::Result<()> {
    println!("Entropy command executed");
    Ok(())
}
