# ADR-0003: Artifact Frontmatter Format

**Status:** Provisional
**Date:** 2026-03-05

## Context

With the pipeline contract schema defined for skills (ADR-0002), every artifact produced by the pipeline also needs machine-readable metadata. This frontmatter lives at the top of each artifact file in `.groundwork/artifacts/` and is the read surface for `groundwork check` — it builds the dependency graph and computes pipeline state from these fields.

This ADR documents four design decisions about that frontmatter's structure: the two-axis status model, why certain states are computed rather than stored, how dependencies reference other artifacts, and how artifact content format is determined.

## Decision Drivers

- The frontmatter must support both agent-driven and human-driven state transitions
- Pipeline state must be derivable without global coordination between artifacts
- Artifact references must be stable across projects with different directory layouts
- The schema must be machine-validatable using the same `jsonschema` tooling as skill frontmatter

## Decision

### 1. Two-axis status model: freshness x approval

Artifact status is modeled as two orthogonal axes rather than a single status field:

- **`freshness`** (`fresh` | `stale`) — determined by agents. An artifact is `fresh` when it was produced from current inputs; `stale` when any of its dependencies have changed since production.
- **`approval`** (`pending` | `approved` | `rejected`) — determined by humans. Present only when `pipeline.toml` requires human review for the artifact type.

**Rationale:** Agent-determined and human-determined states change at different times, through different mechanisms, and for different reasons. Collapsing them into a single enum (e.g., `draft`, `reviewed`, `approved`, `stale`) creates ambiguous states: is a `stale` artifact one that was never reviewed, or one that was approved but whose inputs changed? Two orthogonal axes make every combination expressible and unambiguous.

### 2. Computed states: `blocked` and `ready` are derived, not stored

The states `blocked` (waiting on upstream dependencies) and `ready` (all dependencies satisfied, eligible to run) are computed by `groundwork check` from the dependency graph. They are not stored in artifact frontmatter.

**Rationale:** Storing `blocked` or `ready` in individual artifacts would require keeping them in sync whenever any upstream artifact changes. This creates a distributed consistency problem — every artifact update would need to cascade through the graph. Instead, `groundwork check` reads all artifact frontmatter, builds the dependency graph, and computes these states on demand. The graph is the single source of truth; individual artifacts only store their own local state.

### 3. Artifact names, not file paths

The `depends_on` array references artifacts by type name (e.g., `behavior-contract`) or by specific filename (e.g., `tested-implementation.yaml`), not by file paths. The dependency entry pattern (`^[a-z][a-z0-9]*([-.][a-z0-9]+)*$`) is slightly more permissive than the skill-schema artifact entry pattern to accommodate filenames with extensions.

**Rationale:** Same rationale as ADR-0002 decision 2 — artifact type names are stable identifiers that mean the same thing across projects. File paths are implementation details. The `schema` field (separate from dependency entries) points to the artifact type's schema definition, keeping structural validation separate from dependency declaration.

### 4. Artifact content format

The content below the frontmatter varies by artifact type:

- **YAML frontmatter + Markdown body** for prose-heavy artifacts (e.g., implementation plans, behavior contracts). The frontmatter is validated by this schema; the Markdown body carries the artifact's content but is not schema-validated.
- **Pure YAML** for structured artifacts (e.g., test evidence, completion records). The entire file is YAML, and both frontmatter and body sections are schema-validatable.

The artifact type schema (referenced by the `schema` field in frontmatter) determines which format applies.

**Rationale:** Prose-heavy artifacts need Markdown for readability — forcing implementation plans into YAML structures would sacrifice clarity for machine-parseability. Structured artifacts benefit from end-to-end schema validation. Rather than forcing one format on all artifact types, the type schema declares the expected format. This keeps the frontmatter schema simple (it validates only the common metadata header) while allowing type schemas to specify their own content structure.

### 5. String-level type constraints

The `schema` field uses the same relative-path pattern as ADR-0002 decision 4 (`^[a-z0-9]`, `minLength: 1`). The `produced_by` field uses the kebab-case skill-name pattern (`^[a-z][a-z0-9]*(-[a-z0-9]+)*$`). The `depends_on` artifact pattern (`^[a-z][a-z0-9]*([-.][a-z0-9]+)*$`) is slightly more permissive to allow filenames with extensions. The `produced_at` field uses a regex for ISO 8601 with mandatory timezone; fractional seconds are intentionally excluded — producing agents must truncate to whole seconds.

As with ADR-0002: the `schema` path pattern blocks leading `../` and `/` but not mid-path traversal segments (e.g., `a/../b`); runtime path resolution must canonicalize.

## Consequences

### Good

- `groundwork check` can build the full dependency graph from frontmatter alone, without parsing artifact bodies
- The two-axis model makes agent and human state transitions independent — no coordination required
- Computed states (`blocked`, `ready`) are always consistent because they derive from the graph, not from stored values
- The content format decision avoids a false choice between prose readability and schema validation

### Neutral

- The `approval` field is optional at the schema level. Whether an artifact type requires approval is a pipeline configuration concern (`pipeline.toml`), not a format concern.

### Bad

- Two axes mean more states to reason about (6 combinations when approval is present). Tooling must present these clearly.
- Dependency names must be coordinated across skills and artifacts. There is no registry yet; that is a future concern (same as ADR-0002).

### Risks

- If the dependency graph grows large, computing `blocked`/`ready` on every `groundwork check` invocation could become slow. Caching strategies can mitigate this if needed.
- The content format split (Markdown vs. pure YAML) means tooling must handle two parsing paths. The `schema` field tells tooling which path to take, but this adds implementation complexity.
