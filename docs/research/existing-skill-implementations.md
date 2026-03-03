# Existing Skill Implementations: Follow-Up Research

**Date:** 2026-03-03
**Predecessor:** Groundwork Landscape Analysis (2026-03-03)
**Status:** Complete

---

## D1: Skill-by-Skill Comparison Table

### Rating Key

- **Adopt:** High-quality equivalent exists. Use it instead of building.
- **Adapt:** Partial equivalent exists. Adaptation < building from scratch.
- **Build:** No equivalent exists, or existing options are philosophically
  incompatible. Build from scratch.
- **Keep:** Already built and no better alternative found.

### Comparison

| Groundwork Skill | Best Existing Equivalent | Quality | Recommendation |
|---|---|---|---|
| **ground** (cognitive discipline) | None. Three "first-principles" prompt templates exist (majesticlabs, davisbuilds, escotilha) but these are generic question-scaffolding, not cognitive discipline. No system addresses the descriptive/normative distinction, assumed-constraint patterns, orient-first protocol, or corruption modes. Superpowers' rationalization tables are process-compliance guardrails ("are you following the workflow?"), not cognitive discipline ("are you thinking about the problem correctly?"). | N/A | **Keep. Ground is unique.** Verified across 64,000+ indexed skills, 127+ repos on the `agent-skills` GitHub topic, five methodology frameworks (Superpowers, BMAD, spec-kit, OpenSpec, PromptX), and academic debiasing literature. Nothing occupies this category. |
| **bdd** (behavioral specification) | No standalone BDD-as-methodology skill exists. Superpowers has TDD only (no BDD, no Given/When/Then). spec-kit uses Given/When/Then in product specifications but is coupled to its CLI. OpenSpec uses Given/When/Then in behavior contracts but is coupled to its CLI. BMAD's QA agent generates tests from existing code, does not specify behavior. | N/A | **Keep.** BDD test-writing skills exist in the ecosystem but none frame BDD as a specification methodology that drives implementation priority. Our approach (behavior specification as contract, not test boilerplate) has no equivalent. |
| **planning** (session planning) | No equivalent. Superpowers' `writing-plans` produces task-level implementation plans (2-5 min granularity with exact code). BMAD has multi-agent workflow sequencing. Neither addresses session-level work selection from an issue graph with WSJF-lite scoring. | N/A | **Keep.** Our planning operates at a different granularity (session-sized increments from issue graph) than Superpowers' writing-plans (task-sized increments from design docs). These are complementary, not competing. |
| **issue-craft** (issue lifecycle) | No equivalent. BMAD's `bmad-os-gh-triage` is operational automation (fetches issues, dispatches sub-agents for batch analysis). spec-kit's `taskstoissues` is mechanical conversion (task checklist to GitHub issues). Neither teaches agents how to write agent-executable issues. | N/A | **Keep.** The concept of crafting issues specifically for autonomous agent execution -- with binary acceptance criteria, session-sized scope, and vertical-slice bias -- has no published equivalent. |
| **land** (completion) | Superpowers' `finishing-a-development-branch` is the closest. It presents 4 options (merge/PR/keep/discard), verifies tests, and cleans up worktrees. However, it has **no issue lifecycle integration** -- no issue commenting, no issue closing, no PR discovery. netresearch/git-workflow-skill covers branching and conventional commits but not issue closure. | Partial | **Keep.** Our land is more complete for forge-native workflows (merge + push + branch cleanup + issue comment + issue close + verification). Superpowers' skill is interactive and git-focused; ours is deterministic and forge-integrated. |
| **research** (structured research) | No standalone research methodology skill found in any registry. BMAD has three research workflows (market, domain, technical) but they are persona-driven multi-step workflows, not standalone cognitive skills. Superpowers has no research skill. | N/A | **Keep.** Among the most thorough research methodology skills available. The 6-phase workflow with Tavily integration, source evaluation hierarchy, and conflict resolution protocol has no published equivalent. |

### Summary

**All six Groundwork skills should be kept.** No high-quality equivalent was found for any of them. The decision gates resolve as:

- **G1 (adopt):** Does not trigger. No skill has a high-quality equivalent.
- **G2 (adapt):** Does not trigger. No partial equivalent is close enough that adaptation < building.
- **G3 (import Superpowers):** Triggers for Superpowers execution-phase skills (see D2), but not as replacements for our existing skills.

---

## D2: Superpowers Import List

Superpowers (obra/superpowers) contains 14 skills. Assessment of each
relative to Groundwork's methodology:

### Use As-Is (Import for Execution Phase)

| Superpowers Skill | What It Does | Why Import |
|---|---|---|
| **test-driven-development** | Enforces strict RED-GREEN-REFACTOR with anti-rationalization tables. "Write code before test? Delete it." 11-entry rationalization prevention table. | Groundwork has no TDD skill. This is battle-tested (69K stars, 286 commits of refinement). Use as the implementation discipline during Execution phase. Complementary to our BDD: we specify behavior, Superpowers enforces TDD during implementation. |
| **systematic-debugging** | 4-phase root cause process: Investigation, Pattern Analysis, Hypothesis & Testing, Implementation. 3 supporting technique docs (root-cause-tracing, defense-in-depth, condition-based-waiting). | Groundwork has no debugging methodology. This is comprehensive and well-structured. Import directly. |
| **verification-before-completion** | Requires running verification commands and confirming output before completion claims. 5-step gate function. Addresses "should/probably/seems to" language as red flags. | Groundwork has no verification-at-task-level skill. This is a point skill that could compose with a broader Verification phase methodology we build. |
| **subagent-driven-development** | Same-session plan execution with fresh subagent per task and two-stage review (spec compliance + code quality). | Groundwork has no subagent orchestration. This is the core autonomous execution engine in Superpowers. |

### Study for Inspiration (Don't Import Directly)

| Superpowers Skill | What It Does | What to Learn |
|---|---|---|
| **brainstorming** | Socratic design refinement: one-question-at-a-time dialogue, 2-3 approaches with tradeoffs, design doc before any implementation. | The hard gate on implementation ("Do NOT invoke any implementation skill... until you have presented a design and the user has approved it") is a strong pattern. Our `ground` skill could reference this approach for design phases. |
| **writing-skills** (meta-skill) | TDD applied to skill creation: RED-GREEN-REFACTOR cycle using pressure scenarios on subagents. Includes Cialdini's persuasion principles adapted for LLM compliance. | The pressure-testing methodology for skills and the persuasion-principles document are genuinely valuable for our own skill development. The Meincke et al. (2025) finding that persuasion techniques doubled LLM compliance (33% to 72%) is worth incorporating. |
| **writing-plans** | Converts designs into bite-sized tasks (2-5 min, exact file paths, complete code). | The granularity model (2-5 min tasks with exact code) is useful for understanding how our issue-level decomposition hands off to task-level decomposition. |
| **requesting-code-review** / **receiving-code-review** | Dispatches code-reviewer subagent; handles review feedback with explicit ban on performative agreement ("You're absolutely right!" is forbidden). | The anti-performative-agreement stance and the push-back-when-wrong protocol are worth incorporating into any Groundwork verification skill. |

### Irrelevant to Our Workflow

| Superpowers Skill | Why Skip |
|---|---|
| **executing-plans** | Batch execution for separate sessions. Our workflow uses single-session execution. |
| **dispatching-parallel-agents** | Parallel debugging dispatch. Niche use case. |
| **using-git-worktrees** | We use standard branching, not worktrees. |
| **finishing-a-development-branch** | Our `land` is more complete for forge-native workflows. |
| **using-superpowers** | Bootstrap/skill-discovery skill specific to the Superpowers framework. |

### Composition Architecture

```
Groundwork                          Superpowers
─────────                           ───────────
ground (think correctly)
  ↓
bdd (specify behavior)
  ↓
planning (select session work)
  ↓
issue-craft (write executable issues)
  ↓                                 brainstorming (refine design)
                                      ↓
                                    writing-plans (decompose to tasks)
                                      ↓
                                    subagent-driven-development (execute)
                                      ↓
                                    test-driven-development (enforce TDD)
                                      ↓
                                    systematic-debugging (fix issues)
                                      ↓
                                    verification-before-completion (prove done)
                                      ↓
                                    requesting-code-review (review)
  ↓
verify (against behavioral spec)    ← composes with Superpowers verification
  ↓
land (merge, close, ship)
```

Groundwork owns the bookends. Superpowers owns the middle.

---

## D3: Ecosystem Gems

Skills from the broader ecosystem worth adopting or referencing,
organized by relevance.

### Tier 1: Strong Candidates for Adoption

| Skill | Source | What It Does | Recommendation |
|---|---|---|---|
| **ADR skill** | skillrecordings/adr-skill | Four-phase ADR workflow: Scan existing ADRs, Capture intent via Socratic questioning, Draft with template (simple or MADR 4.0), Review with checklist. Includes shell scripts for creation, status changes, and bootstrapping. | **Adapt.** Well-structured and compatible with Groundwork's philosophy. Could become a Groundwork skill for the Specification or Execution phase. The Socratic questioning approach aligns with ground's orient-first protocol. |
| **ADR skill (lighter)** | product-on-purpose/pm-skills `develop-adr` | Nygard-format ADR with context/decision/consequences. Apache 2.0 licensed. Part of a 24-skill PM collection. | **Reference.** Lighter alternative to skillrecordings. Use as a template for simpler ADR needs. |
| **Code review (6-pass)** | Cygnusfear/claude-stuff | Six-pass methodology: technical, consistency, architecture, environment, verification, synthesis. | **Study.** The multi-pass structure could inform a Groundwork verification skill. The separation of concerns (architecture review vs. code quality) maps to different verification questions. |

### Tier 2: Worth Referencing

| Skill | Source | What It Does | Recommendation |
|---|---|---|---|
| **Release notes** | product-on-purpose/pm-skills `deliver-release-notes` | Translates changelog to user-facing benefit language. Apache 2.0. | **Reference** for completion phase. The benefit-translation framing ("what users can now do" vs. "what changed technically") is a useful discipline. |
| **Retrospective** | product-on-purpose/pm-skills `iterate-retrospective` | Structured reflection: what went well, what to improve, action items. Multiple formats (Start/Stop/Continue, 4Ls, Sailboat). | **Reference** for a potential reflection skill. |
| **Git workflow** | netresearch/git-workflow-skill | Comprehensive git workflow: branching, conventional commits, PR processes, CI/CD. Includes verification script. | **Reference** for enhancing our `land` skill. The commit conventions and "Tidy First" separation (structural vs. behavioral commits) are worth incorporating. |
| **Commit discipline** | kaneshin/incubator | Enforces clean commit standards: single logical unit, passing tests, Tidy First separation of structural from behavioral changes. | **Reference** for completion phase. The "Tidy First" concept maps well to Groundwork's philosophy. |
| **Raven's Verdict** | BMAD `bmad-os-review-pr` | Adversarial PR review: cynical internal analysis transformed to professional tone. Two-phase approach (think harshly, speak constructively). | **Study.** The two-phase pattern (internal severity assessment -> external professional delivery) is a clever prompt engineering technique for verification skills. |
| **Root Cause Analysis** | BMAD `bmad-os-root-cause-analysis` | 5 Whys, guardrail evaluation, timeline construction, direct attribution. | **Reference** for verification phase. Informative but operational (runs commands) rather than cognitive. |

### Tier 3: Interesting but Out of Scope

| Skill | Source | Notes |
|---|---|---|
| **cognitive-toolworks** | williamzujkowski | 80+ skills with context budget management (discovery <=8k tokens, execution <=20k). Sophisticated but enterprise-scale. |
| **levnikolaevich/claude-code-skills** | GitHub | 100+ pipeline skills. Maximalist approach. Interesting auditor skills (ln-620 through ln-654) for verification reference. |
| **Hegelian Dialectic** | KyleAMathews | Spawns two sub-agents to argue committed positions, synthesizes via Aufhebung. Creative thinking amplification. |
| **product-on-purpose/pm-skills** (full set) | GitHub | 24 PM skills covering discovery through iteration. Good reference for how to structure a skill collection. Apache 2.0. |

### What's NOT in the Ecosystem (Gaps Groundwork Should Fill)

The ecosystem has 64,000+ skills but is missing:

1. **Verification methodology** -- No unified "does the work meet the spec?" phase skill
2. **Delta specification** -- No skill for specifying changes to existing behavior (OpenSpec concept, not distributable)
3. **Issue crafting for agents** -- No skill teaching how to write agent-executable issues
4. **Session planning** -- No issue-graph-first work selection discipline
5. **Cognitive discipline** -- No agent self-debiasing during design work

---

## D4: sk Publishing Guide

### Current State

Groundwork is already sk-compatible. The `agents.toml` at repo root and
the `skills/` directory structure with `SKILL.md` files make it a valid
sk subdirectory package.

### Minimum Viable sk Package

The absolute minimum is a single `SKILL.md` file at repo root with YAML
frontmatter including a `name` field:

```markdown
---
name: my-skill
description: What this skill does
---

# My Skill

[skill content]
```

That's it. One file, one field.

### Multi-Skill Package (What We Have)

For multiple skills, use subdirectories:

```
groundwork/
  agents.toml          # optional: enables exports control
  skills/
    foundation/
      ground/
        SKILL.md
      research/
        SKILL.md
    specification/
      bdd/
        SKILL.md
    decomposition/
      planning/
        SKILL.md
      issue-craft/
        SKILL.md
    completion/
      land/
        SKILL.md
```

sk detects this as a "subdirectory package" -- it finds all directories
containing `SKILL.md` files.

### Consumer agents.toml

A consumer adds Groundwork like this:

```toml
[agents]
claude-code = true
codex = true

[dependencies]
groundwork = { gh = "pentaxis93/groundwork" }
```

For individual phases:

```toml
[dependencies]
foundation = { gh = "pentaxis93/groundwork", path = "skills/foundation" }
decomposition = { gh = "pentaxis93/groundwork", path = "skills/decomposition" }
```

Then `sk sync` installs skills into the agent's skill directory.

### Cross-Platform Consumption

sk installs SKILL.md files into each agent's native skill directory:
- Claude Code: `.claude/skills/`
- Codex: `.codex/skills/` or equivalent
- Amp, OpenCode, Factory: their respective skill dirs

A Superpowers user on Claude Code already has skills in `.claude/skills/`.
sk populates the same directory. Both systems consume the same SKILL.md
format. No conflict -- an agent can load both Groundwork and Superpowers
skills simultaneously.

### Package Manifest (Optional)

For more control, add to `agents.toml`:

```toml
[package]
name = "groundwork"
version = "1.0.0"

[exports.auto_discover]
skills = "./skills"
```

### Naming Conventions

- Skill names: lowercase with hyphens (`ground`, `issue-craft`)
- Package names: lowercase, single word or hyphenated (`groundwork`)
- Registry format: `@owner/skill-name` on agentskill.sh
- GitHub format: `owner/repo` for sk

### Publishing Steps

1. Ensure each skill has `SKILL.md` with YAML frontmatter (`name` required)
2. Push to GitHub (already done: `pentaxis93/groundwork`)
3. Consumers add to their `agents.toml` and run `sk sync`
4. Optionally submit to skills.sh / agentskill.sh for discovery

No build step. No npm publish. No compilation. sk reads SKILL.md files
directly from git.

### Examples of Standalone Methodology Skills via sk

Methodology skills published via sk are rare. The ecosystem is dominated
by code-generation skills. The closest examples:

- **obra/superpowers** -- Distributed via Claude Code plugin marketplace
  AND installable via sk. 14 methodology skills.
- **product-on-purpose/pm-skills** -- 24 PM methodology skills.
  Not published via sk but sk-compatible structure.
- **pentaxis93/groundwork** -- Us. Among the first pure-methodology
  packages designed for sk distribution.

---

## Decision Gate Results

### G1: For any skill where a high-quality equivalent exists -> Adopt it.

**Result: No triggers.** No existing skill is a high-quality equivalent
to any Groundwork skill.

### G2: For any skill where a partial equivalent exists -> Evaluate
adaptation.

**Result: One candidate.** The skillrecordings/adr-skill is worth
adapting as a new Groundwork skill for the Specification or Execution
phase. This would be a new addition, not a replacement.

### G3: For Superpowers skills that cover our gaps -> Import directly.

**Result: Four skills to import.** `test-driven-development`,
`systematic-debugging`, `verification-before-completion`, and
`subagent-driven-development` should be referenced as the recommended
execution layer. These cover Groundwork's empty Execution phase.

---

## Sources

### Primary (Repository Analysis)
- obra/superpowers: All 14 skill files, supporting docs, agent definitions
- bmad-code-org/BMAD-METHOD: 8 Claude Code skills, agent YAML definitions, workflow files
- github/spec-kit: Command templates, constitution template, spec template
- Fission-AI/OpenSpec: README, proposal format, delta spec structure
- Deepractice/PromptX: Role definitions, thought files, DPML protocol
- miltonian/principles: TypeScript agent pipeline source

### Secondary (Registry/Ecosystem Search)
- agentskill.sh: 64,000+ indexed skills searched
- skills.sh: Registry and leaderboard analysis
- awesome-skills aggregators: heilcheng (2.6K stars), VoltAgent, skillmatic-ai, CommandCodeAI, gmh5225, kodustech, sickn33 (968+ skills)
- GitHub topic search: `agent-skills` (127+ repos), `ai-skills`, `claude-skills`
- LobeHub skill marketplace
- OpenAI skills catalog (openai/skills): .system and .curated directories
- Anthropic skills (anthropics/skills): 16 skills

### Tertiary (Verification of Uniqueness Claim)
- majesticlabs-dev/majestic-marketplace: first-principles skill
- davisbuilds dojo-first-principles: LobeHub listing
- escotilha claude-first-principles: LobeHub listing
- digital-stoic-org agent-skills-challenge: Challenge skill (404, likely removed)
- flpbalada/my-opencode-config: cognitive-biases skill
- KyleAMathews Hegelian Dialectic skill
- kylesnowschwartz sc-think skill
- arxiv 2504.04141 (SACD), arxiv 2510.19973 (6G cognitive biases)
- Alignment Forum: "Human-like metacognitive skills will reduce LLM slop"
- williamzujkowski/cognitive-toolworks: 80+ skills
- levnikolaevich/claude-code-skills: 100+ pipeline skills
- skillrecordings/adr-skill: ADR methodology
- product-on-purpose/pm-skills: 24 PM skills
- netresearch/git-workflow-skill: Git workflow
- kaneshin/incubator: Commit discipline
- 803/skills-supply: sk documentation and package format
