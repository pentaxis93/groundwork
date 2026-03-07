---
name: land
description: >-
  One-word closeout: land this branch. Merge to main, sync, delete feature
  branch, verify acceptance criteria, close satisfied issues, comment progress
  on partial issues. Use after review is complete.
  Trigger on: 'land', 'land this', 'merge and close', 'ship it'.
---

# Land — Merge, Sync, Cleanup, Close

**Version 1.6**

## Overview

Use this skill when the user wants full delivery closure in one command.

`land` means:
1. Verify CHANGELOG covers user-visible changes
2. Scan for documentation drift; fix or flag
3. Evaluate acceptance criteria against the branch diff
4. Evaluate commit history; squash when iterative refinement adds noise
5. Discover PR number
6. Merge via PR API and sync local state
7. Remove the feature branch (local; remote usually handled by merge)
8. Close satisfied issue(s); comment progress on partial issue(s)
9. Verify final state (including documentation coverage summary)

Do not stop after merge — stopping leaves branches dangling and issues unclosed. The full sequence is atomic: merge through close.

Do not ask for confirmation before landing. Invoking `land` IS the user's approval to execute the entire workflow.

---

## Preconditions

- Working tree must be clean before starting.
- Current branch must not be `main`.
- CHANGELOG must include entries for user-visible changes. Version bumps must state the rationale for the increment level.
- Issue number(s) must be known:
  - Prefer explicit user-provided issue number(s).
  - Else infer from branch name using one of:
    - `issue-<number>/<slug>` (single issue)
    - `issues-<number>-<number>-.../<slug>` (multi-issue, unbounded)

---

## Procedure

### 1. Resolve context

Confirm the current branch is not `main` and the working tree is clean. Record the feature branch name.

Extract issue numbers from the branch name:
- `issue-42/fix-login` → issue 42
- `issues-5-8/license-cleanup` → issues 5 and 8

If neither the user nor the branch name provides issue numbers, stop and ask the user.

### 2. Verify CHANGELOG

Check whether `CHANGELOG.md` appears in the branch diff (`git diff origin/main...HEAD --name-only`).

This is a **hard gate** per the pipeline contract — user-visible changes must not land without a CHANGELOG entry. If absent, confirm with the user before proceeding. The only valid exception is a branch with zero user-visible changes, which the user must explicitly confirm.

### 3. Documentation drift scan

Scan for obvious documentation impacts from the changed files. This is a focused drift check, not a full documentation review.

1. List changed files in the branch diff.
2. Check whether changes affect areas with documentation artifacts (README, ARCHITECTURE, CONTRIBUTING, API docs).
3. Fix simple drift directly on the feature branch and commit the fix.
4. For deeper documentation work beyond this landing's scope, file tracking issues.
5. Record a documentation coverage summary: each artifact checked, its status (accurate, drifted, missing, not applicable), and action taken (updated, verified, or tracking issue filed with number).

**Failures are warnings, not blockers.** If the drift scan encounters errors (skill unavailable, classification unclear), report the issue in the coverage summary and proceed to merge. The CHANGELOG gate in step 2 satisfies the pipeline contract minimum; the drift scan is best-effort.

### 4. Evaluate acceptance criteria

Fetch each target issue (`gh issue view`) and evaluate whether the branch changes satisfy its acceptance criteria. This step runs pre-merge while the branch diff is available.

1. **Extract acceptance criteria** from the issue body — checklist items (`- [ ]`), an acceptance criteria section, or explicitly stated requirements.
2. **Evaluate each criterion** against the branch diff (`git diff origin/main...HEAD`). A criterion is met when the changes demonstrably satisfy it.
3. **Classify:**
   - **Satisfied** — all extracted criteria met.
   - **Partial** — some criteria met but others remain, **or** no acceptance criteria found in the issue body. Closing an issue without verified criteria is not safe — the issue stays open for human review.

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

### 9. Comment and close issue(s)

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

If `gh-issue-sync` is available on `PATH`, run `gh-issue-sync pull` to update the local issue mirror. If the tool is not installed or the sync fails, skip gracefully — a sync failure after successful merge is not catastrophic. Remote state is already correct; only the local mirror is stale.

### 10. Verify and report

Confirm success conditions:
- Current branch is `main`
- Working tree is clean
- Feature branch absent on origin
- PR state is `MERGED` (not just `CLOSED`)
- Every satisfied issue state is `CLOSED`
- Every partial issue has a progress comment listing remaining criteria
- Documentation coverage summary reported

Report the final state including:
- Issue disposition: satisfied (closed) and partial (open with remaining criteria)
- Documentation coverage summary from step 3
- Any warnings or failed operations from earlier steps

---

## Failure Policy

- If `gh pr merge` fails: stop immediately, do not close issue. If the failure is transient (network), retry once. If structural (merge conflict, check failure), report and stop.
- If PR API merge fails but local fallback succeeds: warn that the PR will show as "closed" rather than "merged" on GitHub.
- If branch deletion fails after successful merge: warn about the deletion failure and continue to issue close/comment steps. The code is safely on `main`; branch cleanup is not a prerequisite for issue closure.
- If issue comment/close API fails for one issue: continue processing remaining issues, then report failed issue number(s) explicitly.
- If acceptance criteria evaluation fails (issue fetch error, criteria unparseable): treat the issue as partial, log a warning, and do not close it. The operator must resolve manually.
- If documentation drift scan fails (skill unavailable, classification error): report the error in the coverage summary and proceed. Do not block the merge.
- If commit history evaluation is uncertain: default to preserve (`--no-ff`). Squashing is an optimization; when in doubt, keep the original history.

---

## Related Skills

- `documentation` for deeper documentation review beyond the drift scan in step 3
- `issue-craft` for issue lifecycle patterns and tracking issues from doc review
- `next-issue` for issue selection and session workflow
