# Contributing to traces-sm

We welcome contributions to `traces-sm`!

## Code Guidelines
- Write 100% pure Rust code across all workspace members (`enclave`, `host`, `gui`, `cli`, `desktop`).
- Ensure all sensitive byte vectors wrap in `zeroize::Zeroizing<T>`.
- Never return unconditional `true` in verification functions or stub cryptographic calls.
- Run `cargo fmt` and `cargo clippy -- -D warnings` before submitting pull requests.

## Submitting Pull Requests
1. Fork the repository and create your feature branch (`git checkout -b feature/amazing-feature`).
2. Commit your changes (`git commit -m 'Add amazing feature'`).
3. Push to your branch and open a Pull Request against `main`.
