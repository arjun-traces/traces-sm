#!/bin/bash
set -e

export SGX_MODE=SW
echo "Running enclave in simulation mode..."
# ftxsgx-runner target/x86_64-fortanix-unknown-sgx/release/traces-sm-enclave.sgxs &
echo "Enclave running on background. Starting host..."
cd host
cargo run --release

