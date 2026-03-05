# ADR-0002: Groundwork Frontmatter Format

**Status:** Accepted
**Date:** 2026-03-05

## Context

Groundwork is migrating from methodology distribution to pipeline runtime (Epic #32). Every skill that participates in the pipeline must declare what stage it belongs to, what artifacts it requires, and what it produces. This metadata lives in a `groundwork:` YAML block within each skill's frontmatter.

This ADR documents three design decisions about that block's structure: the schema dialect, how artifacts are referenced, and how stages are constrained.

## Decision Drivers

- The schema must be machine-validatable — pipeline tooling will enforce contracts at skill load time
- Artifact references must be stable across projects that use different directory layouts
- The stage vocabulary must reflect pipeline topology, not just categorization
- The Rust `jsonschema` crate is the expected runtime validator

## Decision

### 1. JSON Schema Draft 2020-12

The `groundwork:` block is validated by a JSON Schema using Draft 2020-12 (`$schema: https://json-schema.org/draft/2020-12/schema`).

**Rationale:** The Rust `jsonschema` crate fully supports Draft 2020-12. The json-schema-org recommends both Draft 7 and 2020-12 as stable targets. Since there is no compatibility cost (no existing schemas to migrate), we use the newer standard.

### 2. Artifact type names, not file paths

Each entry in `requires` and `produces` identifies an artifact by its type name (e.g., `verified-constraints`, `behavior-contract`) rather than a file path.

**Rationale:** Artifact type names are stable identifiers that mean the same thing across projects. File paths are implementation details that change per project layout, per forge, and per agent workspace. Binding pipeline contracts to file paths would make skills non-portable. The `schema` field in each entry points to the artifact's schema definition (relative to `.groundwork/`), keeping structural validation separate from naming.

### 3. Closed enum for `stage`

The `stage` field accepts exactly five values: `specification`, `decomposition`, `execution`, `verification`, `completion`. The enum is closed — unknown values are rejected.

**Rationale:** Stages define pipeline topology: which skills can run when, and what handoffs are legal between them. Adding a stage changes the pipeline's structure, not just its metadata. This should be a deliberate, versioned change to the schema — not something that happens implicitly when a skill declares a novel stage name.

### 4. String-level type constraints

Artifact names are constrained to kebab-case (`^[a-z][a-z0-9]*(-[a-z0-9]+)*$`) and schema paths must begin with a lowercase alphanumeric character (`^[a-z0-9]`, `minLength: 1`). These patterns enforce what the prose in decisions 2 and 3 already states: artifact names are stable identifiers (not free text) and schema paths are relative to `.groundwork/` (not absolute or traversal paths).

## Consequences

### Good

- Pipeline tooling can validate skill contracts at load time with no custom parsing
- Artifact type names give skills a portable vocabulary for declaring dependencies
- The closed stage enum prevents accidental topology drift

### Neutral

- The schema validates only the `groundwork:` block, not the full frontmatter. Existing fields (`name`, `description`, `metadata`) remain separate concerns with their own validation.

### Bad

- Adding a new pipeline stage requires a schema change and a new ADR — this is intentional friction, but it is friction
- Artifact type names must be coordinated across skills. There is no registry yet; that is a future concern.

### Risks

- If the Rust `jsonschema` crate drops or changes Draft 2020-12 support, the schema dialect would need migration. This is low-probability given the crate's current trajectory.
- The five-stage model is based on the current pipeline design. If the pipeline architecture changes fundamentally, the enum and possibly the entire `groundwork:` block structure would need revision.
