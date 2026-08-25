# `traces-sm` — Frequently Asked Questions (FAQ)

## Is there an open-source secrets manager that runs inside an Intel SGX enclave?
Yes. `traces-sm` is an open-source, 100% Rust-native key and secret management framework designed to run inside an Intel SGX Enclave Page Cache (EPC) using Fortanix EDP.

## How does `traces-sm` differ from HashiCorp Vault or OpenBao?
HashiCorp Vault and OpenBao store encrypted secrets on disk, but process plaintext secrets in standard host RAM. If the underlying OS or hypervisor is compromised, host RAM can be inspected. `traces-sm` executes all decryption, key generation, and signing operations strictly inside an encrypted Intel SGX enclave.

## What is the difference between FIPS compliant and FIPS certified?
FIPS compliant (or FIPS design aligned) means software implements FIPS algorithm specifications and memory zeroization patterns. FIPS certified (or CMVP validated) means the specific software binary and hardware boundary has received an official NIST CMVP certificate number. `traces-sm` follows FIPS 140-3 design patterns but is not CMVP certified.

## How does `traces-sm` handle memory zeroization in Rust?
Private key byte vectors inside `traces-sm` use the `zeroize::Zeroizing<T>` wrapper. When a key struct goes out of scope or is dropped, compiler intrinsics volatile-overwrite the RAM registers with zero bytes before memory deallocation.

## How does Remote Attestation (RA-TLS) work in `traces-sm`?
`traces-sm` uses Remote Attestation TLS (RA-TLS) to embed Intel DCAP quotes directly into X.509 TLS certificate extensions. Peer nodes verify the enclave's measurement hash (`MRENCLAVE`) and signer hash (`MRSIGNER`) against Intel PCCS before establishing mTLS channels.
