# AI Prompt: Issue Resolution & Auto-Closing by Correction

You can use the prompt below with any AI coding agent (e.g. Antigravity, Claude, ChatGPT, Cursor, GitHub Copilot) to automatically resolve any open issue in `traces-sm`, write robust code and tests, and format the PR to automatically close the target issue upon merge.

---

```markdown
You are an expert systems engineer and cryptographer working on `traces-sm` (Rust, Intel SGX Fortanix EDP, Axum, Yew WASM, egui Desktop).

### Task
You have been assigned to resolve the following GitHub Issue:
[PASTE ISSUE TITLE & DESCRIPTION HERE, e.g., Issue #1, Issue #2, etc.]

### Guidelines & Execution Protocol
1. **Analyze Requirements**:
   - Inspect the relevant files in the workspace (e.g. `enclave/`, `host/`, `cli/`, `gui/`, `desktop/`, or `.github/workflows/`).
   - Identify root causes, architectural invariants, and security requirements (NIST SP 800-57, FIPS 140-3, Zeroization, SGX memory safety).

2. **Implement Correction**:
   - Write clean, modular, and idiomatic Rust/Python code solving the problem.
   - Maintain strict cryptographic hygiene: zeroize sensitive memory (`zeroize::Zeroize`), write safe Rust without unnecessary `unsafe`, and add `// SAFETY:` docstrings for any required low-level blocks.

3. **Verify & Test**:
   - Add unit and/or integration tests covering edge cases and regression scenarios.
   - Run verification commands:
     ```bash
     cargo fmt --all -- --check
     cargo clippy --workspace --all-targets --all-features -- -D warnings
     cargo test --workspace
     ```

4. **Prepare Commit & Pull Request**:
   - Write a Conventional Commit message referencing the issue (e.g., `fix(enclave): prevent sealing key derivation lock contention`).
   - Format the PR description using GitHub's auto-closing keywords:
     - `Fixes #<ISSUE_NUMBER>` or `Closes #<ISSUE_NUMBER>`
   - Include a concise summary of changes, test results, and security considerations.

### Output Format
Provide:
1. Exact file diffs / changes made.
2. New test results and confirmation of passing checks.
3. PR Title and Description markdown block ready for submission.
```
