# ADR-0004: Internal Development History Policy

**Status:** Accepted \
**Date:** 2026-03-10

## Context

Repository artifacts accumulate internal development history — skill
genealogies, rename chains, version-bump logs, add-then-remove cycles — in
places where the reader needs current state, not transition narrative. Agents
document transitions by default because transitions are what they just did.
This is useful during active development but becomes noise once the transition
is complete.

Four specific surfaces carry this pattern:

1. **Skill `origin:` fields** mix attribution (legally required for adapted
   upstream work) with development narrative (what the predecessor did wrong,
   what v3.0 broadened). Attribution must stay; narrative clutters frontmatter
   that agents parse on every skill load.

2. **Skill `replaces:` fields** reference predecessor skills — some upstream
   (obra/superpowers), some internal (clean-slate). For upstream predecessors,
   attribution is already handled by `origin:` + `LICENSE-UPSTREAM`. For
   internal predecessors that were never released, there is no reader who
   benefits from knowing the old name.

3. **CHANGELOG `[Unreleased]`** sections accumulate internal iteration: version
   bumps, renames, add-then-remove entries. Users encountering the first
   release need to know what ships, not what happened along the way.

4. **ADR bodies** continue to read as active policy after being superseded.
   A superseded note at the top helps, but without a convention the pattern
   recurs with each new ADR.

## Decision

### Principle

**Document state, not transitions.** Repo artifacts describe what exists now
and why. Development history belongs in git log and issue threads, not in
user-facing or agent-facing documents.

Exceptions: attribution (origin metadata, LICENSE-UPSTREAM) and architectural
rationale (ADR bodies) are preserved because they serve a reader need beyond
the transition itself.

### origin: field

Two rules based on whether upstream work is involved:

- **Upstream-adapted skills:** Terse attribution pointer — upstream project,
  license, and a reference to `LICENSE-UPSTREAM`. No narrative about what was
  preserved, adapted, or restructured (that detail, including the pinned
  commit hash, lives in LICENSE-UPSTREAM).
  Format: `"Adapted from <project> (<license>). See LICENSE-UPSTREAM."`

- **Internal-only skills:** No `origin:` field. Internal genealogy (skill A
  replaced skill B) is development history, not attribution. It belongs in
  git log and issue threads.

### replaces: field

Remove from all skills. The field conflates attribution (handled by `origin:`
+ `LICENSE-UPSTREAM`) with development narrative (handled by git log). It
appears in no schema, no tooling, and no reader workflow.

### CHANGELOG discipline

Only log user-visible changes. An entry belongs in CHANGELOG when an end user
or adopting agent would notice the difference. Internal iteration — version
bumps to individual skills, renames of internal concepts, add-then-remove
cycles, policy doc rewrites — does not.

When releasing, the `[Unreleased]` section describes what ships, not what
happened during development.

### ADR lifecycle

When an ADR is superseded:

1. Update the `Status:` line to include `Superseded by: <reference>`.
2. Add a dated note immediately after the status block explaining what changed
   and why the body is now historical context.
3. Preserve the full body — ADRs are historical records of *why* a decision
   was made, not living policy documents. Trimming the body loses the
   reasoning that led to the decision.

The superseded note is the reader's signal. If they need current policy, follow
the pointer. If they need the historical rationale, read the body.

### Skill versions

Keep the `version:` field in frontmatter (current state). No per-skill
changelog. Version changes are reflected in the project CHANGELOG only when
they represent user-visible changes.

## Consequences

### Good

- Agents and contributors have a clear rule for what goes where
- Skill frontmatter stays terse and machine-parseable
- CHANGELOG stays useful across releases
- ADR lifecycle has a documented convention

### Neutral

- Existing git history preserves all development narrative for anyone who
  needs it

### Bad

- Narrative context about *why* a skill evolved a certain way is less
  discoverable (buried in git log rather than visible in frontmatter)
- Contributors must exercise judgment about "user-visible" for CHANGELOG
  entries — the boundary is not a bright line
