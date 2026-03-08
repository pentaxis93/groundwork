---
name: documentation
description: >-
  Use when project documentation needs creation, review, or update —
  after code changes that may cause drift, at project initialization,
  when architectural decisions are made, or when existing docs fail
  the audience test. Also use when documentation quality is degrading:
  stale references, verbose-but-unhelpful content, or docs written
  for no particular reader.
---

# Documentation

*Documentation is communication to a specific reader, not a record that work happened.*

## Goal

Ensure every documentation artifact is accurate, audience-aware, and maintained
as code evolves.

## Artifact Types

| Artifact | Audience | When Produced |
|----------|----------|---------------|
| README.md | New users, contributors, agents on first encounter | Project init; major capability changes |
| ARCHITECTURE.md | Contributors, agents understanding system structure | After grounding; significant structural changes |
| ADR | Future decision-makers (human and AI) | When a significant decision is made (MADR 4.0 format) |
| CHANGELOG.md | Users, operators, downstream consumers | Before landing user-visible changes (Keep a Changelog format) |
| CONTRIBUTING.md | New contributors, agents starting dev work | Project init; workflow changes |
| WORKFLOW.md | Contributors, agents executing project workflows | When workflow steps change; process additions or removals |
| API reference | API consumers, agents calling functions | During implementation, alongside code |
| Inline comments | Future maintainers, agents modifying code | At non-obvious decision points during implementation |

## Constraints

- `audience-first`: identify the reader before writing. No document exists
  without a stated audience.
- `code-is-ground-truth`: when docs and code disagree, investigate — do not
  assume either is correct. Code behavior is descriptive truth; docs are claims.
- `drift-is-debt`: stale documentation compounds. Each drifted doc trains
  readers to distrust all docs, making accurate docs worthless too.
- `minimum-viable-detail`: include enough to prevent mistakes, no more. Three
  clear sentences beat two verbose paragraphs.
- `same-pr`: documentation updates ship in the same PR as the code change that
  caused them, because deferred documentation updates become forgotten ones —
  drift is invisible until a reader hits it. If the update requires deeper work,
  create a tracking issue — never leave drift untracked.
- `source-of-truth-over-counts`: avoid hardcoded aggregate counts for dynamic
  sets (skills, endpoints, flags, supported providers) — they drift silently and
  mislead readers without any visible signal. Prefer referencing the
  authoritative object (manifest/config/schema) or generating the value.

## Requirements

- `task-oriented`: documents are organized around what the reader needs to
  accomplish, not around the codebase's file structure.
- `changelog-before-land`: user-visible changes include a CHANGELOG entry
  before landing, because consumers need to understand what changed without
  reading code or commits. Keep a Changelog categories (Added/Changed/
  Deprecated/Removed/Fixed/Security) work well because they map directly to
  how the change affects the consumer.
- `adr-for-decisions`: significant architectural decisions get an ADR.
  "Significant" means: affects contributor work, is hard to reverse, or is
  not obvious. MADR format (Context, Decision Drivers, Options, Outcome,
  Consequences) is a good default because each section forces the writer to
  articulate a different aspect of the decision — but the structure matters
  less than capturing the reasoning.
- `docs-in-acceptance-criteria`: user-facing changes include documentation
  updates as explicit acceptance criteria in the issue.

## Procedures

### audience-identify

Before writing any documentation:

1. Name the audience: end user, contributor, API consumer, or AI agent.
2. State what they already know (assumed context).
3. State what they need to accomplish after reading.
4. Apply the audience test throughout: "Would this reader know what to do
   after reading this?"

Audience profiles:
- **End user**: needs to install, configure, use. Assumes no internals knowledge.
- **Contributor**: needs architecture understanding and dev setup. Assumes
  programming competence but no project-specific knowledge.
- **API consumer**: needs behavior contracts and integration guidance. Assumes
  domain competence.
- **AI agent**: needs explicit file paths, concrete examples, constraint-first
  organization. Assumes no persistent memory across sessions.

### documentation-review

Fires after code changes, before `verification-before-completion`.

1. **Identify changed files.** Diff working tree or commit against base branch.
2. **Map changes to documentation.** For each changed file:
   - README — if the change affects setup, usage, or public API
   - ARCHITECTURE.md — if module boundaries, data flow, or structure changed
   - API reference — if public signatures, types, or behavior contracts changed
   - Inline doc comments — if function behavior changed
   - CHANGELOG — for any user-visible or API-visible change
   - ADRs — if the change implements or reverses a recorded decision
   - CONTRIBUTING — if build/test/dev workflow changed
   - WORKFLOW.md — if workflow steps, sequencing, or process changed
3. **Classify each mapped document:**
   - `accurate` — no update needed
   - `drifted` — claims no longer match code (update required)
   - `missing` — should exist but does not (creation required)
   - `obsolete` — references removed functionality (rewrite or delete)
4. **Update or track.** Update drifted/missing docs in the same PR. If deeper
   work is needed, create a tracking issue with `issue-craft`.
5. **Audit numeric claims.** Replace brittle counts with source-of-truth
   references, or explicitly verify and trace any remaining dynamic numbers.
6. **Apply audience test.** For each updated/created doc: "Would the intended
   reader know what to do after reading this?"
7. **Record coverage.** In the PR or commit, state: which docs were updated,
   which were verified accurate, which were flagged with tracking issues.

### write-artifact

1. Run `audience-identify`.
2. Use `ground` if the artifact requires design decisions (ARCHITECTURE, ADR).
3. Write for the identified audience at the appropriate depth.
4. Apply `minimum-viable-detail` — cut any section the reader does not need
   at the point of use.
5. Verify the audience test passes before committing.

### evaluate-existing-docs

When encountering existing documentation (e.g., onboarding to a project):

1. For each documentation file, classify as `accurate`, `drifted`, `missing`,
   or `obsolete` by comparing claims against actual code behavior.
2. Prioritize fixes: `missing` critical docs first, then `drifted`, then
   `obsolete`.
3. Create issues for each fix using `issue-craft`.

## Triggers

- after code changes that modify documented behavior
- at project initialization
- when architectural decisions are made
- when onboarding to an unfamiliar codebase
- when existing docs fail the audience test
- when `issue-craft` requires documentation acceptance criteria
- before `verification-before-completion`
- before `land` for user-visible changes

## Corruption Modes

- `structure-not-understanding`: document headings mirror the directory tree.
  Sentences begin with "This file..." not "To accomplish...". Restructure
  around tasks and concepts.
- `verbose-not-useful`: document is long but a reader cannot extract what they
  need. Ask: "Could this section be removed without reducing the reader's
  ability to accomplish their task?"
- `drift-tolerance`: documentation is known stale but updating feels expensive.
  Run `documentation-review`. Track what you cannot fix now.
- `audience-blindness`: no stated audience. Technical depth is inconsistent.
  The document would not change if the audience changed. Identify the reader
  before writing.
- `ceremony-over-substance`: docs updated to check a box. Changelog entries
  say "updated X" without explaining what changed. Ask: "Would a reader who
  missed this PR understand what changed from this doc update alone?"
- `ground-truth-confusion`: docs say one thing, code does another, agent
  believes the docs. Run the code. Observe behavior. Then decide which needs
  to change.
- `agent-outpacing-docs`: multiple PRs land per day, doc reviews deferred,
  drift compounds. Documentation review is part of completion, not a separate
  phase. If velocity makes full review impractical, reduce documentation scope
  to what can be maintained — less accurate docs beat more stale ones.

## Principles

- `documentation-is-communication`: docs exist to help a reader accomplish
  something, not to record that work happened.
- `less-accurate-beats-more-stale`: a small set of maintained docs is more
  valuable than a comprehensive set of drifted ones.
- `code-is-truth`: code behavior is the ground truth. Documentation is a
  model of that truth, subject to verification.

## Cross-References

- `ground`: Ground's Orient identifies who documentation serves and what it
  must enable — that output defines the documentation audience. Fires before
  writing ARCHITECTURE docs or ADRs; grounded constraints belong in
  ARCHITECTURE.md, significant decisions in ADRs.
- `bdd`: behavior contracts are the authoritative source for API documentation.
  Public behaviors should be reflected in user-facing docs, not only test files.
- `issue-craft`: user-facing changes include documentation expectations in
  acceptance criteria. This skill defines what that means: identify which
  artifacts need creation or update, and include those as criteria.
- `test-driven-development`: inline documentation (doc comments, type
  annotations) is written alongside code — doc comments are implementation
  work, not afterthought.
- `verification-before-completion`: documentation accuracy is completion
  evidence; `documentation-review` fires before verification.
- `land`: CHANGELOG entry required for user-visible changes. Documentation
  coverage is recorded alongside behavior coverage.
