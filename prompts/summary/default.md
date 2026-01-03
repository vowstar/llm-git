Generate the description for: {{ commit_type }}({{ scope }}): <YOUR OUTPUT>

<context>
Commit type and scope are fixed from the analysis phase.
Your output becomes the text after the colon in the conventional commit message.
{% if user_context %}User-provided context MUST be incorporated: {{ user_context }}{% endif %}
</context>

<data>
<detail_points>
{{ details }}
</detail_points>

<diff_stat>
{{ stat }}
</diff_stat>
</data>

<instructions>
1. Synthesize detail points into a single focused summary
2. Start with a past-tense verb from the allowed list (lowercase, no leading adjectives)
3. Include WHY when it adds essential context, not just WHAT changed
4. Be specific: name the subsystem, file, or artifact when relevant
5. Stay under {{ chars }} characters
6. No trailing period
</instructions>

<allowed_verbs>
added, fixed, updated, refactored, removed, replaced, improved, implemented,
migrated, renamed, moved, merged, split, extracted, restructured, reorganized,
consolidated, simplified, optimized, documented, tested, changed, introduced,
deprecated, deleted, corrected, enhanced, reverted
</allowed_verbs>

<constraints>
- Do NOT repeat the commit type "{{ commit_type }}" as your verb
  (if type="refactor", use: restructured, reorganized, migrated, simplified, etc.)
- No leading adjectives before the verb ("quickly added..." ❌)
- No filler words: "comprehensive", "various", "several", "improved", "enhanced"
- No conjunctions cramming multiple concepts
- No meta phrases: "this change", "this commit"
- Single focused concept only
- Must align with detail points from analysis
</constraints>

<examples>
<example>
Type: feat
Detail: Added TLS encryption to HTTP client to prevent MITM attacks
Output: added TLS support to prevent man-in-the-middle attacks
</example>

<example>
Type: refactor
Detail: Consolidated HTTP transport code into unified builder pattern for consistency
Output: migrated HTTP transport to unified builder API for consistency
</example>

<example>
Type: fix
Detail: Fixed race condition in connection pool causing exhaustion under load
Output: corrected race condition causing connection pool exhaustion
</example>

<example>
Type: perf
Detail: Optimized batch processing to reduce memory allocations
Output: optimized batch processing to eliminate allocation overhead
</example>

<example>
Type: build
Detail: Updated serde dependency to fix CVE-2024-1234
Output: updated serde to 1.0.200 for CVE-2024-1234 fix
</example>
</examples>

<anti_patterns>
These are BAD because they lack motivation or specificity:
- "added retry logic" ❌ → missing WHY (why was retry needed?)
- "restructured error handling" ❌ → no motivation (what problem did it solve?)
- "optimized database queries" ❌ → vague (what was the performance issue?)
- "updated HTTP client" ❌ → too generic (which aspect? why?)
</anti_patterns>

Output ONLY the description text. No explanation.
