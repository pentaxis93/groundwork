---
name: submit
description: >-
  Activates on a `documentation-record` artifact and produces the `patch`
  artifact — the packaged change ready for review. The protocol's substantive
  work is analyzing changes, planning commit structure, and obtaining the
  forge values (`pr_reference`, `branch`, `commit`) that the `patch` requires.
requires: ["completion-evidence", "documentation-record"]
accepts: []
produces: ["patch"]
may_produce: []
trigger:
  on_artifact: "documentation-record"
---

# Submit

## Overview

The `submit` protocol activates on a `documentation-record` artifact and
produces the `patch` artifact — the packaged change ready for review. By the
time submit activates, `completion-evidence` and `documentation-record` are
both in injected context, confirming the work is verified and its
documentation is covered.

Submit's substantive work is:

1. Analyzing changes and planning commit structure (the cognitive
   discipline).
2. Obtaining the forge values the `patch` requires — `pr_reference`,
   `branch`, `commit` — via the `forge` skill in forge-targeting contexts.
3. Delivering the `patch`.

Submit is the transition from execution to review. It comes after `document`
and before `land`.

---

## Preconditions

- The working tree must carry uncommitted or unpushed work. If there is
  nothing to package, the protocol should not have activated; report and
  stop.
- If an open PR already exists for the current branch, report and stop —
  the operator likely wants `land`, not `submit`.

---

## Procedure

### 1. Receive injected context

Runa has delivered `completion-evidence` and `documentation-record`. Read
both. The criterion-level coverage and the documentation-coverage status
together frame what the packaged change carries.

### 2. Analyze changes and plan commit structure

This step is the protocol's core cognitive work: understanding the change
well enough to produce meaningful commit structure, not just staging
everything at once.

**2a. Understand intent.** Examine all uncommitted modifications and
cross-reference them against the acceptance criteria carried in
`completion-evidence`.

**2b. Identify logical groups.** Find related changes that belong in the
same atomic commit. Apply the Complete Feature Rule: implementation,
documentation, and tests for one feature belong in one commit — never split
docs from the code they document.

**2c. Plan commits.** Design atomic commits, each serving a single purpose.
Use conventional commit format: `type(scope): description`, where type is
`feat`, `fix`, `refactor`, `docs`, `test`, or `chore`. Reference relevant
issues in the commit body (not the title). Commit messages explain **why**,
not just what.

If analysis produces no clear grouping (one tangled change), fall back to a
single commit with a comprehensive message. A single honest commit is
better than artificial splitting.

### 3. Obtain forge values

The `patch` artifact requires `pr_reference`, `branch`, and `commit`. In
forge-targeting contexts, invoke the `forge` skill to produce the commit
series, push the branch, and create the PR; the skill returns the three
values and owns the gh/git cognitive method.

### 4. Deliver the `patch`

Deliver the `patch` artifact through the session's MCP tool named `patch`.
Runa supplies `work_unit` from execution context; the agent supplies
`instance_id` and the values obtained in step 3. The MCP server validates
the payload against the artifact schema and writes it to the artifact
store.

---

## Failure Policy

- **Nothing to submit:** Clean working tree with no unpushed work — the
  protocol should not have activated; report and stop.
- **No forge values:** If the forge skill cannot produce `pr_reference`,
  `branch`, and `commit`, the `patch` cannot be delivered. Stop and report.

---

## Corruption Modes

- `premature-submit`: the protocol activates with incomplete work.
  Recognition: tests not run, WIP markers in code (TODO, FIXME, incomplete
  stubs). Submit is not a verification gate, but obvious signs of
  incomplete work should block delivery.
- `empty-submit`: nothing to submit. Recognition: clean tree, no unpushed
  commits. Report and stop.
- `split-avoidance`: dumping all changes into one commit to skip analysis.
  Recognition: a single commit touching many unrelated files with a vague
  message. Step 2 exists to prevent this.
- `submit-as-land`: treating the `patch` as the end of the workflow.
  Submit is the middle of the lifecycle — `land` produces the
  `completion-record` that closes the chain.

---

## Cross-References

- `document`: the preceding protocol; `documentation-record` is the trigger
  artifact this protocol activates on.
- `land`: the next protocol; activates on the `patch` this protocol
  produces.
- `verify`: produces the `completion-evidence` this protocol reads from
  injected context.
- `begin`: the opening bookend of the session lifecycle.
- `forge`: the skill that owns gh/git cognitive methods — invoked from
  step 3 to produce forge values.
