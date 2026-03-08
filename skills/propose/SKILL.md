---
name: propose
description: >-
  Package working changes into a PR: ensure feature branch, analyze and commit
  changes, push, create PR with derived title/body linked to issue(s).
  The middle phase of the session lifecycle between next-issue and land.
  Trigger on: 'propose', 'submit pr', 'create pr', 'open pr',
  'send for review', 'package this up'.
---

# Propose — Commit, Push, PR

**Version 1.0**

## Overview

Use this skill when implementation is complete and changes need to become a PR.

`propose` means:
1. Resolve the working context (branch, changes, linked issues)
2. Ensure a feature branch exists (guard rail if on `main`)
3. Analyze changes and produce well-formed commits
4. Push to remote
5. Create a PR with derived title/body and issue linkage
6. Report the result and suggest next steps

The session lifecycle is: `next-issue` (initiate session) → implement → `propose`
(package for review) → review → `land` (merge and close). `propose` is the
transition from execution to review.

Do not stop after creating the PR — the report step (step 6) is part of the
skill. Invoking `propose` IS the operator's approval to execute the full
sequence.

---

## Preconditions

- Uncommitted changes OR unpushed commits on a feature branch must exist. If
  the working tree is clean and no unpushed commits exist, there is nothing to
  propose — report and stop.
- If the current feature branch already has an open PR, report the PR URL and
  stop. The PR already exists; the operator likely wants `land`, not `propose`.
- `gh` CLI must be authenticated and the remote accessible.

---

## Procedure

### 1. Resolve context

Determine:
- Current branch name.
- Whether on `main` or a feature branch.
- Whether uncommitted changes exist (`git status`).
- Whether unpushed commits exist (`git log origin/main..HEAD` or
  `git log main..HEAD`).
- Issue number(s), resolved in priority order:
  1. Explicit operator-provided issue number(s).
  2. Branch name pattern: `issue-<N>/<slug>` (single) or
     `issues-<N>-<M>-.../<slug>` (multi).
  3. None — proceed without issue linkage.

If issue number(s) are available, fetch issue title(s) and body(ies) via
`gh issue view` for use in steps 3 and 5.

Check for an existing open PR on the current branch
(`gh pr list --head <branch> --state open`). If one exists, report its URL and
stop.

### 2. Ensure feature branch

If already on a feature branch: record the branch name and continue.

If on `main` (or detached HEAD):
1. Derive a branch name:
   - If issue number(s) known: `issue-<N>/<slug>` (single) or
     `issues-<N>-<M>/<slug>` (multi), where slug is the issue title —
     lowercase, hyphenated, truncated to 40 chars.
   - If no issue: `feat/<slug>`, `fix/<slug>`, or `chore/<slug>` based on
     change classification, where slug summarizes the changes.
2. Create and switch to the branch: `git checkout -b <branch>`.

### 3. Analyze changes and commit

This step is the skill's core analytical value: understanding changes well
enough to produce meaningful commit structure, not just staging everything at
once.

If all changes are already committed (only unpushed commits exist), skip to
step 4.

**3a. Understand intent.** Examine all uncommitted modifications
(`git diff`, `git diff --staged`). If issue context is available,
cross-reference changes against acceptance criteria.

**3b. Identify logical groups.** Find related changes that belong in the same
atomic commit. Apply the Complete Feature Rule: implementation, documentation,
and tests for one feature belong in one commit — never split docs from the code
they document.

**3c. Plan commits.** Design atomic commits, each serving a single purpose.
Use conventional commit format: `type(scope): description` where type is
`feat`, `fix`, `refactor`, `docs`, `test`, or `chore`. If issue number(s) are
known, reference them in the commit body (not the title): `Refs #N`. Commit
messages explain **why**, not just what.

**3d. Execute commits.** Stage and commit each logical group using
`git add <paths>`. When a single file contains changes for different commits,
stage it with the dominant change set and note the grouping compromise in the
commit message — intra-file splitting (`git add -p`) requires interactive input
and may not be available in agent environments. Verify each commit leaves the
working tree in a valid state.

If analysis produces no clear grouping (one tangled change), fall back to a
single commit with a comprehensive message. A single honest commit is better
than artificial splitting.

### 4. Push

Push the feature branch to origin:
- No upstream set: `git push -u origin <branch>`.
- Upstream exists: `git push`.

### 5. Create PR

Create a pull request via `gh pr create`.

**Title** (under 70 chars):
- Single issue: use the issue title, condensed if needed. Prefix with
  conventional commit type if not already present.
- Multi-issue: synthesize a title capturing the combined scope.
- No issue: derive from commit message(s).

**Body:**

```
## Summary

[1-3 bullet points: what this PR does and why]

## Changes

[Grouped description derived from commit messages]

## Issue(s)

Closes #N
[or "Refs #N" for partial progress]

## Test plan

[How to verify — derived from acceptance criteria if available]
```

**Flags:**
- Do NOT use `--draft` by default. The operator invoked `propose` because the
  work is ready for review. If the operator explicitly says "draft" or "propose
  as draft," use `--draft`.

### 6. Report

Output:
- PR URL.
- Branch name.
- Commit summary (count and brief subjects).
- Issue linkage (which issues referenced, close vs. ref).
- Next step: "`requesting-code-review` to get review, then `land` when
  approved."

---

## Failure Policy

- **`gh` not authenticated or remote unreachable:** Stop immediately. This is
  infrastructure the operator must fix.
- **Branch creation fails** (name collision, unresolvable dirty state): Stop
  and report. Do not force-create or silently choose an alternate name.
- **Push rejected** (remote diverged): Stop and report. Do not force-push. Do
  not rebase automatically. Commits are safely local; the operator decides how
  to reconcile.
- **Push network error:** Retry once. If still failing, report.
- **`gh pr create` fails:**
  - Branch already has an open PR: report the existing PR URL (not an error).
  - Permissions error: stop and report.
  - Other: report. The branch is pushed; the operator can retry or create
    manually.
- **Issue fetch fails:** Continue without issue context. Derive PR title/body
  from commits alone. Warn that issue linkage is manual.

---

## Corruption Modes

- `premature-propose`: invoking before verification. Recognition: tests not
  run, WIP markers in code (TODO, FIXME, incomplete stubs). `propose` is not a
  verification gate, but it should warn on obvious signs of incomplete work.
- `empty-propose`: nothing to propose. Recognition: clean tree, no unpushed
  commits. Report and stop.
- `split-avoidance`: dumping all changes into one commit to skip analysis.
  Recognition: single commit touching many unrelated files with a vague
  message. The analysis phase exists to prevent this.
- `orphan-pr`: no issue linkage when issues are clearly relevant. Recognition:
  branch name contains an issue number but the PR body omits it. Detect issue
  numbers from branch names automatically.
- `title-shrug`: uninformative PR title ("Update files", "Changes"). The title
  is the first thing reviewers see — it must communicate the change's purpose.
- `propose-as-land`: treating the PR as the end of the workflow. `propose` is
  the middle of the lifecycle. The report step suggests next actions explicitly.

---

## Related Skills

- `next-issue` for work initiation — select issue(s), prepare workspace, declare direction (the preceding phase)
- `land` for merge, cleanup, and issue closure (the following phase)
- `requesting-code-review` for dispatching review after the PR exists
- `verification-before-completion` — should fire before `propose`
- `documentation` for documentation review before proposing
