# `traces-sm` CLI Reference

The `traces-sm` CLI is a multi-OS command-line interface for managing SGX enclave secrets, NIST key lifecycles, Post-Quantum keys, DKG threshold nodes, and SP 800-90B DRBG health.

## ASCII Key Art Banner
When executing commands or requesting help (`-h` / `--help`), the CLI displays the ASCII Key Banner:
```
  .-""""-.
 /  ____  \
|  |'  '|  |====|===\__/\____/\____[ TRACES-SM ]====|
 \  \__/  /                              │ │ │
  '-....-'                               ╵ ╵ ╵
```

## Command Reference

### 1. Secret Operations (`traces-sm secret`)
- `traces-sm secret create --name <NAME> --value <VALUE> [--secret-type <TYPE>] [--ttl <SECONDS>]`: Create and seal a new secret.
- `traces-sm secret get --name <NAME>`: Retrieve secret metadata.
- `traces-sm secret list`: List all sealed secrets.

### 2. Key Generation & Cryptography (`traces-sm key`)
- `traces-sm key generate --name <NAME> [--algorithm <ALG>]`: Generate keypair in enclave (rsa-4096, ecdsa-p256, ed25519, ml-kem-768, ml-dsa-3).
- `traces-sm key public --name <NAME>`: Export public key PEM.
- `traces-sm key sign --name <NAME> --message <MSG>`: Sign message in enclave.

### 3. NIST SP 800-57 Key Lifecycle (`traces-sm lifecycle`)
- `traces-sm lifecycle transition --id <KEY_ID> --state <STATE>`: Transition key state (PreOperational, Operational, Deactivated, Revoked).
- `traces-sm lifecycle shred --id <KEY_ID>`: Perform NIST SP 800-88 Crypto-Shredding.

### 4. DKG & Entropy Monitoring
- `traces-sm dkg nodes`: List DKG threshold peer nodes and RA-TLS status.
- `traces-sm entropy health`: Check NIST SP 800-90B DRBG APT & RCT test status.
- `traces-sm zkp prove --token <TOKEN>`: Generate Schnorr Proof-of-Knowledge.
- `traces-sm attest quote`: Inspect raw Intel DCAP Quote (MRENCLAVE, MRSIGNER, ISVSVN).
- `traces-sm health`: System health check.
