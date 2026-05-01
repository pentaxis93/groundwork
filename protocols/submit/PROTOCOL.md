---
name: submit
description: >-
  Package working changes into a PR: ensure feature branch, analyze and commit
  changes, push, then create or update a PR linked to GitHub issue(s).
  The middle phase of the session lifecycle between take and land.
  Trigger on: 'submit', 'submit pr', 'create pr', 'open pr',
  'send for review', 'package this up'.
---

# Submit — Commit, Push, PR

## Overview

Protocol for packaging verified, documented changes into a PR and delivering
the `patch` artifact. Fires when implementation is complete and reviewed
changes need to cross into the review stage.

`submit` means:
1. Resolve the working context (branch, changes, linked issues)
2. Ensure a feature branch exists (guard rail if on `main`)
3. Analyze changes and produce well-formed commits
4. Resolve whether delivery updates an existing PR or opens a new PR
5. Push to remote
6. Create a PR when needed
7. Deliver the `patch` artifact via the MCP tool
8. Report the result and suggest next steps

The session lifecycle is: `take` (initiate session) → implement → `submit`
(package for review) → review → `land` (merge and close). `submit` is the
transition from execution to review.

Do not stop after creating or updating the PR — the delivery and report steps
(7 and 8) are part of the protocol. Invoking `submit` IS the operator's
approval to execute the full sequence.

---

## Preconditions

- Deliverable local work means uncommitted changes OR committed work identified
  from the branch's tracking and PR state. If the working tree is clean and no
  deliverable commits exist, there is nothing new to submit. When an open PR
  exists, report its current state and recommend `land` if appropriate; when no
  open PR exists, report and stop.
- Producing the `patch` capstone requires a real PR reference, which in turn
  requires forge tooling (`gh` authenticated against the remote). Where forge
  tooling is absent, the protocol can commit and push locally but cannot
  deliver the `patch` artifact — see step 6 for the failure path.

---

## Procedure

### 1. Resolve context

Determine:
- Current branch name.
- Whether on `main` or a feature branch.
- Whether uncommitted changes exist (`git status`).
- Whether an open PR exists for the current branch (`gh pr list --head <branch>
  --state open`) when `gh` is available. For an open PR, record the PR head SHA
  (`headRefOid`).
- Whether committed local work is deliverable:
  - **With upstream:** commits ahead of the branch's remote
    tracking ref (`git log @{upstream}..HEAD`).
  - **No upstream and no open PR:** local commits on the feature branch are
    deliverable under first-push semantics.
  - **No upstream and open PR:** compare local `HEAD` (`git rev-parse HEAD`)
    with the PR head SHA (`headRefOid`). If they match, no committed local work
    is deliverable and the existing PR state should be reported. If they differ,
    the divergent local commits are deliverable through the existing PR update
    path.
- GitHub issue number(s), resolved in priority order:
  1. Explicit operator-provided GitHub issue number(s).
  2. Branch name pattern: `issue-<N>/<slug>` (single) or
     `issues-<N>-<M>-.../<slug>` (multi).
  3. None — proceed without GitHub issue linkage.

If GitHub issue number(s) are available and `gh` is installed, fetch issue
title(s) and body(ies) via `gh issue view` for use in steps 3 and 6.

### 2. Ensure feature branch

If already on a feature branch: record the branch name and continue.

If on `main` (or detached HEAD):
1. Derive a branch name:
   - If GitHub issue number(s) known: `issue-<N>/<slug>` (single) or
     `issues-<N>-<M>/<slug>` (multi), where slug is the GitHub issue title —
     lowercase, hyphenated, truncated to 40 chars.
   - If no linked GitHub issue: `feat/<slug>`, `fix/<slug>`, or `chore/<slug>` based on
     change classification, where slug summarizes the changes.
2. Create and switch to the branch: `git checkout -b <branch>`.

### 3. Analyze changes and commit

This step is the protocol's core analytical value: understanding changes
well enough to produce meaningful commit structure, not just staging
everything at once.

This shared step applies before either PR delivery path. The same commit
analysis discipline applies whether the deliverable is a new PR or an existing
PR update.

If all deliverable work is already committed, skip to step 4.

**3a. Understand intent.** Examine all uncommitted modifications
(`git diff`, `git diff --staged`). If GitHub issue context is available,
cross-reference changes against acceptance criteria.

**3b. Identify logical groups.** Find related changes that belong in the same
atomic commit. Apply the Complete Feature Rule: implementation, documentation,
and tests for one feature belong in one commit — never split docs from the code
they document.

**3c. Plan commits.** Design atomic commits, each serving a single purpose.
Use conventional commit format: `type(scope): description` where type is
`feat`, `fix`, `refactor`, `docs`, `test`, or `chore`. If GitHub issue number(s) are
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

### 4. Resolve PR delivery path

Use the context from step 1 after commit analysis has completed:

- **open PR exists and deliverable local work exists:** push to the existing PR
  branch. This is the normal review-fix path after a PR already exists.
- **open PR exists and no deliverable local work exists:** report the PR URL
  and current state, recommend `land` when review status makes that
  appropriate, and stop. There is nothing new for `submit` to deliver.
- **no open PR exists:** deliver by opening a new PR after pushing the branch.

### 5. Push

Push the feature branch to origin:
- No upstream set: `git push -u origin <branch>`.
- Upstream exists: `git push`.

For an existing PR update, this push is the delivery action: it updates the
remote branch backing the open PR. Report this path as `pushed to existing PR`.

### 6. Create or identify PR

The `patch` capstone requires a real PR reference. Where `gh` is available
and authenticated against the remote:
- Existing PR update path: reuse the open PR reference found in step 1.
- New PR path: create the PR via `gh pr create`.

Where forge tooling is absent, the protocol cannot complete — report what was
committed and pushed locally, note that no `patch` artifact was delivered, and
stop. Do not synthesize a PR reference.

**Title** (under 70 chars):
- Single GitHub issue: use the issue title, condensed if needed. Prefix with
  conventional commit type if not already present.
- Multi-GitHub-issue: synthesize a title capturing the combined scope.
- No linked GitHub issue: derive from commit message(s).

**Body:**

```
## Summary

[1-3 bullet points: what this PR does and why]

## Changes

[Grouped description derived from commit messages]

## GitHub Issue(s)

Closes #N
[or "Refs #N" for partial progress]

## Test plan

[How to verify — derived from acceptance criteria if available]
```

**Shell-safe transport (required):**
- Build multiline Markdown body content in a file and pass it with
  `--body-file <path>`.
- Acceptable file creation patterns:
  - direct write to a temporary file
  - single-quoted heredoc redirected to a file (no interpolation)
- Never pass multiline Markdown bodies inline with double quotes (for example:
  ``gh pr create --body "...\`cmd\`..."``) because shells can evaluate
  backticks as command substitution.

Safe examples:

```bash
# Create
cat > /tmp/pr-body.md <<'EOF'
## Summary
- ...
EOF
gh pr create --title "fix(propose): shell-safe PR body handling" --body-file /tmp/pr-body.md

# Edit
cat > /tmp/pr-body.md <<'EOF'
## Summary
- updated after review
EOF
gh pr edit <pr-number-or-url> --body-file /tmp/pr-body.md
```

**Flags:**
- Do NOT use `--draft` by default. The operator invoked `submit` because the
  work is ready for review. If the operator explicitly says "draft" or "submit
  as draft," use `--draft`.

### 7. Deliver `patch`

The capstone is delivery of the `patch` artifact via the `patch` MCP tool.
`patch` is the complete latest PR state snapshot after submission, with the
same shape for both a newly opened PR and an updated existing PR:

```
patch({
  instance_id: "<slug>",
  pr_reference: "<PR URL from step 6>",
  branch: "<feature branch name>",
  commit: "<head commit SHA at submission>"
})
```

Runa injects `work_unit` from session context, validates the payload
against the patch schema, persists the artifact, and records it in the
artifact store. Do not emit a partial update artifact that assumes the operator
already knows the surrounding PR context.

### 8. Report

Output:
- PR URL.
- Branch name.
- Commit summary (count and brief subjects).
- GitHub issue linkage (which GitHub issues are referenced, close vs. ref).
- `patch` artifact instance_id.
- Delivery action: "opened new PR" or "pushed to existing PR."
- Next step: "Get review, then `land` when approved."

---

## Failure Policy

- **Forge tooling (`gh`) unavailable or unauthenticated:** The protocol can
  still commit and push (steps 1–5) but cannot create or identify a PR and
  therefore cannot deliver the `patch` capstone. Report what was committed and
  pushed, note the missing forge action, and stop. Do not synthesize a `patch`
  artifact without a real `pr_reference`. Remote-side failures are covered by
  the push bullets below.
- **Branch creation fails** (name collision, unresolvable dirty state): Stop
  and report. Do not force-create or silently choose an alternate name.
- **Push rejected** (remote diverged): Stop and report. Do not force-push. Do
  not rebase automatically. Commits are safely local; the operator decides how
  to reconcile.
- **Push network error:** Retry once. If still failing, report.
- **`gh pr create` fails:**
  - Branch already has an open PR: treat as the existing PR update path if
    deliverable local work was pushed; otherwise report the existing PR URL and
    stop.
  - Permissions error: stop and report.
  - Other: report and stop. The branch is pushed; the operator resolves
    the cause and directs runa to re-activate `submit`. Manual forge
    intervention is outside the protocol surface.
- **PR body corruption from shell interpolation:** If generated content appears
  corrupted or command output is injected, rebuild the body in a file and
  repair using `gh pr edit --body-file <path>`. Do not retry with inline
  double-quoted multiline `--body`.
- **GitHub issue fetch fails:** Continue without GitHub issue context. Derive PR title/body
  from commits alone. Warn that GitHub issue linkage is manual.

---

## Corruption Modes

- `premature-submit`: invoking before verification. Recognition: tests not
  run, WIP markers in code (TODO, FIXME, incomplete stubs). `submit` is not a
  verification gate, but it should warn on obvious signs of incomplete work.
- `empty-submit`: nothing to submit. Recognition: clean tree, no unpushed
  commits. Report and stop.
- `split-avoidance`: dumping all changes into one commit to skip analysis.
  Recognition: single commit touching many unrelated files with a vague
  message. The analysis phase exists to prevent this.
- `orphan-pr`: no GitHub issue linkage when GitHub issues are clearly relevant. Recognition:
  branch name contains a GitHub issue number but the PR body omits it. Detect GitHub issue
  numbers from branch names automatically.
- `title-shrug`: uninformative PR title ("Update files", "Changes"). The title
  is the first thing reviewers see — it must communicate the change's purpose.
- `submit-as-land`: treating the PR as the end of the workflow. `submit` is
  the middle of the lifecycle. The report step suggests next actions explicitly.
- `shell-interpolation-corruption`: passing Markdown via inline double-quoted
  `--body` causes backtick command substitution or other shell interpolation.
  Recognition: unexpected command output in PR text, shell errors from tokens in
  the body, or missing literal Markdown. Remediation: regenerate body via file
  and use `--body-file`; repair with `gh pr edit --body-file`.

---

## Cross-References

- `take`: the opening bookend of the session lifecycle — consumes the
  selected work-unit, prepares the workspace, and produces the `claim`.
- `land`: the closing bookend — closing ceremony, merge, and work-unit
  closure after review.
- `verify`: runs before `submit`; its `completion-evidence` output is
  a required input (per `manifest.toml`).
- `document`: runs between `verify` and `submit`; its
  `documentation-record` output is a required input.
