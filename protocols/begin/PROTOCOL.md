---
name: begin
description: >-
  Activates on an `issue` artifact and produces the `claim` artifact — the
  threading root that carries work-unit identity into the execution-phase
  chain. Opening bookend of the session lifecycle; `land` is the closing
  bookend.
requires: ["issue"]
accepts: []
produces: ["claim"]
may_produce: []
trigger:
  on_artifact: "issue"
---

# Begin

The `begin` protocol activates on an `issue` artifact and produces the `claim`
artifact — the threading root that carries work-unit identity into the
execution-phase chain (`specify` → `plan` → `implement` → `verify` →
`document` → `submit` → `land`).

By the time `begin` activates, runa has already resolved selection: the issue
being worked is in injected context. Begin's substantive work is
acknowledging that issue as active, distilling it into work-unit scope, and
delivering the `claim`.

Begin is the opening bookend of the session lifecycle; `land` is the closing
bookend.

## Procedure

1. **Orient.** If the methodology map has not been loaded this session,
   invoke the `orient` skill. This retains the methodology-loading value of
   the prior four-phase opening — the subsequent protocols in the chain
   operate as one connected system, not in isolation.

2. **Read the injected `issue` artifact.** Runa has delivered the issue
   being worked. Read its fields — title, description, acceptance criteria,
   dependencies. Confirm every hard dependency is closed; if any remain
   open, the session should not have activated. Distill the issue into a
   brief `scope` — a single statement of what is being claimed this session.

3. **Deliver the `claim`** through the session's MCP tool named `claim`.
   Runa supplies `work_unit` from execution context; the agent supplies
   `instance_id` and `scope`. The MCP server validates the payload against
   the artifact schema and writes it to the artifact store.

## Key Terms

Brief definitions for self-contained use. See
[`issue-model.md`](../../docs/architecture/issue-model.md) for the full treatment.

- **Issue graph**: the set of open issues and their dependency edges — the live
  map of what remains and what blocks what.
- **Unblocked**: an issue whose hard dependencies are all closed.
- **Execution layer**: a set of issues that share no mutual dependencies and can
  be worked in parallel once their shared ancestors are closed. Layer 0 has no
  dependencies; layer 1 depends only on layer 0; and so on.
- **Session-sized**: an issue that one agent can complete — from reading context
  through passing verification — in a single focused session.
- **Issue batch**: 2-3 cohesive issues addressed together when they share a
  concern boundary and their combined scope is still session-sized.

## Operating Principles

- **Direction over prediction.** Capture starting direction in the `scope`
  field of the `claim`. Goals sharpen through implementation — rigid upfront
  done conditions are premature precision.
- **One session, one increment.** The claim commits to one independently
  verifiable increment. Fewer, sharper goals beat broad, vague activity — this
  keeps work finishable and reviewable.
- **Dependencies are hard blockers.** If any hard dependency on the injected
  issue is open, the session should not have activated. Blocked work produces
  partial results that complicate the graph.

## Corruption Modes

- `scope-creep`: the claim's `scope` field widens beyond what the issue's
  acceptance criteria actually cover, carrying unrelated work into the
  execution chain.
- `blocker-bypass`: activating on an issue whose hard dependencies are still
  open, on the reasoning that "we can sort them out mid-session."

## Cross-References

- `specify`: the next protocol in the chain — activates on the `claim`
  this protocol produces.
- `decompose`: the protocol that produces the `issue` artifacts this
  protocol activates on.
- `land`: the closing bookend of the session lifecycle.
- `orient`: the methodology-map skill, invoked in step 1.
- `reckon`: first-principles constraint skill; invoke when the issue's
  framing needs validation before a claim is delivered.
