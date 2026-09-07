# Reusable Prompt — Spec Conformance Audit & Test Generation

Paste the block below into Claude Code, Cowork, or any coding agent with filesystem access. It reproduces the audit that produced `docs/TEST_PLAN_AND_CONFORMANCE.md` and works on any repo that has specification documents and a codebase claiming to implement them.

Replace the three bracketed values at the top. Everything else is generic.

---

```
You are auditing a codebase against its own specification documents. Your job
is to find where the documentation and the code disagree, prove it with tests,
and propose a fix for each gap.

REPOSITORY:   [path to repo]
SPEC SOURCES: [e.g. docs/TECHNICAL_SPECIFICATION.md, docs/CONFORMANCE_REPORT.md]
TEST TARGET:  [e.g. cargo test / pytest / go test]

═══════════════════════════════════════════════════════════════════════════
GROUND RULES
═══════════════════════════════════════════════════════════════════════════

1. TREAT DOCUMENTATION AS A CLAIM, NEVER AS EVIDENCE.
   A conformance report is a claim about the code. A checklist item marked
   [x] is a claim. A doc comment describing what a function does is a claim.
   Every claim gets verified against the source or it does not get believed.
   When a document says "100% conformant", that is the strongest signal in
   the repo that you should look hard at exactly that thing.

2. READ THE WHOLE PATH BEFORE JUDGING ANY PART OF IT.
   Do not audit a function in isolation. Trace it: who calls it, what they do
   with the return value, what happens to its errors. Half the serious
   findings live in the gap between two files that were each written
   correctly against a different version of the other.

3. GREP FOR ABSENCE, NOT JUST PRESENCE.
   The highest-value findings are things that do not exist:
     - Functions defined with zero callers (dead safety controls)
     - Error variants defined and never returned (unenforced policy)
     - Struct fields set and never read (fake configuration)
     - Config flags declared and never consulted
     - Dependencies declared and never imported
     - Tables created and never written to
   Run these deliberately. `bearer_token()` having zero callers *is* the
   authentication finding.

4. MARK CONFIDENCE ON EVERY FINDING.
   [C] Confirmed — legible in the source; cite file and line.
   [S] Suspected — needs a compile or a run; state your reasoning and give
       the exact command to verify.
   Never present [S] as [C]. If you cannot build the project, say so in the
   summary and mark the compile-dependent findings accordingly.

5. STATE THE BLAST RADIUS, NOT THE LINE.
   "unseal returns the wrong slice" is a line. "unseal returns the wrong
   slice, therefore every secret read fails and the enclave cannot restart"
   is the finding. Trace each defect to what actually breaks for a user.

6. WHEN A TEST ALREADY EXISTS AND THE BUG IS STILL THERE, THAT IS THE
   FINDING. It means the tests never run. Check CI before you check code:
   read every workflow and list which test commands actually execute.

═══════════════════════════════════════════════════════════════════════════
PHASE 1 — MAP THE TERRITORY (do not judge yet)
═══════════════════════════════════════════════════════════════════════════

- List every file in the repo by directory, with line counts. Note anything
  that does not belong: a second language, vendored node_modules, files
  copied from another project (mismatched domain vocabulary in type names is
  the tell).
- Read every spec document in full.
- Extract every testable claim into a numbered list. A testable claim names
  a behaviour, a standard, a constant, a data shape, or a performance bound.
  Keep the source clause reference for each one.
- Read every CI workflow. Write down exactly which test commands run. This
  usually explains everything else you are about to find.
- Inventory existing tests: what exists, what it covers, whether it runs.

Report: file map, claim list, CI reality, test inventory. No verdicts yet.

═══════════════════════════════════════════════════════════════════════════
PHASE 2 — TRACE THE CODE
═══════════════════════════════════════════════════════════════════════════

Read every source file the spec claims implements something. All of it, not
excerpts. For each, record:

  - What it actually does, in one line
  - Where it diverges from the claim it is cited as evidence for
  - Its trust boundaries: what input arrives from an untrusted source, and
    whether it is validated
  - Its failure modes: what panics, what silently returns a wrong answer,
    what returns Ok for work that did not happen

Pay specific attention to:

  * SILENT SUBSTITUTION — a catch-all match arm, a default fallback, or a
    coercion that returns something plausible instead of an error. These are
    worse than crashes because they surface in production.
  * STUBBED CONTROLS — functions whose body is a comment saying a real
    implementation would do the thing. Cross-reference against what the
    docs claim about them.
  * DEBUG-ONLY VALIDATION — assertions compiled out of release builds
    sitting on a trust boundary.
  * MOCK DATA IN PRODUCTION PATHS — hardcoded "healthy" status, fake
    telemetry, canned responses. In a security or safety product, invented
    status displayed as measured fact is a top-severity finding.
  * OFF-BY-ONE AGAINST A CITED STANDARD — when the spec quotes a cutoff or
    a bound, check the comparison operator against the standard's own text.
  * TWO ENFORCERS, ONE RULE — the same limit implemented in two places with
    different boundary conditions.

═══════════════════════════════════════════════════════════════════════════
PHASE 3 — BUILD THE CONFORMANCE MATRIX
═══════════════════════════════════════════════════════════════════════════

One row per claim from Phase 1:

  | Claim | Spec clause | Cited evidence | Reality | Verdict |

Verdicts: PASS / PARTIAL (implemented, non-conformant) / FAIL (claim
unsupported by code) / ABSENT (no implementation) / UNVERIFIED (needs
hardware, a run, or a benchmark you cannot perform).

UNVERIFIED is an honest answer. Use it rather than guessing. If a claim needs
SGX hardware, a GPU, or a live network, say so.

═══════════════════════════════════════════════════════════════════════════
PHASE 4 — WRITE THE TESTS
═══════════════════════════════════════════════════════════════════════════

Write real, runnable tests in the project's native framework. Not
pseudocode, not a test plan document — code that compiles and runs.

Rules:

- Every test gets a stable ID (TC-<AREA>-<NNN>) and a doc comment naming the
  spec clause it enforces.
- Tests for defects you found MUST FAIL against the current code. A failing
  test is the deliverable. Say so in the header comment so nobody "fixes"
  the test.
- Where the API needed to test something does not exist yet, write the test
  against the API the spec requires and mark it ignored/skipped, with the
  blocking issue ID in the skip reason. Never delete the test — the skip
  list is the outstanding-work list.
- Cover, for each area: the happy path, every boundary the spec names,
  malformed input, hostile input, and the specific defect you found.
- No mocking of the thing under test. Test the real function.
- Prefer one test that fails loudly over five that assert restatements of
  the implementation.
- If the project structure makes testing impossible (binary-only crate, no
  exported entry point, untestable global state), fix that first and say
  you did — it is usually the root cause of the whole audit.

═══════════════════════════════════════════════════════════════════════════
PHASE 5 — WRITE THE ISSUE REGISTER
═══════════════════════════════════════════════════════════════════════════

One entry per issue:

  ID · SEVERITY · CONFIDENCE · one-line title
  Location:  file:line
  What:      the defect, with the offending code quoted
  Impact:    what breaks, for whom, under what conditions
  Fix:       concrete code or a specific change — not "add validation"
  Test:      the TC- ID that demonstrates it

Severity:
  P0  Key/credential exposure, auth bypass, broken crypto guarantee,
      data loss, or the build being broken
  P1  Conformance failure against a named standard, or a remotely
      reachable fault
  P2  Correctness, robustness, or hardening gap
  P3  Hygiene, dead code, doc drift

Prefer the smallest fix that addresses the ROOT cause. If the same defect
appears in five callers, the fix goes in the shared function they all route
through, not in five places. If a fix is one word, say it is one word.

═══════════════════════════════════════════════════════════════════════════
PHASE 6 — FIX CI
═══════════════════════════════════════════════════════════════════════════

Patch the CI configuration so the tests you wrote actually run on every
push: build all targets including tests, run the suite, run the linter with
warnings denied, run a formatter check, and run a dependency vulnerability
audit. Add a step that lists skipped tests so the gap list stays visible.

═══════════════════════════════════════════════════════════════════════════
DELIVERABLES
═══════════════════════════════════════════════════════════════════════════

1. docs/TEST_PLAN_AND_CONFORMANCE.md
     - Executive summary: the three findings that matter most, stated plainly
     - Confidence key and how to verify everything
     - Blocker section if the project does not build
     - Conformance matrix
     - Issue register grouped by component
     - Test suite index: file, count, coverage, expected result today
     - Remediation order in phases, each with a completion gate

2. The test files themselves, in the project's test directory.

3. A patched CI configuration.

4. A replacement conformance summary the team can honestly publish, with
   PLANNED and NOT MET as acceptable values.

═══════════════════════════════════════════════════════════════════════════
TONE
═══════════════════════════════════════════════════════════════════════════

Report what is there. No hedging on a confirmed defect, no dramatising an
unconfirmed one. When documentation makes a claim the code contradicts, say
which document, which clause, and which line contradicts it — the point is
to make the gap fixable, not to score it.

Do not pad. If a section has no findings, one sentence saying so is the
correct length. If the explanation of a fix is longer than the fix, delete
the explanation.

Ask before you start ONLY if the deliverable format is genuinely ambiguous.
Otherwise begin with Phase 1 and report as you go.
```

---

## Notes on using this prompt

**It works best when the repo is over-documented.** Rich specification documents give the audit something to test against. A repo with no docs needs a different prompt — one that infers intent from code.

**The CI check in Phase 1 is doing more work than it looks like.** Reading the workflow files first tells you whether tests run at all. If they do not, you can predict the class of bug you will find before opening a source file: the ones a test would have caught on its first run.

**Phase 4's "tests must fail" instruction is the part agents resist.** Without it, an agent writes tests that pass against the buggy code, because passing tests feel like success. State it explicitly, and repeat it in the file header comments so the failing tests survive the next contributor.

**Scale the phases to the repo.** For something under a few thousand lines, run it as written. For a large monorepo, run Phase 1 across the whole tree, then run Phases 2–5 per component so each pass fits in context.

**Adapting to another language:**

| Rust | Python | Go | TypeScript |
|---|---|---|---|
| `cargo test` | `pytest` | `go test ./...` | `vitest` / `jest` |
| `cargo clippy -- -D warnings` | `ruff check` | `go vet` + `staticcheck` | `eslint --max-warnings 0` |
| `cargo fmt --check` | `ruff format --check` | `gofmt -l` | `prettier --check` |
| `cargo audit` | `pip-audit` | `govulncheck` | `npm audit` |
| `cargo udeps` | `deptry` | `go mod tidy -diff` | `depcheck` |
| `#[ignore = "reason"]` | `@pytest.mark.skip(reason=...)` | `t.Skip("reason")` | `it.skip(...)` |
