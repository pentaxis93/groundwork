---
name: land
description: >-
  One-word closeout: land this branch. Closing ceremony verifies completion and
  documentation before mechanical merge. Merge to main, sync, delete feature
  branch, close satisfied issues, comment progress on partial issues.
  Trigger on: 'land', 'land this', 'merge and close', 'ship it'.
requires: ["patch"]
accepts: ["completion-evidence", "behavior-contract", "documentation-record", "work-unit"]
produces: ["completion-record"]
may_produce: []
trigger:
  on_artifact: "patch"
---

# Land — Close, Verify, Merge

## Overview

Use this skill when the user wants full delivery closure in one command.

`land` operates in two phases:

- **Phase 0: Closing** — a ceremony that prepares the work for delivery. Four
  steps mirror `take`'s opening ceremony in reverse: gather context, verify
  acceptance, review documentation, seal the gate.
- **Phase 1: Mechanical** — the merge-and-close sequence, gated by Phase 0.

Phase 0 (gather, verify, review, seal) is to `land` what Phase 0 (orient,
observe, frame, banish) is to `take`. The opening ceremony prepares the agent
for work; the closing ceremony prepares the work for delivery.

Do not stop after merge — stopping leaves branches dangling and issues unclosed. The full sequence is atomic: ceremony through close.

Do not ask for confirmation before landing. Invoking `land` IS the user's approval to execute the entire workflow.

---

## Preconditions

- Working tree must be clean before starting.
- Current branch must not be `main`.
- GitHub issue numbers are optional:
  - Prefer explicit user-provided GitHub issue number(s).
  - Else infer from branch name using one of:
    - `issue-<number>/<slug>` (single GitHub issue)
    - `issues-<number>-<number>-.../<slug>` (multi-GitHub-issue, unbounded)
  - If no GitHub issue numbers are available, continue with the no-GitHub-issue closeout path.

---

## Procedure

### Phase 0: Closing

The closing ceremony equips the landing with everything the mechanical phase
needs: context about what was built and why, verification that acceptance
criteria are met, confirmation that documentation reflects reality, and a gate
that blocks merge until readiness is confirmed. Each step builds on the
previous — gather loads context, verify checks acceptance, review checks
documentation, seal confirms the gate.

Follows the closing sequence: gather → verify → review → seal.

#### 0a. Gather

Collect the full context of the work being landed.

```
◈ GATHER
Branch: [name] | Issues: [numbers or none]
Diff: [file count] files, [insertions]+/[deletions]-
Commits: [count] ([oneline summaries])
CHANGELOG: [present | missing — needs entry | not needed]
```

Implementation:
- Confirm the current branch is not `main` and the working tree is clean. Record the feature branch name.
- Extract GitHub issue numbers from the branch name:
  - `issue-42/fix-login` → GitHub issue 42
  - `issues-5-8/license-cleanup` → issues 5 and 8
  - If neither the user nor the branch name provides issue numbers, mark the landing as no-GitHub-issue.
- Summarize the branch diff: `git diff origin/main...HEAD --stat`.
- Summarize the commit log: `git log origin/main..HEAD --oneline`.
- Check whether `CHANGELOG.md` appears in the branch diff (`git diff origin/main...HEAD --name-only`). The changelog is how users discover what changed between versions. Evaluate whether the branch changes are significant enough to warrant an entry. New features, behavior changes, bug fixes, and breaking changes generally belong in the changelog. Internal refactoring, typo fixes, and CI tweaks generally don't. If the changes look user-visible but no entry exists, flag it to the user before proceeding.

#### 0b. Verify

Evaluate whether the branch changes satisfy acceptance criteria and all
verification checks pass. This step runs pre-merge while the branch diff is
available.

```
◈ VERIFY
Criteria: [all met | partial — list remaining]
```

Implementation: invoke the `verify` skill. For
GitHub-issue-linked branches, fetch each target GitHub issue (`gh issue view`) and evaluate
acceptance criteria against the branch diff. Classify each GitHub issue as satisfied
(all criteria met) or partial (some remain). If no acceptance criteria can be
extracted from the GitHub issue body, classify as partial — a GitHub issue without explicit
criteria may have unstated requirements. If a GitHub issue fetch fails, treat that
GitHub issue as partial and log a warning. For no-GitHub-issue landings, skip acceptance
criteria evaluation — `verify` still runs to confirm
the work itself is complete. Store classifications for Phase 1e.

#### 0c. Review

Confirm documentation reflects the changes being landed. This is mandatory —
documentation drift blocks the seal.

```
◈ REVIEW
Docs: [clean | fixed: list | tracked: GitHub issue numbers]
```

Implementation: invoke the `document` protocol's documentation-review
procedure via the Skill tool. Check whether changed files affect areas with documentation artifacts
(README, ARCHITECTURE, API docs). Fix drift directly and commit;
file tracking issues for anything deeper. Record a coverage summary: each
artifact checked, its status, and any action taken.

#### 0d. Seal

Gate check before mechanical merge. All three conditions must hold:

1. Verification passed or all issues classified (Phase 0b complete)
2. Documentation reviewed — drift fixed or tracked (Phase 0c complete)
3. CHANGELOG entry present if changes are user-visible (Phase 0a flagged)

```
◈ SEAL
[PASSED — proceed to Phase 1 | BLOCKED — list failures]
```

If the seal fails, fix the blocking GitHub issue(s) and re-evaluate. Do not proceed to
Phase 1 until the seal passes.

### Phase 1: Mechanical

Gated by Phase 0. Do not enter this phase until the seal passes.

#### 1a. Evaluate commit history for squash

Examine the branch's commit history to decide whether to squash on merge. Use `git log origin/main..HEAD` variants to inspect commit subjects and touched files.

**Decision framework** (apply judgment, not mechanical rules):

- **Squash when** the history is iterative refinement — a feature commit followed by fix-ups that revise the same change: majority of commits are fixes of the initial change, commits touch the same files, all share the same scope/component.
- **Preserve when** commits represent distinct work units — single-commit branches, different components/scopes, multi-step features where each step is meaningful independently.

**When squashing**, draft a consolidated commit message: use the conventional-commit prefix/scope from the initial commit, summarize the consolidated change, don't enumerate squashed commits.

Record the squash decision. If squashing, also record the drafted commit message.

#### 1b. Discover PR number

Look up the open PR for the feature branch (`gh pr list --head <branch> --state open`). This must happen before merge because `gh pr merge` requires the PR number. If no PR is found, fall back to local merge (step 1c fallback).

#### 1c. Merge via PR API

Merge through GitHub's PR API so the PR is recorded as "merged," not just "closed." This is the primary merge path.

1. Run `gh pr merge <number>` with the appropriate strategy flag: `--squash` if squashing (with `--subject` and `--body` for the drafted commit message), `--merge` if preserving history.
2. Include `--delete-branch` to let GitHub clean up the remote branch.
3. After the API merge completes, sync local state: `git checkout main`, `git pull --ff-only origin main`.
4. Record the merge commit SHA from the local main HEAD for use in GitHub issue comments.

**Why not local merge:** A local `git merge` + `git push` lands the code on main but bypasses GitHub's PR merge tracking. GitHub sees the PR's commits already on main and auto-closes the PR as "closed" rather than "merged" when the branch is deleted. This loses PR merge metadata and breaks PR history.

##### 1c fallback: local merge (no PR exists)

This path is only for branches that never had a PR — not a fallback for when `gh pr merge` fails. If the API merge fails, the failure policy applies (stop, don't retry locally).

If no PR was found in step 1b, merge locally:

1. Fetch and fast-forward `main` to match origin (`git fetch origin --prune`, `git checkout main`, `git pull --ff-only origin main`).
2. Merge the feature branch. If squashing: `git merge --squash`, then `git commit` with the drafted message. If preserving: `git merge --no-ff`.
3. Push `main` to origin.
4. Record the merge commit SHA.

#### 1d. Delete feature branch

Delete any remaining branch references:
- Remote: skip if `--delete-branch` was used in step 1c. Otherwise, `git push origin --delete <branch>` (tolerate failure if already gone).
- Local: `git branch -D <branch>`. The `-D` flag (force delete) is required because squash merges don't record merge parentage, so `-d` refuses even though the content is safely on `main`.
- Prune stale remote-tracking references: `git fetch origin --prune`.

#### 1e. Comment and close GitHub issue(s) (GitHub-issue-linked branches only)

If no issues were provided or inferred, skip this step.

Apply the classifications from Phase 0b.

**For satisfied GitHub issues**, post a close comment and close the GitHub issue:

> Implemented and merged in PR #`<number>` (commit `<sha>`). Closing as complete.
>
> *(If no PR was found, omit the PR reference.)*

Then close: `gh issue close <number> --reason completed`.

**For partial GitHub issues**, post a progress comment but leave the GitHub issue open:

> Progress from PR #`<number>` (commit `<sha>`):
>
> **Delivered:**
> - criterion 1
> - criterion 2
>
> **Remaining:**
> - criterion 3

If any comment or close operation fails, continue processing remaining issues, then report which operations failed.

#### 1f. Verify and report

Confirm success conditions:
- Current branch is `main`
- Working tree is clean
- Feature branch absent on origin
- PR state is `MERGED` (not just `CLOSED`)
- For GitHub-issue-linked landings: every satisfied GitHub issue state is `CLOSED`
- For GitHub-issue-linked landings: every partial GitHub issue has a progress comment listing remaining criteria
- Documentation coverage summary reported

Report the final state including:
- GitHub issue disposition:
  - GitHub-issue-linked: satisfied (closed) and partial (open with remaining criteria)
  - No-GitHub-issue: explicitly report "no GitHub issue linked"
- Documentation coverage summary from Phase 0c
- Any warnings or failed operations from earlier steps

---

## Failure Policy

- **Seal failure blocks Phase 1.** If the seal (Phase 0d) fails, fix the blocking GitHub issue(s) and re-enter the ceremony from the failed step. Do not proceed to mechanical merge until the seal passes.
- If `gh pr merge` fails: stop immediately, do not close the GitHub issue. If the failure is transient (network), retry once. If structural (merge conflict, check failure), report and stop. Do not fall back to local merge — the whole point of using the API is to preserve PR merge metadata.
- If branch deletion fails after successful merge: warn about the deletion failure and continue to GitHub issue close/comment steps. The code is safely on `main`; branch cleanup is not a prerequisite for issue closure.
- If GitHub issue comment/close API fails for one GitHub issue: continue processing remaining GitHub issues, then report failed GitHub issue number(s) explicitly.
- If acceptance criteria evaluation fails in Phase 0b (GitHub issue fetch error, criteria unparseable): the inline handling applies — treat the GitHub issue as partial, log a warning. Partial classification does not block the seal; it flows through to Phase 1e where the GitHub issue is left open with a progress comment.
- **Documentation drift blocks the seal.** Drift discovered in Phase 0c must be fixed directly or tracked via a GitHub issue before the seal can pass. Do not proceed with unresolved drift.
- If no GitHub issue numbers are available: do not prompt for GitHub issue IDs during `land`; proceed with merge/sync/cleanup and report a no-GitHub-issue landing.
- If commit history evaluation is uncertain: default to preserve (`--no-ff`). Squashing is an optimization; when in doubt, keep the original history.

---

## Cross-References

- `take`: the opening bookend — opening ceremony and session initiation before
  implementation. `take`'s opening ceremony (orient, observe, frame, banish)
  prepares the agent for work; `land`'s closing ceremony (gather, verify, review,
  seal) prepares the work for delivery. Parallel structure, inverse direction.
- `submit` for the preceding phase: commit, push, and PR creation
- `verify`: invoked during Phase 0b to evaluate
  acceptance criteria and verify completion evidence before merge
- `document`: invoked during Phase 0c for documentation-review — confirms
  documentation reflects the changes being landed
- `decompose` for GitHub issue lifecycle patterns and tracking issues from doc review
