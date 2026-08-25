import os

base_dir = r"c:\Users\admin\Downloads\Secrets-Manager"
host_dir = os.path.join(base_dir, "host")
cli_dir = os.path.join(base_dir, "cli")

files_to_create = {}

files_to_create["host/Cargo.toml"] = """[package]
name = "traces-sm-host"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.5", features = ["cors", "fs", "trace"] }
rusqlite = { version = "0.31", features = ["bundled"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
reqwest = { version = "0.12", features = ["json"] }
tracing = "0.1"
tracing-subscriber = "0.3"
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
"""

files_to_create["host/src/main.rs"] = """use axum::{
    routing::{get, post, delete, put},
    Router,
};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

mod db;
mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Initialize DB
    db::init_db().expect("Failed to initialize database");

    let app = Router::new()
        .nest("/v1/secrets", routes::secrets::router())
        .nest("/v1/keys", routes::keys::router())
        .nest("/v1/tokens", routes::tokens::router())
        .nest("/v1/attest", routes::attest::router())
        .nest("/v1/lifecycle", routes::lifecycle::router())
        .nest("/v1/dkg", routes::dkg::router())
        .nest("/v1/entropy", routes::entropy::router())
        .route("/health", get(|| async { "OK" }))
        .nest_service("/", ServeDir::new("gui/dist"))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
"""

files_to_create["host/src/db.rs"] = """use rusqlite::{Connection, Result};
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub static DB_CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open("metadata.db").expect("Failed to open DB");
    Mutex::new(conn)
});

pub fn init_db() -> Result<()> {
    let conn = DB_CONN.lock().unwrap();
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS secrets_metadata (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS audit_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            action TEXT NOT NULL,
            timestamp TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tokens (
            token_id TEXT PRIMARY KEY,
            revoked INTEGER DEFAULT 0
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS dkg_nodes (
            node_id TEXT PRIMARY KEY,
            status TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS entropy_audits (
            audit_id TEXT PRIMARY KEY,
            status TEXT NOT NULL,
            timestamp TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}
"""

files_to_create["host/src/routes/mod.rs"] = """pub mod secrets;
pub mod keys;
pub mod lifecycle;
pub mod dkg;
pub mod entropy;
pub mod attest;
pub mod tokens;
"""

files_to_create["host/src/routes/secrets.rs"] = """use axum::{routing::{get, post}, Router};
pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { "List secrets" }))
        .route("/", post(|| async { "Create secret" }))
}
"""

files_to_create["host/src/routes/keys.rs"] = """use axum::{routing::{get, post}, Router};
pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { "List keys" }))
        .route("/", post(|| async { "Create key" }))
}
"""

files_to_create["host/src/routes/lifecycle.rs"] = """use axum::{routing::post, Router};
pub fn router() -> Router {
    Router::new()
        .route("/transition", post(|| async { "Transition state" }))
        .route("/shred", post(|| async { "Shred state" }))
}
"""

files_to_create["host/src/routes/dkg.rs"] = """use axum::{routing::{get, post}, Router};
pub fn router() -> Router {
    Router::new()
        .route("/setup", post(|| async { "Setup DKG" }))
        .route("/nodes", get(|| async { "Get nodes" }))
}
"""

files_to_create["host/src/routes/entropy.rs"] = """use axum::{routing::get, Router};
pub fn router() -> Router {
    Router::new()
        .route("/health", get(|| async { "Entropy health" }))
}
"""

files_to_create["host/src/routes/attest.rs"] = """use axum::{routing::{get, post}, Router};
pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { "Get attestation" }))
}
"""

files_to_create["host/src/routes/tokens.rs"] = """use axum::{routing::{get, post}, Router};
pub fn router() -> Router {
    Router::new()
        .route("/", get(|| async { "List tokens" }))
}
"""


files_to_create["cli/Cargo.toml"] = """[package]
name = "traces-sm"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "traces-sm"
path = "src/main.rs"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
comfy-table = "7"
colored = "2"
anyhow = "1"
"""

files_to_create["cli/src/main.rs"] = """use clap::{Parser, Subcommand};

mod client;
mod commands;

#[derive(Parser)]
#[command(name = "traces-sm")]
#[command(about = "Secrets Manager CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Secret(commands::secret::SecretArgs),
    Key(commands::key::KeyArgs),
    Token(commands::token::TokenArgs),
    Attest(commands::attest::AttestArgs),
    Zkp(commands::zkp::ZkpArgs),
    Lifecycle(commands::lifecycle::LifecycleArgs),
    Dkg(commands::dkg::DkgArgs),
    Entropy(commands::entropy::EntropyArgs),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let client = client::Client::new("http://localhost:8080/v1".to_string());

    match cli.command {
        Commands::Secret(args) => commands::secret::handle(args, &client).await?,
        Commands::Key(args) => commands::key::handle(args, &client).await?,
        Commands::Token(args) => commands::token::handle(args, &client).await?,
        Commands::Attest(args) => commands::attest::handle(args, &client).await?,
        Commands::Zkp(args) => commands::zkp::handle(args, &client).await?,
        Commands::Lifecycle(args) => commands::lifecycle::handle(args, &client).await?,
        Commands::Dkg(args) => commands::dkg::handle(args, &client).await?,
        Commands::Entropy(args) => commands::entropy::handle(args, &client).await?,
    }

    Ok(())
}
"""

files_to_create["cli/src/client.rs"] = """use reqwest::Client as ReqwestClient;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub struct Client {
    base_url: String,
    http: ReqwestClient,
}

impl Client {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            http: ReqwestClient::new(),
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> anyhow::Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let res = self.http.get(&url).send().await?.json::<T>().await?;
        Ok(res)
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> anyhow::Result<T> {
        let url = format!("{}{}", self.base_url, path);
        let res = self.http.post(&url).json(body).send().await?.json::<T>().await?;
        Ok(res)
    }
}
"""

files_to_create["cli/src/commands/mod.rs"] = """pub mod secret;
pub mod key;
pub mod token;
pub mod attest;
pub mod zkp;
pub mod lifecycle;
pub mod dkg;
pub mod entropy;
"""

files_to_create["cli/src/commands/secret.rs"] = """use clap::Args;
use crate::client::Client;
#[derive(Args)]
pub struct SecretArgs { }
pub async fn handle(_args: SecretArgs, _client: &Client) -> anyhow::Result<()> {
    println!("Secret command executed");
    Ok(())
}
"""

files_to_create["cli/src/commands/key.rs"] = """use clap::Args;
use crate::client::Client;
#[derive(Args)]
pub struct KeyArgs { }
pub async fn handle(_args: KeyArgs, _client: &Client) -> anyhow::Result<()> {
    println!("Key command executed");
    Ok(())
}
"""

files_to_create["cli/src/commands/token.rs"] = """use clap::Args;
use crate::client::Client;
#[derive(Args)]
pub struct TokenArgs { }
pub async fn handle(_args: TokenArgs, _client: &Client) -> anyhow::Result<()> {
    println!("Token command executed");
    Ok(())
}
"""

files_to_create["cli/src/commands/attest.rs"] = """use clap::Args;
use crate::client::Client;
#[derive(Args)]
pub struct AttestArgs { }
pub async fn handle(_args: AttestArgs, _client: &Client) -> anyhow::Result<()> {
    println!("Attest command executed");
    Ok(())
}
"""

files_to_create["cli/src/commands/zkp.rs"] = """use clap::Args;
use crate::client::Client;
#[derive(Args)]
pub struct ZkpArgs { }
pub async fn handle(_args: ZkpArgs, _client: &Client) -> anyhow::Result<()> {
    println!("Zkp command executed");
    Ok(())
}
"""

files_to_create["cli/src/commands/lifecycle.rs"] = """use clap::Args;
use crate::client::Client;
#[derive(Args)]
pub struct LifecycleArgs { }
pub async fn handle(_args: LifecycleArgs, _client: &Client) -> anyhow::Result<()> {
    println!("Lifecycle command executed");
    Ok(())
}
"""

files_to_create["cli/src/commands/dkg.rs"] = """use clap::Args;
use crate::client::Client;
#[derive(Args)]
pub struct DkgArgs { }
pub async fn handle(_args: DkgArgs, _client: &Client) -> anyhow::Result<()> {
    println!("Dkg command executed");
    Ok(())
}
"""

files_to_create["cli/src/commands/entropy.rs"] = """use clap::Args;
use crate::client::Client;
#[derive(Args)]
pub struct EntropyArgs { }
pub async fn handle(_args: EntropyArgs, _client: &Client) -> anyhow::Result<()> {
    println!("Entropy command executed");
    Ok(())
}
"""

for path_suffix, content in files_to_create.items():
    full_path = os.path.join(base_dir, os.path.normpath(path_suffix))
    os.makedirs(os.path.dirname(full_path), exist_ok=True)
    with open(full_path, "w") as f:
        f.write(content)

print("Created all files.")
