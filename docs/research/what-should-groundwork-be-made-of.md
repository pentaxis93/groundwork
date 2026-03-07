# Research Commission: What Should Groundwork Be Made Of?

**Date:** 2026-03-03
**Status:** Draft (saved from chat output)

---

## Ground (Applied To This Research Itself)

**Orient (what this must enable):** an evidence-backed decision about what *Groundwork* should contain (and what it should *not*), so AI coding agents reliably move from first encounter to shipped change to closed loop, while avoiding LLM failure modes (anchoring, premature generation, context loss, skipped verification, sloppy completion).

**Verified constraints (non-negotiable):**
- Distribution format: `sk`-style `SKILL.md` files.
- Composition: Superpowers owns execution layer (TDD, systematic debugging, verification-before-completion, subagent orchestration).
- `ground` stays and remains the distinctive core.
- Sovereignty: human owns **WHAT**, agent owns **HOW**.
- Single-developer context (not enterprise program/process scale).

**Assumptions (treated as optional until evidence supports them):**
- The current 6-phase structure is useful. (Actively consider alternatives.)
- "Methodology" should be skill-sized (invokable) and artifact-driven (produces/checks tangible outputs).

**Reconstruction (research method):**
1. Enumerate high-quality "how to work" skill collections (not framework-specific build skills).
2. Extract the best 15-30 *methodology* skills (problem solved, quality, mission relevance, adopt/adapt/learn).
3. Mine practitioner pain points and issue trackers for gaps (unmet needs).
4. Synthesize a Groundwork architecture where every skill earns its place by preventing a failure mode.

---

## D1: Landscape of Excellent Methodology Skills (15-30)

Below are **25** skills rated as strong "agentic software methodology" building blocks, drawn from the strongest collections found and audited locally.

### A) Intent, Spec, and Decision Quality (prevents: premature code, vague goals, spec drift)

1. **`superpowers:brainstorming`** (obra/superpowers)
Solves: "agents code before design is approved" with a hard gate plus design doc output.
Quality: hard-gates plus sequencing.
Mission fit: critical.
Recommendation: adopt as the pre-execution intake gate.

2. **`pm-skills:define-problem-statement`** (product-on-purpose/pm-skills)
Solves: "build the wrong thing" by forcing user segment, impact, success criteria.
Quality: crisp, reusable, tool-agnostic.
Mission fit: high.
Recommendation: adapt to single-dev software work (less "leadership", more "user + operator").

3. **`pm-skills:deliver-prd`**
Solves: "spec is missing scope/metrics/risks" via a PRD scaffold.
Quality: strong but can be heavy.
Mission fit: medium-high.
Recommendation: adapt into a "PRD-lite / Feature Spec" variant for solo dev.

4. **`pm-skills:deliver-edge-cases`**
Solves: "happy-path only" specs; drives test scenarios and error recovery paths.
Quality: excellent.
Mission fit: very high.
Recommendation: adopt with light terminology tweaks for software agents.

5. **`anthropics/skills:doc-coauthoring`**
Solves: "docs/specs don't work for readers" with staged coauthoring plus "reader testing" via fresh context.
Quality: excellent workflow design.
Mission fit: high (specs are the contract).
Recommendation: adapt (keep reader-testing concept, drop connector-specific bits).

6. **`adr-skill:adr-skill`** (skillrecordings/adr-skill)
Solves: ADRs as agent-executable specs (explicit implementation plan plus verification checklist plus Socratic gate).
Quality: standout; directly targets agent failure modes.
Mission fit: very high.
Recommendation: adapt (confirm licensing before adopting verbatim).

7. **`pm-skills:develop-design-rationale`**
Solves: "future agents lose the why" by capturing alternatives, evaluation criteria, tradeoffs.
Quality: strong.
Mission fit: high.
Recommendation: adopt/adapt depending on how ADR-heavy Groundwork becomes.

### B) Planning, Decomposition, and Work Control (prevents: over-scoped tasks, non-executable plans, lost progress)

8. **`superpowers:writing-plans`**
Solves: "plans that aren't runnable" with file-level steps, TDD granularity, verification hooks.
Quality: excellent, but ecosystem notes enforcement gaps around TDD structure.
Mission fit: high.
Recommendation: adopt, and add a companion "plan quality gate" skill.

9. **`superpowers:executing-plans`**
Solves: "agents skip steps" by forcing task-by-task execution.
Quality: excellent.
Mission fit: high.
Recommendation: adopt (execution layer stays in Superpowers).

10. **`groundwork:issue-craft`** (in this repo)
Solves: "issues aren't agent-executable" with strict acceptance-criteria and decomposition rules.
Quality: strong.
Mission fit: high.
Recommendation: keep; integrate prompt-injection hygiene (see D2).

11. **`kata-skills:kata-list-phase-assumptions`** (gannonh/kata-skills)
Solves: "hidden assumptions" by forcing the agent to surface what it thinks before planning.
Quality: very good and relatively rare.
Mission fit: high.
Recommendation: adapt into a small, tool-agnostic Groundwork skill.

12. **`kata-skills:kata-pause-work`** and **`kata-skills:kata-resume-work`**
Solves: "context resets / lost state" with explicit handoff artifacts and structured resumption.
Quality: strong (Kata is a whole-system, but the technique composes).
Mission fit: extremely high (LLM context is brittle).
Recommendation: adapt into Groundwork as the canonical checkpointing mechanism.

13. **`kata-skills:kata-map-codebase`**
Solves: "agents can't build a correct mental model of an existing repo" by parallel mapping plus durable artifacts.
Quality: strong; highly relevant to brownfield work.
Mission fit: high.
Recommendation: adapt (simplify; integrate with Groundwork repo conventions).

### C) Debugging, Verification, Review, and Closure (prevents: unverified claims, shallow reviews, incomplete shipping)

14. **`superpowers:systematic-debugging`**
Solves: "random-fix thrash" with a disciplined debugging protocol.
Quality: excellent.
Mission fit: essential.
Recommendation: adopt (Superpowers layer).

15. **`superpowers:verification-before-completion`**
Solves: "declares done without evidence" with explicit run-and-confirm gates.
Quality: excellent.
Mission fit: essential.
Recommendation: adopt.

16. **`superpowers:requesting-code-review`** and **`superpowers:receiving-code-review`**
Solves: "no structured review loop" and "performative compliance."
Quality: strong.
Mission fit: high.
Recommendation: adopt.

17. **`bmad-os-review-pr (Raven's Verdict)`** (bmadcode/BMAD-METHOD)
Solves: "polite reviews miss bugs" via adversarial internal pass plus tone transform plus posting gate.
Quality: unusually strong for PR review rigor.
Mission fit: high (single dev still benefits).
Recommendation: adapt the adversarial-then-professional pattern into Groundwork's review skill.

18. **`bmad-os-root-cause-analysis`**
Solves: "bugs recur because learnings aren't extracted" with an RCA template plus guardrail audit.
Quality: strong structure; but "name the individual" principle may not match culture.
Mission fit: medium-high.
Recommendation: adapt (keep guardrail analysis; make attribution/blame optional).

19. **`pm-skills:iterate-retrospective`**
Solves: "no learning loop" with a formal retro.
Quality: solid.
Mission fit: high.
Recommendation: adopt/adapt as lightweight single-dev retro.

20. **`pm-skills:deliver-release-notes`**
Solves: "changes shipped without narrative/communication" (also helps future agents).
Quality: solid.
Mission fit: medium-high.
Recommendation: adopt.

21. **`pm-skills:deliver-launch-checklist`**
Solves: "forgot the last 10%" (docs, configs, monitoring, rollout).
Quality: strong.
Mission fit: high.
Recommendation: adapt to "solo shipping checklist" (especially migrations, flags, rollback).

22. **`groundwork:land`** (in this repo)
Solves: "shipping without cleanup/closure" with a one-word closeout workflow.
Quality: strong but WeForge-specific.
Mission fit: high.
Recommendation: keep; consider forge-agnostic wrapper plus WeForge plugin variant.

### D) Security and Safety of Inputs (prevents: prompt injection, tool abuse, poisoned requirements)

23. **`developer-kit-claude-code:github-issue-workflow`**
Solves: "prompt injection via issue bodies/comments" by treating issue text as untrusted data and forcing human-confirmed requirements gates.
Quality: excellent; explicitly addresses a real agent failure mode.
Mission fit: very high.
Recommendation: adapt into a Groundwork "untrusted input isolation" skill that applies to issues, docs, PR comments, logs.

24. **`openai/skills:security-threat-model`** (OpenAI skill catalog)
Solves: threat modeling as a repeatable workflow.
Quality: good, but tool/ecosystem-specific.
Mission fit: medium-high (for agents making architectural changes).
Recommendation: learn from, or adapt into a lightweight "change threat check" gate.

### E) Meta: Maintaining the Skill Ecosystem Itself (prevents: skill rot, mismatched conventions)

25. **`superpowers:writing-skills`**
Solves: "skills that don't actually change behavior" by applying TDD-style RED/GREEN/REFACTOR to process docs.
Quality: excellent and aligned with Groundwork's mission.
Mission fit: high (Groundwork is a methodology library).
Recommendation: historical candidate only. Groundwork later adopted external `skill-creator` as the contributor-facing skill-authoring system instead of adding `writing-skills` to the runtime library.

---

## D2: Unmet Needs Analysis (gaps + evidence)

These are recurring problems power users report (and issue trackers corroborate) where existing skills are partial, fragmented, or too tool-specific.

1. **Persistent development memory management across tasks/sessions**
Evidence: Superpowers issues request "progress-tracking skill for development memory management" and "accumulate learnings across tasks and feed to subsequent subagents."
Gap: Most ecosystems assume uninterrupted context; real work needs checkpoints, summaries, and "what changed since last time" artifacts.
Best partial: Kata pause/resume plus roadmap state, but it's a whole-system.
Opportunity for Groundwork: a small, composable `context-snapshot` / `handoff` / `resume` trilogy.

2. **Spec to plan alignment verification (design doc overwritten or drifting)**
Evidence: Superpowers issues report plan/design drift and design doc overwrites.
Gap: Many skills can write a plan, few can audit it against a spec and detect divergence.
Opportunity: `spec-plan-consistency-check` skill (diff spec requirements vs plan steps; call out missing edge cases/tests/verification).

3. **Tool prerequisite plus environment verification as a first-class protocol**
Evidence: Superpowers RFC requests a "Prerequisites header for tool-dependent skills."
Gap: Skills frequently assume tools/configs exist; agents then improvise dangerously.
Opportunity: a standardized `prereq-check` micro-skill pattern (no side effects) that other skills can compose.

4. **Prompt injection and untrusted content handling outside GitHub issues**
Evidence: prompt injection is widely recognized as a practical risk, especially when LLMs can take actions.
Gap: skill ecosystems don't systematically treat repo files, issues, PR comments, and logs as potentially adversarial instructions.
Opportunity: universal `untrusted-input-isolation` skill.

5. **File list completeness for changes with infra/config side effects**
Evidence: BMAD issue reports incomplete file lists missing infrastructure side effects.
Gap: agents miss non-obvious collateral changes (CI, configs, migrations).
Opportunity: `change-surface-audit` skill: enumerate touched artifacts plus operational impacts plus rollback plan.

6. **A disciplined ask-for-help / escalation protocol**
Evidence: Superpowers has an open request for "Skill for requesting help from human partners."
Gap: agents either stall silently or plow ahead; few have a structured escalation format.
Opportunity: `human-escalation-brief` skill: concise decision-ready questions plus what has been tried plus risk of proceeding.

7. **Context-window overflow handling is still mostly ad hoc**
Evidence: academic/practitioner work exists on pruning/retention and overflow failure modes; day-to-day practice is still improvised.
Gap: skill ecosystems rarely operationalize "what to keep, what to summarize, what to externalize."
Opportunity: Groundwork's memory/handoff skills should explicitly manage context budgets and durable artifacts.

---

## D3: Proposed Groundwork Architecture (what it should contain)

### Recommendation: keep the 6-phase hypothesis, but redefine phases around failure-mode gates and durable artifacts

Why: ecosystems converge on a staged workflow (intent -> plan -> execute -> verify -> close), but the missing piece is durable memory plus safety gates. Superpowers already owns the execution middle; Groundwork should be the bookends plus memory plus safety glue.

### Proposed Table of Contents (skills + adopt/adapt/build)

**1) Foundation (Before any solutioning)**
- `ground` (build/keep; unique). Failure mode: inherited framing, anchoring, category inheritance.
- `untrusted-input-isolation` (adapt from GitHub issue workflow patterns). Failure mode: prompt injection, poisoned requirements.
- `map-codebase` (adapt from Kata). Failure mode: hallucinated architecture, wrong file targeting.
- `context-snapshot` plus `pause-work` plus `resume-work` (adapt from Kata). Failure mode: lost context across sessions, repeated work, regressions.

**2) Specification (Define WHAT, test the spec as an artifact)**
- `problem-statement` (adopt/adapt from pm-skills). Failure mode: building the wrong thing.
- `edge-cases` (adopt from pm-skills). Failure mode: happy-path-only implementations.
- `doc-coauthoring` (adapt from Anthropic). Failure mode: specs/docs that do not work for readers.
- `adr` (adapt from adr-skill; confirm licensing before adopting verbatim). Failure mode: architectural decisions not captured as executable constraints.

**3) Decomposition (Make work executable without clarification)**
- `issue-craft` (keep). Failure mode: non-executable tasks, non-verifiable acceptance criteria.
- `spec-to-plan` (adapt from spec-to-plan patterns; tool-agnostic). Failure mode: plans that do not encode requirements/constraints.
- `spec-plan-consistency-check` (build). Failure mode: drift between approved design/spec and the implementation plan.
- `assumptions-surface` (adapt from Kata assumptions surfacing). Failure mode: hidden agent assumptions becoming unreviewed requirements.

**4) Execution (Explicitly delegated to Superpowers)**
- `execution-router` (build, minimal). Failure mode: agents do not invoke Superpowers at the right times.

**5) Verification (Beyond unit tests: does the system work?)**
- Keep `bdd` only if behavior-spec naming is desired; otherwise defer to Superpowers TDD.
- `uat-check` (adapt from Kata verification/milestone audit). Failure mode: "tests pass" but user workflow is broken.
- `review-protocol` (adapt Raven's Verdict pattern plus Superpowers review skills). Failure mode: shallow reviews, missed edge cases, tone issues.

**6) Completion (Close the loop, preserve intent, delete entropy)**
- `land` (keep; plus forge-agnostic variant). Failure mode: shipping without closure, branch rot, issues left open.
- `release-notes` plus `launch-checklist` plus `retro` (adopt/adapt from pm-skills). Failure mode: undocumented changes, missing last-mile steps, no learning loop.
- `root-cause-analysis` (adapt from BMAD RCA). Failure mode: recurring bugs due to missing guardrail analysis.

### Decision gate outcomes (as of 2026-03-03)
- **G1 (curated collection plus only original ground):** not quite; Groundwork likely needs a few original glue skills: `spec-plan-consistency-check`, `execution-router`, and a tool-agnostic `context-snapshot/pause/resume` package if Kata is not adopted wholesale.
- **G2 (6 phases survive):** yes, if phases are treated as gates plus artifacts, not a fixed taxonomy.
- **G3 (better organizing principle):** strongest alternative is artifact-centric: the "Project Packet" (Problem, Spec, Plan, Decisions, Logs, Release, Lessons) with skills that create/audit each artifact.

---

## Sources (URLs)

```text
Superpowers: https://github.com/obra/superpowers
Superpowers issues: https://github.com/obra/superpowers/issues

BMAD Method: https://github.com/bmadcode/BMAD-METHOD
BMAD issues: https://github.com/bmadcode/BMAD-METHOD/issues

PM Skills: https://github.com/product-on-purpose/pm-skills
ADR Skill: https://github.com/skillrecordings/adr-skill
Kata Skills: https://github.com/gannonh/kata-skills

OpenAI Skills: https://github.com/openai/skills
Anthropic Skills: https://github.com/anthropics/skills

Prompt injection (background):
https://simonwillison.net/series/prompt-injection/
```
