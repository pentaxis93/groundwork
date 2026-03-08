---
name: land
description: >-
  One-word closeout: land this branch. Merge to main, sync, delete feature
  branch, verify acceptance criteria, close satisfied issues, comment progress
  on partial issues. Use after review is complete.
  Trigger on: 'land', 'land this', 'merge and close', 'ship it'.
---

# Land — Merge, Sync, Cleanup, Close

**Version 1.9**

## Overview

Use this skill when the user wants full delivery closure in one command.

`land` means:
1. Check whether changes warrant a CHANGELOG entry
2. Scan for documentation drift; fix or flag
3. Evaluate acceptance criteria against the branch diff
4. Evaluate commit history; squash when iterative refinement adds noise
5. Discover PR number
6. Merge via PR API and sync local state
7. Remove the feature branch (local; remote usually handled by merge)
8. Sync installed skill copies if the branch touched skill sources
9. Close satisfied issue(s); comment progress on partial issue(s)
10. Verify final state (including documentation coverage summary)

Do not stop after merge — stopping leaves branches dangling and issues unclosed. The full sequence is atomic: merge through close.

Do not ask for confirmation before landing. Invoking `land` IS the user's approval to execute the entire workflow.

---

## Preconditions

- Working tree must be clean before starting.
- Current branch must not be `main`.
- Issue numbers are optional:
  - Prefer explicit user-provided issue number(s).
  - Else infer from branch name using one of:
    - `issue-<number>/<slug>` (single issue)
    - `issues-<number>-<number>-.../<slug>` (multi-issue, unbounded)
  - If no issue numbers are available, continue with the no-issue closeout path.

---

## Procedure

### 1. Resolve context

Confirm the current branch is not `main` and the working tree is clean. Record the feature branch name.

Extract issue numbers from the branch name:
- `issue-42/fix-login` → issue 42
- `issues-5-8/license-cleanup` → issues 5 and 8

If neither the user nor the branch name provides issue numbers, mark the landing as no-issue and continue.

### 2. Check whether changes warrant a CHANGELOG entry

Check whether `CHANGELOG.md` appears in the branch diff (`git diff origin/main...HEAD --name-only`).

The changelog is how users discover what changed between versions — without it, behavior changes silently. Evaluate whether the branch changes are significant enough to warrant an entry. New features, behavior changes, bug fixes, and breaking changes generally belong in the changelog. Internal refactoring, typo fixes, and CI tweaks generally don't. If the changes look user-visible but no entry exists, flag it to the user before proceeding.

### 3. Documentation drift scan

Quick check for obvious documentation impacts — not a full review. Look at the changed files and check whether they affect areas with documentation artifacts (README, ARCHITECTURE, CONTRIBUTING, API docs). Fix simple drift directly and commit; file tracking issues for anything deeper.

Record a coverage summary: each artifact checked, its status, and any action taken.

This step is best-effort — if the scan encounters errors, note them in the summary and proceed to merge.

### 4. Evaluate acceptance criteria (issue-linked branches only)

Skip this step when no issue numbers are available.

Fetch each target issue (`gh issue view`) and evaluate whether the branch changes satisfy its acceptance criteria. This step runs pre-merge while the branch diff is available.

1. **Extract acceptance criteria** from the issue body — checklist items (`- [ ]`), an acceptance criteria section, or explicitly stated requirements.
2. **Evaluate each criterion** against the branch diff (`git diff origin/main...HEAD`). A criterion is met when the changes demonstrably satisfy it.
3. **Classify:**
   - **Satisfied** — all extracted criteria met.
   - **Partial** — some criteria met but others remain.

   If no acceptance criteria can be extracted from the issue body, classify it as partial. An issue without explicit criteria may have unstated requirements — closing it without verification risks premature closure. Leave it open for human review.

For each partial issue, store its satisfied and remaining criteria lists separately, keyed by issue number. Every issue must appear in exactly one classification.

If an issue fetch fails, treat that issue as partial and log a warning.

### 5. Evaluate commit history for squash

Examine the branch's commit history to decide whether to squash on merge. Use `git log origin/main..HEAD` variants to inspect commit subjects and touched files.

**Decision framework** (apply judgment, not mechanical rules):

- **Squash when** the history is iterative refinement — a feature commit followed by fix-ups that revise the same change: majority of commits are fixes of the initial change, commits touch the same files, all share the same scope/component.
- **Preserve when** commits represent distinct work units — single-commit branches, different components/scopes, multi-step features where each step is meaningful independently.

**When squashing**, draft a consolidated commit message: use the conventional-commit prefix/scope from the initial commit, summarize the consolidated change, don't enumerate squashed commits.

Record the squash decision. If squashing, also record the drafted commit message.

### 6. Discover PR number

Look up the open PR for the feature branch (`gh pr list --head <branch> --state open`). This must happen before merge because `gh pr merge` requires the PR number. If no PR is found, fall back to local merge (step 7a).

### 7. Merge via PR API

Merge through GitHub's PR API so the PR is recorded as "merged," not just "closed." This is the primary merge path.

1. Run `gh pr merge <number>` with the appropriate strategy flag: `--squash` if squashing (with `--subject` and `--body` for the drafted commit message), `--merge` if preserving history.
2. Include `--delete-branch` to let GitHub clean up the remote branch.
3. After the API merge completes, sync local state: `git checkout main`, `git pull --ff-only origin main`.
4. Record the merge commit SHA from the local main HEAD for use in issue comments.

**Why not local merge:** A local `git merge` + `git push` lands the code on main but bypasses GitHub's PR merge tracking. GitHub sees the PR's commits already on main and auto-closes the PR as "closed" rather than "merged" when the branch is deleted. This loses PR merge metadata and breaks PR history.

#### 7a. Fallback: local merge (no PR exists)

This path is only for branches that never had a PR — not a fallback for when `gh pr merge` fails. If the API merge fails, the failure policy applies (stop, don't retry locally).

If no PR was found in step 6, merge locally:

1. Fetch and fast-forward `main` to match origin (`git fetch origin --prune`, `git checkout main`, `git pull --ff-only origin main`).
2. Merge the feature branch. If squashing: `git merge --squash`, then `git commit` with the drafted message. If preserving: `git merge --no-ff`.
3. Push `main` to origin.
4. Record the merge commit SHA.

### 8. Delete feature branch

Delete any remaining branch references:
- Remote: skip if `--delete-branch` was used in step 7. Otherwise, `git push origin --delete <branch>` (tolerate failure if already gone).
- Local: `git branch -D <branch>`. The `-D` flag (force delete) is required because squash merges don't record merge parentage, so `-d` refuses even though the content is safely on `main`.
- Prune stale remote-tracking references: `git fetch origin --prune`.

### 9. Comment and close issue(s) (issue-linked branches only)

If no issues were provided or inferred, skip this step.

Apply the classifications from step 4.

**For satisfied issues**, post a close comment and close the issue:

> Implemented and merged in PR #`<number>` (commit `<sha>`). Closing as complete.
>
> *(If no PR was found, omit the PR reference.)*

Then close: `gh issue close <number> --reason completed`.

**For partial issues**, post a progress comment but leave the issue open:

> Progress from PR #`<number>` (commit `<sha>`):
>
> **Delivered:**
> - criterion 1
> - criterion 2
>
> **Remaining:**
> - criterion 3

If any comment or close operation fails, continue processing remaining issues, then report which operations failed.

### 9a. Sync issue state to local mirror

If no issues were provided or inferred, this step is optional and may be skipped.

If `gh-issue-sync` is available on `PATH`, run `gh-issue-sync pull` to update the local issue mirror. If the tool is not installed or the sync fails, skip gracefully — a sync failure after successful merge is not catastrophic. Remote state is already correct; only the local mirror is stale.

### 10. Sync installed skill copies

Check if the merge introduced changes under `skills/` by inspecting the diff of the merge commit (`git diff --name-only HEAD~1 HEAD | grep '^skills/'`).

**If no skill files changed:** skip, record "no skill changes."

**If skill files changed:**

1. Check if `sk` is on PATH (`command -v sk`).
2. If not found: skip with a warning ("sk not on PATH; installed skill copies may be stale"), record "skipped (sk not found)."
3. If found: run `sk sync --skill-target name --non-interactive`.
   - On success: record "synced."
   - On failure: warn but do not roll back the merge. Record "sync failed" with the error.

### 11. Verify and report

Confirm success conditions:
- Current branch is `main`
- Working tree is clean
- Feature branch absent on origin
- PR state is `MERGED` (not just `CLOSED`)
- For issue-linked landings: every satisfied issue state is `CLOSED`
- For issue-linked landings: every partial issue has a progress comment listing remaining criteria
- Documentation coverage summary reported

Report the final state including:
- Issue disposition:
  - Issue-linked: satisfied (closed) and partial (open with remaining criteria)
  - No-issue: explicitly report "no issue linked"
- Documentation coverage summary from step 3
- Skill sync outcome: one of `synced`, `skipped (no skill changes)`, `skipped (sk not found)`, `sync failed`
- Any warnings or failed operations from earlier steps

---

## Failure Policy

- If `gh pr merge` fails: stop immediately, do not close issue. If the failure is transient (network), retry once. If structural (merge conflict, check failure), report and stop. Do not fall back to local merge — the whole point of using the API is to preserve PR merge metadata.
- If branch deletion fails after successful merge: warn about the deletion failure and continue to issue close/comment steps. The code is safely on `main`; branch cleanup is not a prerequisite for issue closure.
- If issue comment/close API fails for one issue: continue processing remaining issues, then report failed issue number(s) explicitly.
- If acceptance criteria evaluation fails (issue fetch error, criteria unparseable): treat the issue as partial, log a warning, and do not close it. The operator must resolve manually.
- If no issue numbers are available: do not prompt for issue IDs during `land`; proceed with merge/sync/cleanup and report a no-issue landing.
- If documentation drift scan encounters errors: report them in the coverage summary and proceed. Do not block the merge.
- If commit history evaluation is uncertain: default to preserve (`--no-ff`). Squashing is an optimization; when in doubt, keep the original history.
- If `sk sync` fails: report the failure but do not roll back — the code is safely on main, only installed copies are stale.
- If `sk` is not on PATH: skip with a warning, not a failure.

---

## Related Skills

- `propose` for the preceding phase: commit, push, and PR creation
- `documentation` for deeper documentation review beyond the drift scan in step 3
- `issue-craft` for issue lifecycle patterns and tracking issues from doc review
- `next-issue` for work initiation and session opening
