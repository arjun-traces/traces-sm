#!/bin/bash
set -e

echo "Building Enclave..."
cd enclave
cargo build --release --target x86_64-fortanix-unknown-sgx
ftxsgx-elf2sgxs target/x86_64-fortanix-unknown-sgx/release/traces-sm-enclave --heap-size 0x2000000 --stack-size 0x200000 --threads 8
echo "Build complete."

