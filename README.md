# groundwork

An opinionated development methodology for AI coding agents.

Most agent skill systems enforce workflow — do step 1, then step 2, then step 3.
Groundwork is different. It teaches agents *how to think* before they act.

The core insight: LLMs exaggerate human cognitive biases. They anchor on the first
information in context. They pattern-match from training data instead of reasoning
from requirements. They preserve complexity because removing it seems risky. Without
deliberate cognitive discipline, agents produce fluent, confident, wrong output.

Groundwork addresses this with skills organized across six phases of the development
lifecycle. Each skill is independently useful. Together, they form a complete
methodology.

## Phases

| Phase | What it governs | Skills |
|-------|----------------|--------|
| **Foundation** | How to think before acting | `ground` |
| **Specification** | Defining done before starting | `bdd` |
| **Decomposition** | Breaking goals into executable work | `planning` |
| **Execution** | How to build well | *planned* |
| **Verification** | How to know work is correct | *planned* |
| **Completion** | How work lands so others can build on it | `land` |

## Installation

Each skill is a self-contained `SKILL.md` file. Copy the ones you need into
your agent's skill directory:
```bash
# Claude Code
cp skills/foundation/ground/SKILL.md .claude/skills/ground.md

# OpenCode
cp skills/foundation/ground/SKILL.md .opencode/skill/ground/SKILL.md

# Or use loadout (https://github.com/pentaxis93/agent-skills)
```

## Philosophy

**Ground before you build.** Every skill in this system traces back to one
principle: establish what the work must enable before generating anything.
Agents that skip this step produce work shaped by their training data, not
by the actual need.

**Descriptive vs. normative.** What exists is not what's needed. The most
common agent failure is treating the current system as the definition of
what the system should do. Groundwork trains agents to distinguish between
the two.

**Work completes on landing.** Work isn't done when it's written. It's done
when the recipient can act on it without asking clarifying questions. A
brilliant artifact only its maker can understand is a wall, not a door.

## Current State

Groundwork is a personal experiment. Three of six phases have skills. The foundation,
specification, and completion phases are battle-tested from production use.
Decomposition is functional but evolving. Execution and verification are
planned but not yet written.

Contributions welcome — especially in the empty phases.

## License

Apache-2.0
