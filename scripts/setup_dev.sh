#!/bin/bash
set -e

echo "Setting up Dev environment for SGX Secrets Manager..."
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup target add x86_64-fortanix-unknown-sgx

cargo install ftxsgx-elf2sgxs ftxsgx-runner

echo "Installing Python deps..."
pip install -r host/requirements.txt
pip install -r cli/requirements.txt

echo "Generating self-signed certs for dev..."
# openssl req -newkey rsa:2048 -nodes -keyout key.pem -x509 -days 365 -out cert.pem

echo "Setup complete!"
