---
name: land
description: "One-word closeout workflow: merge active branch to main, sync local main, delete feature branch (remote+local), verify acceptance criteria, close satisfied issue(s), comment progress on partial issue(s). Trigger on: 'land', 'merge and close', 'ship it'."
---

# Land — Merge, Sync, Cleanup, Close

**Version 1.3**

## Overview

Use this skill when the user wants full delivery closure in one command.

`land` means:
1. Verify CHANGELOG covers user-visible changes
2. Run documentation coherence check; fix drifted docs on the feature branch
3. Evaluate acceptance criteria against the branch diff
4. Merge the active feature branch into `main`
5. Push `main`
6. Remove the feature branch (remote + local)
7. Close satisfied issue(s); comment progress on partial issue(s)
8. Verify final state (including documentation coverage summary)

Do not stop after merge.
Do not ask for an additional confirmation before landing; invoking `land` is the user's approval to execute this workflow.

---

## Preconditions

- Working tree must be clean before starting.
- Current branch must not be `main`.
- Commands in this skill are Bash-specific and must be run under `bash`.
- CHANGELOG must include entries for user-visible changes. Version bumps must state the rationale for the increment level.
- Issue number(s) must be known:
  - Prefer explicit user-provided issue number(s).
  - Else infer from branch name using one of:
    - `issue-<number>/<slug>` (single issue)
    - `issues-<number>-<number>-.../<slug>` (multi-issue, unbounded)

---

## Procedure

### 1. Resolve context

```bash
FEATURE_BRANCH="$(git branch --show-current)"
if [ "$FEATURE_BRANCH" = "main" ]; then
  echo "ERROR: land must run from a feature branch"; exit 1
fi

if [ -z "${BASH_VERSION:-}" ]; then
  echo "ERROR: land requires bash (BASH_VERSION not set)"; exit 1
fi

git diff --quiet && git diff --cached --quiet || {
  echo "ERROR: working tree not clean"; exit 1;
}

ISSUE_NUMBERS=()

if [[ "$FEATURE_BRANCH" =~ ^issue-([0-9]+)/ ]]; then
  ISSUE_NUMBERS=("${BASH_REMATCH[1]}")
elif [[ "$FEATURE_BRANCH" =~ ^issues-([0-9]+(-[0-9]+)*)/ ]]; then
  IFS='-' read -r -a ISSUE_NUMBERS <<< "${BASH_REMATCH[1]}"
else
  echo "ERROR: cannot infer issue number(s) from branch name; require explicit issue number(s)"; exit 1
fi

if [ "${#ISSUE_NUMBERS[@]}" -eq 0 ]; then
  echo "ERROR: parsed zero issue numbers"; exit 1
fi
```

### 2. Verify CHANGELOG

```bash
if git diff origin/main...HEAD --name-only | grep -q CHANGELOG.md; then
  echo "CHANGELOG entry present"
else
  echo "WARNING: no CHANGELOG entry — verify this branch has no user-visible changes"
fi
```

If the warning fires, confirm with the user before proceeding.

### 3. Documentation coherence check

Identify changed files:

```bash
git diff origin/main...HEAD --name-only
```

Invoke the `documentation` skill's `documentation-review` mode against the changed files:

1. Map code changes to documentation artifacts per the `documentation` skill's artifact table.
2. Classify each mapped document as `accurate`, `drifted`, `missing`, or `obsolete`.
3. Update or create drifted/missing docs on the feature branch. Commit fixes before proceeding.
4. For deeper documentation work beyond the scope of this landing, file tracking issues using `issue-craft`.
5. Record the classification results as `DOC_COVERAGE_SUMMARY` for the verify step: a plain-text summary listing each documentation artifact checked, its classification (`accurate`/`drifted`/`missing`/`obsolete`), and the action taken (updated, verified, or tracking issue filed with number).

If all documentation is `accurate` and no updates are needed, record that and proceed.

### 3a. Evaluate acceptance criteria

Fetch each target issue and evaluate whether the branch changes satisfy its acceptance criteria. This step runs pre-merge while the branch diff is available.

```bash
for ISSUE_NUMBER in "${ISSUE_NUMBERS[@]}"; do
  gh issue view "$ISSUE_NUMBER" --json title,body --jq '"\(.title)\n\n\(.body)"' || {
    echo "WARNING: could not fetch issue #$ISSUE_NUMBER"; continue
  }
done
```

For each issue:

1. **Extract acceptance criteria** from the issue body — checklist items (`- [ ]`), an acceptance criteria section, or explicitly stated requirements.
2. **Evaluate each criterion** against the branch diff (`git diff origin/main...HEAD`). A criterion is met when the changes demonstrably satisfy it.
3. **Classify:**
   - **Satisfied** — all criteria met, or no acceptance criteria found in the issue body.
   - **Partial** — some criteria met, others remain. Record which criteria are satisfied and which remain.

Record results as `SATISFIED` and `PARTIAL` issue lists for step 7.

### 4. Merge and push

```bash
git fetch origin --prune
git checkout main
git pull --ff-only origin main
git merge --no-ff "$FEATURE_BRANCH"
git push origin main
MERGE_SHA="$(git rev-parse --short HEAD)"
```

### 5. Delete feature branch

```bash
git push origin --delete "$FEATURE_BRANCH" || true
git branch -d "$FEATURE_BRANCH"
git fetch origin --prune
```

### 6. Discover PR number (best effort)

```bash
PR_NUMBER="$(gh pr list --head "$FEATURE_BRANCH" --state merged --json number --jq '.[0].number')"
```

If PR is not found, continue with issue close using merge commit only.

### 7. Comment and close issue(s)

Apply the classifications from step 3a.

```bash
FAILED_OPS=()

for ISSUE_NUMBER in "${SATISFIED[@]}"; do
  if [ -n "$PR_NUMBER" ]; then
    BODY="Implemented and merged in PR #${PR_NUMBER} (commit ${MERGE_SHA}). Closing as complete."
  else
    BODY="Implemented and merged in commit ${MERGE_SHA}. Closing as complete."
  fi
  gh issue comment "$ISSUE_NUMBER" --body "$BODY" || FAILED_OPS+=("comment:#$ISSUE_NUMBER")
  gh issue close "$ISSUE_NUMBER" --reason completed || FAILED_OPS+=("close:#$ISSUE_NUMBER")
done

for ISSUE_NUMBER in "${PARTIAL[@]}"; do
  # Compose BODY from step 3a evaluation — see template below.
  gh issue comment "$ISSUE_NUMBER" --body "$BODY" || FAILED_OPS+=("comment:#$ISSUE_NUMBER")
done

if [ "${#FAILED_OPS[@]}" -gt 0 ]; then
  echo "WARNING: failed operations: ${FAILED_OPS[*]}"
fi
```

For each partial issue, compose `BODY` with the PR/commit reference, then bulleted lists of **Satisfied** and **Remaining** criteria from step 3a. If no PR was found, reference the commit only.

### 7a. Sync issue state to local mirror

```bash
gh-issue-sync pull
```

### 8. Verify and report

```bash
git status --short
git branch --show-current
git rev-parse --short HEAD

for ISSUE_NUMBER in "${ISSUE_NUMBERS[@]}"; do
  gh issue view "$ISSUE_NUMBER" --json state,title --jq '"\(.state) \(.title)"'
done
```

Report:
- Issue disposition: `SATISFIED` (closed) and `PARTIAL` (open with remaining criteria).
- `DOC_COVERAGE_SUMMARY` from step 3: which docs were updated, verified accurate, or flagged with tracking issues.

Success conditions:
- current branch is `main`
- working tree is clean
- feature branch absent on origin
- every satisfied issue state is `CLOSED`
- every partial issue has a progress comment listing remaining criteria
- documentation coverage summary reported

---

## Failure Policy

- If merge/push fails: stop immediately, do not close issue.
- If branch deletion fails after successful merge: report partial completion and keep issue(s) open.
- If issue comment/close API fails for one issue: continue processing remaining issues, then report failed issue number(s) explicitly.
- If acceptance criteria evaluation fails (issue fetch error, criteria unparseable): default to satisfied for that issue and log a warning. Do not block the landing on evaluation ambiguity.
- If documentation coherence check fails (skill unavailable, classification error, or commit failure): stop and report the error. Do not proceed to merge with unresolved documentation state.

---

## Related Skills

- `documentation` for documentation coherence check (step 3)
- `issue-craft` for issue lifecycle patterns and tracking issues from doc review
- `next-issue` for issue selection and session workflow
