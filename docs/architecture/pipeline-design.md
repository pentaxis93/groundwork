# Groundwork Pipeline Design

Audience: contributors and agents who need both current implementation truth and the target pipeline architecture.

## Reading Mode

This document uses three status labels to separate descriptive truth from normative design:

- `Implemented` — present in the repository today.
- `Designed` — agreed target design, not fully implemented.
- `Pending` — open question or research-dependent area.

As of **March 6, 2026**, Epic 1 tasks `#33`, `#34`, `#35`, and `#36` are closed; Epic issue `#32` remains open for later epics.

## As-Built Baseline (`Implemented`)

### CLI and `.groundwork/` behavior

- `groundwork init` and `groundwork update` reconcile `agents.toml`, run `sk sync`, and provision:
  - `.groundwork/schemas/`
  - `.groundwork/artifacts/`
- Managed schemas are embedded at compile time and reconciled content-aware:
  - missing file -> create
  - changed file -> overwrite
  - matching file -> unchanged
  - extra files in `.groundwork/schemas/` -> preserved
- If `.groundwork/schemas` or `.groundwork/artifacts` exists but is not a directory, init/update fail fast with explicit errors.
- Corrupted/non-UTF8 managed schema files are treated as drift and overwritten during sync.
- `groundwork doctor` reports schema health without enforcing:
  - missing managed schemas -> `warn`
  - drifted managed schemas -> `warn`
  - unreadable managed schemas -> `warn` (non-fatal)
  - extra schema files -> `info`

### Epic 1 schema and contract artifacts

Shipped schemas in `schemas/` (`Implemented`):

- `groundwork-frontmatter.schema.json` (`#33`)
- `artifact-frontmatter.schema.json` (`#34`)
- `behavior-contract.schema.json` (`#35`)
- `implementation-plan.schema.json` (`#35`)
- `test-evidence.schema.json` (`#35`)
- `completion-evidence.schema.json` (`#35`)
- `completion-record.schema.json` (`#35`)
- `research-record.schema.json` (`#35`)

Epic 1 implementation integration (`#36`):

- embedded schema distribution in CLI init/update
- schema diagnostics in `groundwork doctor`
- `.groundwork/artifacts/` creation during install/update reconciliation

## Target Pipeline Design (`Designed`)

### Design principles

- Normative-first boundary design: define what boundaries must enable, then map implementation.
- Filesystem as integration substrate: durable artifacts in `.groundwork/artifacts/`, not ephemeral handoff.
- Two-axis artifact state:
  - freshness (mechanical, dependency-driven)
  - approval (policy/gate-driven, optional)

### Dependency model

- Static dependency topology: skill-level contracts in `groundwork:` frontmatter.
- Dynamic dependencies: artifact-level `depends_on` declared at creation time.
- Derived states (`blocked`, `ready`, `next action`) computed from graph state, not stored.

### Boundary artifact flow (target)

- `intake-spec` -> `behavior-contract` -> `implementation-plan` -> decomposition outputs -> execution evidence -> completion evidence -> `completion-record`
- Optional/emergent artifacts (for example `research-record-*`) attach via dynamic dependencies.
- `behavior-contract` remains the threading contract across planning, execution, verification, and closure.

## Gaps and Non-Goals (Current)

### Not implemented yet (`Designed`, not `Implemented`)

- `groundwork check` command and full graph/state computation.
- `pipeline.toml` parsing and approval policy configuration.
- Extended schema set beyond Epic 1 (for example `intake-spec`, `design-exploration`, `execution-steps`, `review-record`, `documentation-review-record`).
- Wrapper skill topology for curated boundary skills.

### Structural/decomposition open area (`Pending`)

- Decomposition-stage topology is settled in the live pipeline as `plan` for convergence and `issue-craft` for executable work units.
- Wrapper-skill policy for curated boundary skills remains open beyond the current decomposition routing.

## Source of Truth and Cross-References

- Pipeline operating contract: `docs/architecture/pipeline-contract.md`
- System overview: `ARCHITECTURE.md`
- Integration manual: `WORKFLOW.md`
- Accepted architecture decisions:
  - `docs/architecture/decisions/0001-pipeline-integration-strategy.md`
  - `docs/architecture/decisions/0002-groundwork-frontmatter-format.md`
  - `docs/architecture/decisions/0003-artifact-frontmatter-format.md`
