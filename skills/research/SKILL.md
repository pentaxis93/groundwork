---
name: research
description: Systematic multi-source research with citations and synthesis using 6-phase workflow and Tavily
license: MIT
metadata:
  version: "1.2.0"
  source: internal (adapted for eterne)
  updated: "2026-01-31"
  workflow: 6-phase (Clarify → Decompose → Gather → Evaluate → Resolve → Synthesize)
---

# Research Skill

This file is the methodology contract for systematic research. It is designed to stand alone: any agent using it should apply the workflow with the tools, permissions, and output format available in the current environment.

## Core Principles

### 1. Start Simple, Add Complexity When Needed
Begin with focused searches before expanding scope. Complex multi-agent research is only warranted when simpler approaches fail to produce adequate results.

### 2. Context Engineering is Critical
The quality of research depends on providing the right information, in the right format, at the right time. Structure queries precisely and validate sources systematically.

### 3. Three-Source Rule
Never trust a single source. Cross-reference with at least two independent sources before treating information as reliable.

### 4. Version Everything
Most source conflicts dissolve when versions/dates are explicit. Always anchor findings to specific versions and timestamps.

### 5. Empirical Verification Over Authority
When stakes are high, test claims directly rather than trusting even authoritative sources. Code behavior beats documentation. **Constraint-aware:** when the current environment is read-only or lacks shell access, gather the strongest available source-code evidence using the tools you do have, or explicitly note the limitation in the Confidence section rather than skipping the step silently.

---

## Research Workflow

### Phase 1: Clarification

Before researching, clarify the query:

1. **Identify the core question**: What specifically needs to be answered?
2. **Determine scope**: Breadth vs depth? Historical context needed?
3. **Specify output format**: Report, comparison table, decision recommendation?
4. **Note constraints**: Time period, specific technologies, geographic scope?

If the query is vague, ask clarifying questions before proceeding. Do not research a vague question — you'll waste cycles on wrong targets.

### Phase 2: Query Decomposition

Break the research question into 3-5 specific sub-questions that together form a complete picture:

```
Main: "Should we use PostgreSQL or MongoDB for this project?"

Sub-questions:
1. What are the data modeling requirements? (structured vs flexible)
2. What are the scaling requirements? (read-heavy, write-heavy, both)
3. What's the team's existing expertise?
4. What are the operational complexity tradeoffs?
5. What do production users of similar scale report?
```

Sub-questions should be independently searchable. Each should map to at least one source.

**Tavily integration:** Each sub-question becomes a separate `tavily-search` call. Design queries to be specific and independently searchable. When sub-questions are independent, fire them in parallel for speed.

### Phase 3: Source Gathering

Consult sources in this priority order, adapting to the domain:

| Source Type | Strengths | Best For | Tavily Strategy |
|-------------|-----------|----------|-----------------|
| **Official Docs** | Authoritative, maintained | API signatures, core concepts | `include_domains` targeting official site |
| **GitHub Issues/PRs** | Real problems, maintainer input | Edge cases, bugs, workarounds | `include_domains: ["github.com"]` |
| **Stack Overflow** | Curated answers, voting signal | Common problems, quick fixes | `include_domains: ["stackoverflow.com"]` |
| **Source Code** | Ground truth | When docs are unclear | Inspect the relevant source code with the tools available in the current environment |
| **Blog Posts** | Deep dives, tutorials | Learning workflows, context | General search, then `tavily-extract` |
| **Discord/Forums** | Cutting-edge, insider knowledge | Latest changes, community consensus | `include_domains` targeting community sites |

**For each source, record:**
- URL and access date
- Version/date of the information
- Author credibility indicators
- Key claims with direct quotes when significant

---

### Tool Strategy: Tavily

Tavily is the primary research engine. It has two tools with distinct roles:

#### `tavily-search` — Discovery

Use for finding sources across the web. Key parameters:

| Parameter | When to Use | Example |
|-----------|------------|---------|
| `search_depth: "basic"` | Default. Start here (Start Simple principle) | Initial sub-question sweep |
| `search_depth: "advanced"` | Basic returned thin results, or topic is niche | Deep technical questions, obscure APIs |
| `topic: "news"` | Current events, recent developments, breaking changes | `topic: "news", days: 7` for last week |
| `topic: "general"` | Default. Technical docs, tutorials, established knowledge | Most research |
| `time_range` | Version-sensitive research (anchors Principle 4) | `time_range: "month"` for recent only |
| `include_domains` | Target specific source types from the table above | `["github.com", "docs.rs"]` |
| `exclude_domains` | Filter known low-quality or irrelevant sites | `["w3schools.com", "geeksforgeeks.org"]` |
| `max_results` | Control breadth | 5 for focused, 15-20 for survey |
| `include_raw_content: true` | Need full text without a separate fetch | When result snippets look promising |

**Parallelization pattern:** Sub-questions from Phase 2 are independent. Fire multiple `tavily-search` calls simultaneously, each with different queries and domain filters:

```
# Parallel execution — all at once
Search 1: sub-question 1, include_domains: ["docs.python.org"]
Search 2: sub-question 2, include_domains: ["github.com"]  
Search 3: sub-question 3, search_depth: "advanced"
Search 4: sub-question 4, topic: "news", days: 30
```

**Escalation pattern:** Start Simple applies to search depth:
1. First pass: `search_depth: "basic"`, `max_results: 10`
2. If results are thin or off-target: `search_depth: "advanced"`, `max_results: 15`
3. If still inadequate: narrow with `include_domains` or broaden the query

#### `tavily-extract` — Deep Reading

Use for pulling full content from specific URLs. Key parameters:

| Parameter | When to Use | Example |
|-----------|------------|---------|
| `extract_depth: "basic"` | Default. Most web pages | Blog posts, docs, SO answers |
| `extract_depth: "advanced"` | LinkedIn, paywalled, or JS-heavy pages | Professional profiles, dynamic content |
| `urls` (batch) | Multiple sources to read in one call | Pass 3-5 URLs from search results |

**When to extract vs. when search is enough:**
- Search result snippets answer the question → don't extract, move on
- Snippet is promising but incomplete → extract that URL
- Need to verify a specific claim in context → extract the source
- Comparing multiple in-depth sources → batch extract all candidate URLs

**Batch extraction pattern:** After search surfaces promising URLs, extract them in a single call:

```
tavily-extract(urls: [url1, url2, url3, url4])
```

This is faster than sequential `webfetch` calls and returns cleaned content.

#### When to Use `webfetch` Instead

Tavily handles most cases, but fall back to `webfetch` when:
- You need a specific URL that Tavily extract can't parse (rare edge cases)
- The URL was provided directly by the user (not discovered via search)
- You need markdown-formatted output for readability

#### When to Use `google_search` Instead

Use `google_search` when:
- You need Google's specific ranking signals for a query
- Tavily results seem off or incomplete for a particular topic
- You want a second search engine's perspective for the Three-Source Rule

---

**Codebase grounding:** When the research topic intersects with the current project, inspect the relevant files and project artifacts before or during source gathering.

### Phase 4: Source Evaluation

Score each source on:

| Criterion | Question | Weight |
|-----------|----------|--------|
| **Relevance** | Does it directly address the question? | High |
| **Credibility** | Author expertise? Peer review? Maintainer? | High |
| **Currency** | How recent? Still applicable? | Medium-High |
| **Specificity** | Contains concrete data, examples, code? | Medium |
| **Consensus** | Does it align with other sources? | Medium |

**Trust hierarchy (highest to lowest):**
1. Source code behavior (empirical test)
2. Official docs with version tags
3. Maintainer statements in issues/PRs
4. Highly upvoted + recently active Stack Overflow
5. Blog posts with working code examples
6. Unverified forum posts

**Red flags:**
- No version/date mentioned
- Code examples without imports
- "This worked for me" with no context
- Confidently stated but lacks detail
- Circular references between sources

### Phase 5: Conflict Resolution

When sources disagree:

1. **Check versions**: Conflict often means different versions, not factual disagreement. Use `tavily-search` with `time_range` to find version-specific information
2. **Find the maintainer**: Their comment trumps community answers. Search `include_domains: ["github.com"]` for maintainer statements in issues/PRs
3. **Deep-read both sides**: Use `tavily-extract` on the conflicting URLs to get full context — snippets often make sources seem more contradictory than they are
4. **Test empirically**: Inspect the relevant source code directly when the environment allows it, or note the limitation
5. **Apply consensus weighting**: Run a broader search (`max_results: 20`) to gauge which position has more independent support
6. **Note the disagreement**: If unresolved, report both positions with evidence in the Conflicts section of output

### Phase 6: Synthesis

Synthesize findings into a self-contained report. At minimum, include the core answer, supporting evidence with citations, notable conflicts, confidence, and any material limitations. Key quality standards:

- Every factual claim has a citation
- Quantitative data preferred over qualitative assertions
- Multiple perspectives on controversial topics
- Conclusions are specific, not vague generalizations
- The Synthesis section gives a direct answer — no hedging with "it depends"
- The Confidence section honestly assesses source quality and consensus level
- Conflicts are surfaced, not hidden

---

## Capturing Practical Wisdom

Official docs miss the "clever hacks" and "power user moves." Find them by:

### Search Patterns

These patterns work with both `tavily-search` and `google_search`. For Tavily, combine the query text with `include_domains` for precision:

```
# GitHub issue archaeology
query: "[technology] workaround OR hack OR trick"
include_domains: ["github.com"]

# Maintainer statements
query: "[technology] [problem] fix OR resolved"
include_domains: ["github.com"]
search_depth: "advanced"

# Stack Overflow deep cuts
query: "[technology] [problem] note that OR also need to"
include_domains: ["stackoverflow.com"]

# Recent developments (last 30 days)
query: "[technology] [feature] announcement OR release"
time_range: "month"

# Breaking changes / migration
query: "[technology] breaking change OR migration guide"
topic: "news", days: 90

# Dotfile / config wisdom
query: "[tool] dotfiles OR awesome-[tool] config"
include_domains: ["github.com"]

# Conference talk wisdom
query: "[technology] conference talk transcript"
search_depth: "advanced"
```

**Follow-up pattern:** When a search surfaces promising URLs, batch them:
```
tavily-extract(urls: [promising_url_1, promising_url_2, promising_url_3])
```

### Emotional Language Signals

Hard-won knowledge often comes with emotional markers:
- "Finally!"
- "After hours of debugging..."
- "The trick is..."
- "What the docs don't tell you..."
- "I wish I knew this earlier..."

When you find such knowledge, preserve context:
```
## [Problem]
**Source**: [URL] ([date])
**Version**: [when this worked]
**The trick**: [solution]
**Why**: [explanation if known]
**Caveats**: [when this might break]
```

---

## Anti-Patterns to Avoid

| Anti-Pattern | Why It Fails | Instead |
|--------------|--------------|---------|
| **Single-source answers** | No verification | Cross-reference 3+ sources |
| **Skipping clarification** | Vague query -> poor results | Ask questions first |
| **Trusting first results** | Confirmation bias | Seek disconfirming evidence |
| **Copy-paste without understanding** | Cargo culting | Understand the "why" |
| **Ignoring version/date** | Outdated info applied incorrectly | Always note versions |
| **Vague conclusions** | "It depends" helps no one | Make a specific recommendation |
| **Hiding uncertainty** | Overconfidence misleads | State confidence levels |

---

## Quality Checklist

Before concluding research, verify:

- [ ] Core question answered directly and specifically
- [ ] 3+ diverse sources consulted
- [ ] All factual claims have citations with URLs
- [ ] Version/date noted for time-sensitive information
- [ ] Conflicting sources investigated and explained (not hidden)
- [ ] Quantitative data included where available
- [ ] Practical wisdom captured (not just official docs)
- [ ] Conclusion is concrete, not hedged
- [ ] Confidence level stated with reasoning
- [ ] Limitations and caveats acknowledged
- [ ] Output is self-contained and includes answer, evidence, confidence, and limitations

---

## Vault Integration

Research outputs stay in conversation by default. When the user wants to persist findings:

- **Actionable findings** -> tasks in relevant project/area files
- **Knowledge insights** -> permanent notes via `archive` skill
- **Reference material** -> `vault/03-resources/reference-notes/`
- **Full research report** -> `vault/archives/intelligence/research/research-YYYY-MM-DD-slug.md`

Persistence is the user's decision, not automatic.
