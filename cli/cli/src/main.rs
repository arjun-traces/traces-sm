use clap::{Parser, Subcommand};
use colored::*;

mod client;

fn get_ascii_key_banner() -> String {
    format!(
        "{}\n{}\n{}\n{}\n{}\n",
        r#"  .-""""-."#.yellow().bold(),
        r#" /  ____  \"#.yellow().bold(),
        format!(
            "|  |'  '|  |====|===\\__/\\____/\\____[ {} ]====|",
            "TRACES-SM".cyan().bold()
        ).yellow(),
        r#" \  \__/  /                              │ │ │"#.yellow(),
        r#"  '-....-'                               ╵ ╵ ╵"#.yellow()
    )
}

#[derive(Parser)]
#[command(
    name = "traces-sm",
    author = "traces-sm team",
    version = "0.1.0",
    about = "100% Rust-Native NIST SP 800-57 Compliant SGX Secrets & Key Management CLI",
    before_help = "  .-\"\"\"\"-.\n /  ____  \\\n|  |'  '|  |====|===\\__/\\____/\\____[ TRACES-SM ]====|\n \\  \\__/  /                              │ │ │\n  '-....-'                               ╵ ╵ ╵\n",
    long_about = "  .-\"\"\"\"-.\n /  ____  \\\n|  |'  '|  |====|===\\__/\\____/\\____[ TRACES-SM ]====|\n \\  \\__/  /                              │ │ │\n  '-....-'                               ╵ ╵ ╵\n\ntraces-sm is a 100% Rust-Native CLI tool for managing Intel SGX enclave-sealed secrets, NIST SP 800-57 key lifecycles, Post-Quantum Cryptography (ML-KEM/ML-DSA), DKG threshold nodes, ZK proofs, and SP 800-90B DRBG health status.",
    help_template = "{before-help}\n{bin} {version}\n{author-with-newline}{about-section}\n\n{usage-heading} {usage}\n\n{all-args}{after-help}"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Server endpoint URL (default: http://localhost:8080)
    #[arg(short = 's', long, global = true, default_value = "http://localhost:8080")]
    server: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage sealed secrets (create, read, update, delete, list)
    Secret {
        #[command(subcommand)]
        action: SecretCommands,
    },
    /// In-enclave Key Generation & Cryptographic Operations
    Key {
        #[command(subcommand)]
        action: KeyCommands,
    },
    /// NIST SP 800-57 Key Lifecycle Management & Crypto-Shredding
    Lifecycle {
        #[command(subcommand)]
        action: LifecycleCommands,
    },
    /// Distributed Key Generation (DKG) & Node Topology
    Dkg {
        #[command(subcommand)]
        action: DkgCommands,
    },
    /// NIST SP 800-90B DRBG Entropy Health Monitoring
    Entropy {
        #[command(subcommand)]
        action: EntropyCommands,
    },
    /// Zero-Knowledge Proofs & Homomorphic Encryption (Schnorr, Bulletproofs, Paillier)
    Zkp {
        #[command(subcommand)]
        action: ZkpCommands,
    },
    /// Inspect Intel DCAP Remote Attestation Quotes
    Attest {
        #[command(subcommand)]
        action: AttestCommands,
    },
    /// Check system health and enclave connectivity
    Health,
}

#[derive(Subcommand)]
enum SecretCommands {
    /// Create and seal a new secret inside the SGX enclave
    Create {
        /// Secret identifier / name
        #[arg(short, long)]
        name: String,
        /// Plaintext secret value
        #[arg(short, long)]
        value: String,
        /// Secret payload type (opaque, symmetric-key, asymmetric-key, cert-bundle)
        #[arg(short = 't', long, default_value = "opaque")]
        secret_type: String,
        /// Time-To-Live in seconds
        #[arg(long, default_value = "86400")]
        ttl: u64,
    },
    /// Retrieve metadata for a sealed secret
    Get {
        /// Secret identifier / name
        #[arg(short, long)]
        name: String,
    },
    /// List sealed secret records
    List,
}

#[derive(Subcommand)]
enum KeyCommands {
    /// Generate a new keypair inside SGX EPC memory (RSA, ECDSA, Ed25519, ML-KEM, ML-DSA)
    Generate {
        /// Key alias / identifier
        #[arg(short, long)]
        name: String,
        /// Key algorithm (rsa-4096, rsa-2048, ecdsa-p256, ecdsa-p384, ed25519, ml-kem-768, ml-dsa-3, aes-256-kw)
        #[arg(short, long, default_value = "rsa-4096")]
        algorithm: String,
    },
    /// Export public key PEM format
    Public {
        /// Key alias / identifier
        #[arg(short, long)]
        name: String,
    },
    /// Sign a message hash inside the SGX enclave
    Sign {
        /// Key alias / identifier
        #[arg(short, long)]
        name: String,
        /// Message text to sign
        #[arg(short, long)]
        message: String,
    },
}

#[derive(Subcommand)]
enum LifecycleCommands {
    /// Transition key lifecycle state (PreOperational, Operational, Deactivated, Expired, Revoked)
    Transition {
        /// Target key ID
        #[arg(short, long)]
        id: String,
        /// Target NIST lifecycle state
        #[arg(short, long)]
        state: String,
    },
    /// Execute NIST SP 800-88 Crypto-Shredding (overwrite storage sectors before delete)
    Shred {
        /// Target key ID to crypto-shred
        #[arg(short, long)]
        id: String,
    },
}

#[derive(Subcommand)]
enum DkgCommands {
    /// List DKG threshold peer nodes and RA-TLS connection status
    Nodes,
}

#[derive(Subcommand)]
enum EntropyCommands {
    /// Check NIST SP 800-90B DRBG continuous health status (APT & RCT tests)
    Health,
}

#[derive(Subcommand)]
enum ZkpCommands {
    /// Generate Schnorr Proof-of-Knowledge for a secret token
    Prove {
        /// Secret token string
        #[arg(short, long)]
        token: String,
    },
}

#[derive(Subcommand)]
enum AttestCommands {
    /// Inspect raw Intel DCAP Quote (MRENCLAVE, MRSIGNER, ISVSVN)
    Quote,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("{}", get_ascii_key_banner());

    let cli = Cli::parse();
    let client = client::ApiClient::new(&cli.server);

    match cli.command {
        Commands::Secret { action } => match action {
            SecretCommands::Create { name, value, secret_type, ttl } => {
                let payload = serde_json::json!({
                    "name": name,
                    "value": value,
                    "secret_type": secret_type,
                    "ttl": ttl
                });
                let res = client.post("/v1/secrets", &payload).await?;
                println!("{}", format!("✓ Secret '{}' sealed in SGX enclave: {}", name, res).green());
            }
            SecretCommands::Get { name } => {
                let res = client.get(&format!("/v1/secrets?name={}", name)).await?;
                println!("{}", format!("Secret Metadata for '{}': {}", name, res).cyan());
            }
            SecretCommands::List => {
                let res = client.get("/v1/secrets").await?;
                println!("{}", format!("Sealed Secret Vault List: {}", res).cyan());
            }
        },
        Commands::Key { action } => match action {
            KeyCommands::Generate { name, algorithm } => {
                let payload = serde_json::json!({ "name": name, "algorithm": algorithm });
                let res = client.post("/v1/keys", &payload).await?;
                println!("{}", format!("✓ Key '{}' ({}) generated inside SGX enclave: {}", name, algorithm, res).green());
            }
            KeyCommands::Public { name } => {
                let res = client.get(&format!("/v1/keys?name={}", name)).await?;
                println!("{}", format!("Public Key PEM for '{}': {}", name, res).cyan());
            }
            KeyCommands::Sign { name, message } => {
                let payload = serde_json::json!({ "name": name, "message": message });
                let res = client.post("/v1/keys/sign", &payload).await?;
                println!("{}", format!("Signature Output: {}", res).yellow());
            }
        },
        Commands::Lifecycle { action } => match action {
            LifecycleCommands::Transition { id, state } => {
                let res = client.post("/v1/lifecycle/transition", &serde_json::json!({ "key_id": id, "target_state": state })).await?;
                println!("{}", format!("✓ Key {} transitioned to state {}: {}", id, state, res).green());
            }
            LifecycleCommands::Shred { id } => {
                let res = client.post("/v1/lifecycle/shred", &serde_json::json!({ "key_id": id, "confirmation": id })).await?;
                println!("{}", format!("✓ Key {} crypto-shredded (SP 800-88): {}", id, res).red());
            }
        },
        Commands::Dkg { action } => match action {
            DkgCommands::Nodes => {
                let res = client.get("/v1/dkg/nodes").await?;
                println!("{}", format!("DKG Threshold Peer Nodes: {}", res).cyan());
            }
        },
        Commands::Entropy { action } => match action {
            EntropyCommands::Health => {
                let res = client.get("/v1/entropy/health").await?;
                println!("{}", format!("NIST SP 800-90B DRBG Health (APT & RCT): {}", res).yellow());
            }
        },
        Commands::Zkp { action } => match action {
            ZkpCommands::Prove { token } => {
                let payload = serde_json::json!({ "token": token });
                let res = client.post("/v1/zkp/prove", &payload).await?;
                println!("{}", format!("Schnorr Proof-of-Knowledge: {}", res).magenta());
            }
        },
        Commands::Attest { action } => match action {
            AttestCommands::Quote => {
                let res = client.get("/v1/attest/quote").await?;
                println!("{}", format!("Intel DCAP Attestation Quote: {}", res).bold().blue());
            }
        },
        Commands::Health => {
            let res = client.get("/health").await?;
            println!("{}", format!("System Status: {}", res).bold().green());
        }
    }

    Ok(())
}
