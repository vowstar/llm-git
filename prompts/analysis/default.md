Analyze git changes and classify as conventional commit with detail points.

═══════════════════════════════════════════════════════════════════════════════
SECTION 1: CONTEXT
═══════════════════════════════════════════════════════════════════════════════

OVERVIEW OF CHANGES:

```
{{ stat }}
```

SCOPE SUGGESTIONS (derived from changed files, line-count weighted): {{ scope_candidates }}
{% if recent_commits %}

PROJECT COMMIT STYLE (last 10 commits for consistency):

```
{{ recent_commits }}
```

Use the above commits as few-shot examples to match this project's commit style and conventions.
{% endif %}
{% if common_scopes %}

COMMON SCOPES FROM HISTORY (with frequency): {{ common_scopes }}

CRITICAL SCOPE SELECTION RULE:
- PREFER existing scopes from history over new scopes when applicable
- Only introduce new scopes if the change clearly targets a new component not in history
- When a change fits an existing scope (even partially), use that scope for consistency
- Example: If history shows "api (15)" and change touches src/api/, use "api" not "api/new-feature"
{% endif %}

DETAILED DIFF:

```diff
{{ diff }}
```

═══════════════════════════════════════════════════════════════════════════════
SECTION 2: DECISION TREE
═══════════════════════════════════════════════════════════════════════════════

TYPE CLASSIFICATION FLOWCHART:

START: Does this change add NEW public API surface?
├─ YES → feat
│  Examples: new functions/methods, new CLI flags, new endpoints, new exports
│  Diff indicators: "pub fn", "pub struct", "pub enum", "export function"
│
└─ NO ──→ Does this enable NEW user-observable capability/behavior?
   ├─ YES → feat
   │  Examples: protocol changes (HTTP→gRPC), async migration, event model
   │  Diff indicators: architectural shifts, new dependencies for features
   │
   └─ NO ──→ Does this fix incorrect behavior?
      ├─ YES → fix
      │  Examples: bugs, crashes, wrong outputs, race conditions
      │  Diff indicators: "unwrap()" → "?", bounds checks, error handling
      │
      └─ NO ──→ Does this change ONLY internal structure (provably same behavior)?
         ├─ YES → refactor
         │  Examples: renames, module reorg, extract functions, consolidate
         │  Requirement: PROOF of unchanged behavior (same tests, same API)
         │
         └─ NO ──→ Apply OTHER HEURISTICS (below)

OTHER HEURISTICS (ordered by precedence):

1. Commit msg starts "Revert" → revert
2. ONLY .md files / doc comments → docs
3. ONLY test files → test
4. ONLY lock files / .gitignore / config → chore
5. ONLY whitespace / formatting → style
6. Proven perf improvement (benchmarks) → perf
7. Build scripts / Cargo.toml / package.json → build
8. .github/workflows / CI configs → ci

CRITICAL: feat vs refactor decision

- feat: ANY observable behavior change OR new public API
  ✓ "Migrated HTTP client to async" → feat (behavior: now async)
  ✓ "Added pub fn process_batch()" → feat (new API)
  ✓ "Replaced polling with event-driven model" → feat (observable change)

- refactor: ONLY when provably unchanged
  ✓ "Renamed internal module structure" → refactor (no API/behavior change)
  ✓ "Extracted helper functions" → refactor (same outputs)
  ✗ "Migrated to gRPC" → feat (protocol = behavior change)

SCOPE EXTRACTION (>60% threshold examples):

- Use scope when: ≥60% of line changes in single dir/component
  Example: 150 lines in src/api/, 30 in src/lib.rs → scope: api
  Example: 80 lines in src/core/utime.rs, 20 elsewhere → scope: core/utime
  Example: 50 in src/api/, 50 in src/types/ → NO scope (50% each)

- Omit scope when: multi-component, cross-cutting, unclear focus
- Format: lowercase, max 2 segments, use / - \_ only
- Special scopes (even if not suggested): toolchain, deps, config

WHEN TO OMIT SCOPE (prefer null over unhelpful values):
- Multi-component or cross-cutting changes
- No clear dominant component (e.g., 50/50 split)
- Project-wide refactoring
- Entire codebase changes
- ALWAYS prefer null over generic/unhelpful scopes

FORBIDDEN SCOPES (NEVER use these):
- src, lib, include, tests, test, benches, examples, docs
- Project name or repository name
- Generic terms: app, main, core (unless core is a specific module)
- Overly broad: entire, all, everything, general, misc
- If only these would apply, use null instead

═══════════════════════════════════════════════════════════════════════════════
SECTION 3: OUTPUT SCHEMA
═══════════════════════════════════════════════════════════════════════════════

You will call function `create_change_analysis` with this JSON structure:

{
"commit_type": "feat|fix|refactor|docs|test|chore|style|perf|build|ci|revert",
"scope": "optional-scope" or null,
"details": [
"Added retry logic with exponential backoff for transient failures (#123, #124).",
"Migrated 8 modules to unified error type for consistency (#125-#132)."
],
"issues": []
}

DETAIL REQUIREMENTS (0-6 items, prefer 3-4):

1. MUST start with past-tense verb from ALLOWED LIST:
   added, fixed, updated, refactored, removed, replaced, improved, implemented,
   migrated, renamed, moved, merged, split, extracted, restructured, reorganized,
   consolidated, simplified, optimized, documented, tested, changed, introduced,
   deprecated, deleted, corrected, enhanced, reverted

2. MUST end with period

3. INLINE ISSUE REFERENCES:
   - If specific issues relate to a detail item, append them in parentheses BEFORE the period
   - Format: (#123), (#123, #456), or (#123-#125) for consecutive ranges
   - Group consecutive numbers as ranges: (#518-#525) not (#518, #519, #520...)
   - Leave issues array empty [] (footers are obsolete)
   - Example: "Implemented table.init instruction (#518, #519)."
   - Example: "Added typed select support (#523-#525)."

4. CRITICAL - Explain WHY, not just WHAT:
   - For each detail, explain the motivation/reasoning behind the change
   - Answer: Why was this change necessary? What problem does it solve?
   - Include context that future developers need to understand the decision
   ✓ "Added retry logic with exponential backoff to handle transient network failures (#123)."
   ✓ "Migrated to async I/O to eliminate blocking operations under high load (#456)."
   ✓ "Consolidated three HTTP builders into unified API to reduce maintenance burden (#123-#125)."
   ✗ "Added retry logic." (states WHAT but not WHY)
   ✗ "Updated HTTP client." (vague, no motivation)
   ✗ "Refactored error handling." (describes action but not purpose)

   NEGATIVE CONSTRAINT:
   - Do NOT generate details that merely restate the diff (e.g., "Changed X to Y")
   - If the motivation is not visible in the diff or context, use general purpose statements:
     ✓ "Updated logic for correctness." (when specific reason unclear)
     ✓ "Refactored for consistency." (when restructuring without clear external reason)
   - NEVER guess or fabricate motivations not supported by the diff or context

5. Abstraction levels (prefer high):
   Level 3 (BEST): Architectural, user-facing, performance
   "Replaced polling with event-driven model for 10x throughput (#456)."
   Level 2 (GOOD): Component/API changes
   "Consolidated three HTTP builders into unified API (#123-#125)."
   Level 1 (AVOID): Low-level details
   "Renamed workspacePath to locate." ❌

6. Group ≥3 similar: "Updated 5 test files for new API (#200-#204)." NOT 5 bullets

7. Priority order: user-visible > perf/security > architecture > internal

8. Empty array [] if no supporting details needed

EXCLUDE FROM DETAILS:

- Imports/use statements
- Whitespace/formatting/indentation
- Trivial renames (unless part of larger API change)
- Debug prints/temp logging
- Comment changes (unless substantial docs)
- File moves without modification
- Single-line tweaks/typo fixes
- Internal impl details invisible to users

WRITING RULES:

- Plain sentences (bullets/numbering added during formatting)
- Short, direct (≤120 chars per detail)
- Precise nouns (module/file/API names)
- Include why/how when meaningful
- NO meta phrases: "This commit", "Updated code", etc

JSON SCHEMA EXAMPLES:

Example 1 - Feature with new API:
{
"commit_type": "feat",
"scope": "api",
"details": [
"Added TLS mutual authentication to prevent man-in-the-middle attacks (#100).",
"Implemented builder pattern to simplify complex transport configuration (#101).",
"Migrated 6 integration tests to exercise new security features (#102-#107)."
],
"issues": []
}

Example 2 - Refactor (provably unchanged):
{
"commit_type": "refactor",
"scope": "core",
"details": [
"Extracted validation logic into separate module to improve reusability.",
"Consolidated error handling across 12 functions to reduce code duplication.",
"Reorganized imports to eliminate circular dependencies."
],
"issues": []
}

Example 3 - Multi-component change (null scope) with mixed issue refs:
{
"commit_type": "feat",
"scope": null,
"details": [
"Added structured error handling to enable programmatic error recovery (#200).",
"Migrated from String errors to typed enums for better error matching (#201, #202).",
"Updated 15 files to ensure consistent error propagation (#203-#217)."
],
"issues": []
}

Example 4 - Fix with inline issue references:
{
"commit_type": "fix",
"scope": "parser",
"details": [
"Corrected off-by-one error causing buffer overflow on large inputs (#456).",
"Added bounds checking to prevent panic when processing empty files (#456, #457)."
],
"issues": []
}

Example 5 - Simple change (minimal details):
{
"commit_type": "chore",
"scope": "deps",
"details": [],
"issues": []
}

NOW ANALYZE THE DIFF AND CALL THE FUNCTION.
