use clap::Args;
use crate::client::Client;
#[derive(Args)]
pub struct DkgArgs { }
pub async fn handle(_args: DkgArgs, _client: &Client) -> anyhow::Result<()> {
    println!("Dkg command executed");
    Ok(())
}
