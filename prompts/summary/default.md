Draft conventional commit summary (WITHOUT type/scope prefix).

═══════════════════════════════════════════════════════════════════════════════
READ-ONLY CONTEXT (DO NOT modify type/scope, already decided in analysis phase)
═══════════════════════════════════════════════════════════════════════════════

COMMIT TYPE (read-only): {{ commit_type }}
SCOPE (read-only): {{ scope }}
{% if user_context %}
USER CONTEXT (CRITICAL - must be incorporated into summary): {{ user_context }}
{% endif %}

DETAIL POINTS (basis for summary):
{{ details }}

DIFF STAT (supporting context):

```
{{ stat }}
```

═══════════════════════════════════════════════════════════════════════════════
YOUR TASK: Generate ONLY the description part
═══════════════════════════════════════════════════════════════════════════════

Output ONLY the text after "type(scope): " in: {{ commit_type }}({{ scope }}): <YOUR OUTPUT>
{% if user_context %}

⚠️  CRITICAL: The user-provided context above MUST be incorporated into the summary.
This is the most important part - ensure the summary reflects the user's context.
{% endif %}

REQUIREMENTS:

1. Maximum {{ chars }} characters
2. First word MUST be past-tense verb from ALLOWED LIST:
   added, fixed, updated, refactored, removed, replaced, improved, implemented,
   migrated, renamed, moved, merged, split, extracted, restructured, reorganized,
   consolidated, simplified, optimized, documented, tested, changed, introduced,
   deprecated, deleted, corrected, enhanced, reverted
3. Start lowercase
4. NO trailing period (conventional commits style)
5. Focus on primary change (single concept if scope specific)
6. NO leading adjectives before verb
7. CRITICAL - Convey motivation/purpose when space permits:
   - Include WHY when it adds essential context (not just WHAT)
   - Distill the purpose from detail points
   - Balance brevity with meaningful context

FORBIDDEN PATTERNS:

- DO NOT repeat commit type "{{ commit_type }}" in summary
  If type="refactor", use: restructured, reorganized, migrated, simplified,
  consolidated, extracted (NOT "refactored")
- NO filler words: "comprehensive", "improved", "enhanced", "various", "several"
- NO "and" conjunctions cramming multiple unrelated concepts
- NO meta phrases: "this change", "this commit"

GOOD EXAMPLES (showing type in parens for clarity):

- (feat) "added TLS support to prevent man-in-the-middle attacks"
- (refactor) "migrated HTTP transport to unified builder API for consistency"
- (fix) "corrected race condition causing connection pool exhaustion"
- (perf) "optimized batch processing to eliminate allocation overhead"
- (build) "updated serde to 1.0.200 for CVE-2024-1234 fix"

BAD EXAMPLES:

- (refactor) "refactor TLS configuration" ❌ (repeats type)
- (feat) "add comprehensive support for..." ❌ (filler + present tense)
- (chore) "update deps and improve build" ❌ (multiple concepts)
- (fix) "Fixed issue with parser" ❌ (capitalized)
- (feat) "added retry logic" ❌ (missing WHY - why was retry needed?)
- (refactor) "restructured error handling" ❌ (no motivation - what problem did it solve?)
- (perf) "optimized database queries" ❌ (vague - what was the performance issue?)

CHECKLIST BEFORE RESPONDING:
✓ Summary ≤{{ chars }} chars
✓ Starts lowercase
✓ First word is past-tense verb from allowed list
✓ Does NOT repeat type "{{ commit_type }}"
✓ NO trailing period
✓ NO filler words
✓ Single focused concept
✓ Aligns with detail points
✓ Specific (names subsystem/artifact when relevant)
✓ Conveys motivation/purpose when space permits (WHY, not just WHAT)
