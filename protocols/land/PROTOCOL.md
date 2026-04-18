---
name: land
description: >-
  Activates on a `patch` artifact and produces the `completion-record`
  artifact — the terminal archival record of the work unit, carrying
  criterion coverage summary, gaps, merge reference, and documentation
  status. The protocol's substantive work is confirming the packaged change
  is ready to land, obtaining the merge reference via the `forge` skill,
  and delivering the `completion-record`.
requires: ["patch"]
accepts: ["completion-evidence", "behavior-contract", "documentation-record", "issue"]
produces: ["completion-record"]
may_produce: []
trigger:
  on_artifact: "patch"
---

# Land

## Overview

The `land` protocol activates on a `patch` artifact and produces the
`completion-record` artifact — the terminal archival record of the work
unit. By the time land activates, the full execution chain has already
delivered its artifacts: `patch`, `completion-evidence`,
`documentation-record`, `behavior-contract`, and `issue` are all in
injected context.

Land operates in two phases:

- **Phase 0: Closing** — a ceremony that prepares the work for delivery.
  Four steps mirror `begin`'s opening in reverse: gather context, verify
  acceptance, review documentation, seal the gate.
- **Phase 1: Mechanical** — the forge-delegated merge and the
  `completion-record` delivery, gated by Phase 0.

Phase 0 prepares the work for delivery; `begin`'s opening prepares the
agent for work. Parallel structure, inverse direction.

---

## Procedure

### Phase 0: Closing

The closing ceremony equips the landing with everything the mechanical
phase needs: context about what was built, verification that acceptance
criteria are met, confirmation that documentation reflects reality, and a
gate that blocks the mechanical merge until readiness is confirmed.

Follows the closing sequence: gather → verify → review → seal.

#### 0a. Gather

Read the injected `patch` to recover the packaging context —
`pr_reference`, `branch`, `commit`. In forge-targeting contexts, the
`forge` skill can inspect branch state, diff, and commit history when the
ceremony needs detail beyond what the `patch` carries.

#### 0b. Verify

Read the injected `completion-evidence`. Classify each acceptance criterion
as `satisfied` (all covering scenarios pass) or `partial` (gap or failing
scenario remains). The artifact already carries criterion-level coverage;
no re-verification is needed here — that is `verify`'s work, upstream.

If `completion-evidence` shows failures or uncovered criteria, the seal
will block the mechanical phase until the upstream evidence is corrected.

#### 0c. Review

Read the injected `documentation-record`. Confirm coverage is present and
drift is either fixed or tracked — the `updated_docs`,
`verified_accurate_docs`, and `tracking_issues` fields tell the story. If
a user-visible change lacks a CHANGELOG entry, flag before proceeding;
the seal blocks on this.

Documentation review itself is `document`'s work, upstream. This step
reads the record, does not re-run the review.

#### 0d. Seal

Gate check before the mechanical phase. All three conditions must hold:

1. Verification showed no failing scenarios and no uncovered criteria.
2. Documentation record shows drift fixed or tracked.
3. CHANGELOG entry present if the change is user-visible.

If the seal fails, the `completion-record` cannot be delivered. Report
the blocking condition; the upstream protocol that produced the offending
artifact must correct it before `land` can proceed.

### Phase 1: Mechanical

Gated by Phase 0. Do not enter this phase until the seal passes.

#### 1a. Evaluate commit history for squash

Examine the branch's commit history to decide whether to squash on merge.
Apply the cognitive framework, not mechanical rules:

- **Squash when** the history is iterative refinement — a feature commit
  followed by fix-ups that revise the same change: majority of commits are
  fixes of the initial change, commits touch the same files, all share the
  same scope/component.
- **Preserve when** commits represent distinct work units — single-commit
  branches, different components/scopes, multi-step features where each
  step is meaningful independently.

Record the squash decision. If squashing, also draft a consolidated commit
message using the conventional-commit prefix/scope from the initial
commit.

#### 1b. Obtain the merge reference

In forge-targeting contexts, invoke the `forge` skill to:

- Look up the PR for the feature branch.
- Execute the merge through the forge's PR API (applying the squash
  decision from step 1a).
- Delete the feature branch.
- Close or comment on the related forge issues — satisfied issues close;
  partial issues get a progress comment listing remaining criteria.

The skill returns `merge_reference` (merge commit SHA or PR URL) and
owns the gh/git cognitive method. The protocol-level contract is that a
value for `merge_reference` exists after this step; without it, the
`completion-record` cannot be delivered.

#### 1c. Deliver the `completion-record`

Deliver the `completion-record` artifact through the session's MCP tool
named `completion-record`. Runa supplies `work_unit` from execution
context; the agent supplies `instance_id` and the cognitive content —
`criterion_summary` (how the acceptance criteria were met), `gaps`
(known gaps or deferred work, empty if none), `merge_reference` from
step 1b, and `documentation_status` (summary of documentation coverage).
The MCP server validates the payload against the artifact schema and
writes it to the artifact store.

---

## Failure Policy

- **Seal failure blocks Phase 1.** If the seal (Phase 0d) fails, the
  blocking condition must be corrected upstream. Do not proceed to the
  mechanical phase.
- **No merge reference available.** If the `forge` skill cannot produce
  `merge_reference`, the `completion-record` cannot be delivered. Stop
  and report.
- **Documentation drift blocks the seal.** Drift discovered in Phase 0c
  must be fixed or tracked before the seal can pass. Unresolved drift
  blocks delivery.

---

## Cross-References

- `submit`: the preceding protocol; `patch` is the trigger artifact this
  protocol activates on.
- `begin`: the opening bookend of the session lifecycle.
- `verify`: produces the `completion-evidence` this protocol reads from
  injected context for Phase 0b.
- `document`: produces the `documentation-record` this protocol reads
  from injected context for Phase 0c.
- `decompose`: produces the `issue` artifact that thread-roots every
  work unit this protocol finalizes.
- `forge`: the skill that owns gh/git cognitive methods — invoked from
  Phase 1b to produce the merge reference.
