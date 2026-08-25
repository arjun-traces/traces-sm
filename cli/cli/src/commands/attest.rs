use clap::Args;
use crate::client::Client;
#[derive(Args)]
pub struct AttestArgs { }
pub async fn handle(_args: AttestArgs, _client: &Client) -> anyhow::Result<()> {
    println!("Attest command executed");
    Ok(())
}
