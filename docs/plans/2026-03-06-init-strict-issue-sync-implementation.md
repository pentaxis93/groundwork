# Strict Init Issue Sync Gate Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Make `groundwork init` fail unless `gh-issue-sync` has completed a successful full pull (`Last full pull` is not `never`).

**Architecture:** Keep existing bootstrap flow but add an init-only verification gate that checks post-pull mirror status. Reuse existing status parsing and remediation classification. Return errors in init mode so success is tied to operational sync.

**Tech Stack:** Rust (`anyhow`, std `process::Command`), existing CLI test module in `crates/groundwork-cli/src/main.rs`

---

### Task 1: Add failing tests for strict init verification gate

**Files:**
- Modify: `crates/groundwork-cli/src/main.rs`
- Test: `crates/groundwork-cli/src/main.rs` (unit tests module)

**Step 1: Write the failing test**
Add tests for new helper behavior:
- init gate passes for status with timestamp
- init gate fails for status with `Last full pull: never`
- init gate fails for status command failure/empty parse

**Step 2: Run test to verify it fails**
Run: `cargo test -p groundwork-cli strict_init_issue_sync`
Expected: FAIL due to missing helper/behavior.

**Step 3: Write minimal implementation**
Add helper(s) that evaluate post-pull status and return `Result<()>` for init mode.

**Step 4: Run test to verify it passes**
Run: `cargo test -p groundwork-cli strict_init_issue_sync`
Expected: PASS.

**Step 5: Commit**
Run:
```bash
git add crates/groundwork-cli/src/main.rs
git commit -m "test(cli): add strict init issue-sync gate coverage"
```

### Task 2: Wire strict gate into init flow

**Files:**
- Modify: `crates/groundwork-cli/src/main.rs`

**Step 1: Write the failing test**
Add a focused test for init bootstrap decision path that should error if post-pull status remains `never`.

**Step 2: Run test to verify it fails**
Run: `cargo test -p groundwork-cli init_requires_non_never_pull`
Expected: FAIL on current behavior.

**Step 3: Write minimal implementation**
- Change bootstrap function to accept `is_init`.
- In init mode, convert current warn-only failures into `Err(...)` with remediation text.
- Ensure call site propagates error in `run_install_in_directory`.

**Step 4: Run test to verify it passes**
Run: `cargo test -p groundwork-cli init_requires_non_never_pull`
Expected: PASS.

**Step 5: Commit**
Run:
```bash
git add crates/groundwork-cli/src/main.rs
git commit -m "feat(cli): require operational issue sync in init"
```

### Task 3: Verify no regressions and document behavior

**Files:**
- Modify: `README.md`
- Modify: `WORKFLOW.md`
- Modify: `crates/groundwork-cli/src/main.rs` (if message text adjustments needed)

**Step 1: Write the failing doc expectation**
Add/update docs describing that `init` fails until sync is operational and include remediation commands.

**Step 2: Run verification commands**
Run:
- `cargo test -p groundwork-cli`
- `cargo run -p groundwork-cli -- doctor`

Expected:
- tests pass
- doctor still reports diagnostics correctly

**Step 3: Commit**
Run:
```bash
git add README.md WORKFLOW.md crates/groundwork-cli/src/main.rs
git commit -m "docs: define init operational issue-sync guarantee"
```
