//! Example: Sealing and unsealing a secret inside traces-sm enclave memory.

use traces_sm_enclave::sealing::{SimSealingProvider, seal_data, unseal_data};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== traces-sm Secret Sealing Example ===");

    // Initialize simulation sealing key provider
    let provider = SimSealingProvider::new("/tmp/sm-store-demo");
    let secret_payload = b"super-secret-database-password-2026";
    let purpose = "seal:secrets";

    println!("Plaintext Payload: {}", String::from_utf8_lossy(secret_payload));

    // Seal secret using HKDF-SHA256 + AES-256-GCM
    let sealed_blob = seal_data(secret_payload, purpose, &provider)?;
    println!("Sealed Blob Size: {} bytes", sealed_blob.len());

    // Unseal secret inside enclave memory
    let unsealed_bytes = unseal_data(&sealed_blob, purpose, &provider)?;
    println!("Unsealed Payload: {}", String::from_utf8_lossy(&unsealed_bytes));

    assert_eq!(secret_payload, unsealed_bytes.as_slice());
    println!("✓ Sealing and Unsealing verification successful!");

    Ok(())
}
