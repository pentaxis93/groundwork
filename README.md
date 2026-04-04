# Groundwork

Groundwork is a methodology plugin for [runa](https://github.com/pentaxis93/runa),
a cognitive runtime for AI coding agents.
It encodes opinions about how software should be built into protocols, skills,
and artifact schemas that a runa instance orchestrates. It is not a runtime, a
CLI, or a framework — it is a methodology definition.

For what methodology plugins are and how runa executes them, see runa's
[core concepts](https://github.com/pentaxis93/runa#core-concepts).

## What Groundwork Believes

These are the methodology choices embedded in groundwork's protocols and skills.
Each traces to the file where it lives.

### How work is understood

**Descriptive is not normative.** What currently exists is evidence, not
constraint. What is actually needed is the starting point for design. Treating
the existing system as the definition of correctness is the most common framing
failure.
→ [`skills/reckon/SKILL.md`](skills/reckon/SKILL.md)

**The issue graph is working memory.** Agent sessions end, context windows
close, agents rotate. The issue graph is the persistence layer that survives
those boundaries — the only working memory reliable across sessions. Work from
the graph, not from memory.
→ [`docs/architecture/issue-model.md`](docs/architecture/issue-model.md)

**Sovereignty.** Every handoff passes outcomes — what must be true — never
implementation steps. Issues define acceptance criteria, not procedure. Plans
define interfaces and decisions, not scripts to follow. The implementer solves
the problem; the author defines the problem.
→ [`protocols/decompose/PROTOCOL.md`](protocols/decompose/PROTOCOL.md)

### How work is executed

**Behavior is the thread.** The behavior contract written during specify traces
through every subsequent stage. Plans link design decisions to behavior
scenarios. Tests verify named scenarios. Verification cites behavior-level
evidence. Landing records what coverage shipped. If you cannot name which
behavior a test verifies, traceability is lost.
→ [`protocols/specify/PROTOCOL.md`](protocols/specify/PROTOCOL.md),
[`skills/contract/SKILL.md`](skills/contract/SKILL.md)

**Evidence before assertion.** No completion claims without fresh verification
evidence. No fixes without root cause investigation. No implementation plans
without grounded constraints. Confidence is not evidence.
→ [`protocols/verify/PROTOCOL.md`](protocols/verify/PROTOCOL.md),
[`skills/debug/SKILL.md`](skills/debug/SKILL.md)

**Test-driven execution.** No production code without a failing test first.
Watch the test fail; confirm it fails for the right reason. Write the minimum
code to pass. Code written before its test gets deleted and restarted — not
kept as reference.
→ [`protocols/implement/PROTOCOL.md`](protocols/implement/PROTOCOL.md)

**Code is ground truth.** When documentation and code disagree, code behavior
is descriptive truth. Documentation is a claim that must be verified against
the code.
→ [`protocols/document/PROTOCOL.md`](protocols/document/PROTOCOL.md)

**Documentation obligation.** User-facing changes carry documentation
requirements. Documentation ships in the same PR as the code that caused it.
Drifted documentation compounds — each instance trains readers to distrust all
documentation.
→ [`protocols/document/PROTOCOL.md`](protocols/document/PROTOCOL.md),
[`skills/orient/SKILL.md`](skills/orient/SKILL.md)

### How obstacles are handled

**Root cause before fixes.** When a test fails or behavior is unexpected,
investigate before proposing any fix. After three failed fix attempts, stop
fixing and question the architecture.
→ [`skills/debug/SKILL.md`](skills/debug/SKILL.md)

**Friction is structural.** Workarounds compound debt. When operational friction
appears — a missing tool, broken configuration, stale convention — resolve it
structurally before continuing. If the fix exceeds side-quest scope, file an
issue. Do not route around it.
→ [`skills/resolve/SKILL.md`](skills/resolve/SKILL.md)

**The third force.** When task momentum collides with an obstacle, neither
suppress the task nor accept the obstacle. Introduce the reconciling move that
transforms the situation — the force that makes both sides yield something
useful.
→ [`skills/resolve/SKILL.md`](skills/resolve/SKILL.md)

## The Shape of the Methodology

Work moves through two phases connected by the issue artifact.

**Planning** takes an external request and produces issue-sized work units.
Survey examines what actually needs doing; decompose breaks that into issues
with acceptance criteria and dependency edges.

**Execution** takes one issue and carries it through to a merged increment:
begin claims the issue and opens the session → specify writes the behavior
contract as Given/When/Then scenarios → plan converges on a decision-complete
design → implement executes through RED-GREEN-REFACTOR → verify gates
completion with evidence → document ensures accuracy → submit packages the
change → land merges and closes the loop.

Each protocol produces an artifact that the next protocol requires. The
artifact chain is the methodology's state — there is no second channel.
→ [`docs/architecture/connecting-structure.md`](docs/architecture/connecting-structure.md)

Six skills operate across the topology:

- **orient** — the methodology map that connects protocols and skills
- **reckon** — first-principles reasoning when creating or analyzing
- **debug** — root cause investigation when failures appear
- **resolve** — structural friction resolution when obstacles impede
- **research** — external evidence gathering when facts are missing
- **contract** — behavior traceability through execution

Not every piece of work needs every stage. A bug with an existing issue enters
at execution. A new capability enters at planning. The constraint is sequence —
you cannot land before executing — not completeness.
→ [`skills/orient/SKILL.md`](skills/orient/SKILL.md)

For how runa orchestrates this topology at runtime, see the
[interface contract](https://github.com/pentaxis93/runa/blob/main/docs/interface-contract.md).

## What the Repo Contains

| Path | Contains |
|------|----------|
| [`groundwork.toml`](groundwork.toml) | Manifest: artifact types, protocol topology, trigger conditions |
| [`protocols/`](protocols/) | 10 protocol definitions — one per stage |
| [`skills/`](skills/) | 6 skills — orientation and cross-cutting disciplines |
| [`schemas/`](schemas/) | JSON Schema contracts for each artifact type |
| [`docs/architecture/`](docs/architecture/) | Topology design rationale and issue state model |

For how these pieces compose into a methodology plugin, see runa's
[methodology authoring guide](https://github.com/pentaxis93/runa/blob/main/docs/methodology-authoring-guide.md).

## License

[MIT](LICENSE)
