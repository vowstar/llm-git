Generate changelog entries for the changes below.

<context>
Changelog location: {{ changelog_path }}
{% if is_package_changelog %}Package-scoped changelog — do NOT prefix entries with package name.{% endif %}
</context>

<diff_summary>
{{ stat }}
</diff_summary>

<diff>
{{ diff }}
</diff>
{% if existing_entries %}

<existing_entries>
{{ existing_entries }}
</existing_entries>
{% endif %}

<instructions>
1. Analyze the diff for user-visible changes
2. {% if existing_entries %}Skip any change already covered in <existing_entries>{% else %}Identify changelog-worthy modifications{% endif %}
3. Categorize each change using Keep a Changelog format
4. Write entries as past-tense action verbs describing user impact
5. Output JSON with entries grouped by category

Categories (include only those with entries):
- Added: New features, capabilities, public APIs
- Changed: Modifications to existing functionality
- Deprecated: Features marked for removal
- Removed: Deleted features or APIs
- Fixed: Bug fixes, corrections
- Security: Security-related changes
- Breaking Changes: API-incompatible changes (use sparingly)
</instructions>

<entry_format>
- Past-tense action verb (Added, Implemented, Fixed, Updated)
- User-visible impact, not internal implementation
- Specific: name the feature, option, or behavior
- Concise: 1-2 lines maximum
- No trailing periods
- No scope/package prefixes
</entry_format>

<include>
- New user-facing features or options
- Behavior changes users will notice
- Bug fixes with observable impact
- Performance improvements users can perceive
- API additions or changes
</include>

<exclude>
- Internal refactoring invisible to users
- Code style/formatting changes
- Import reorganization
- Test-only changes
- Documentation-only changes (unless significant)
- Trivial updates with no user impact
</exclude>

<examples>
<example type="good">
- Added `interruptMode` option to control when queued messages interrupt tool execution
- Implemented "immediate" and "wait" modes for interrupt handling
- Fixed race condition when multiple messages arrive during tool execution
</example>

<example type="bad">
- **agent**: Added interruptMode option  ← Redundant scope prefix
- Added new feature.  ← Vague, has trailing period
- Refactored internal state machine  ← Not user-visible
- Updated imports  ← Trivial change
</example>
</examples>

<output_format>
Return JSON only. No preamble, no explanation.

If changelog-worthy changes exist:
```json
{
  "entries": {
    "Added": ["entry 1", "entry 2"],
    "Fixed": ["entry 1"],
    "Changed": ["entry 1"]
  }
}
```

If no changelog-worthy changes:
```json
{"entries": {}}
```
</output_format>
