# Scaffold Test Pattern

Test the described feature/change using parallel subagents with temporary git repos.

## Instructions

1. **Install the binary first**: Run `cargo install --path . --force` before launching any subagents
2. **Identify edge cases** for the feature being tested (aim for 6-10 scenarios)
3. **Spawn parallel subagents** using the Task tool, each testing one scenario
4. Each subagent should:
   - Create a temp git repo at `/tmp/test-<name>-$$` (unique per scenario)
   - Set up the minimal scaffold needed (files, git commits, staged changes)
   - Run the tool/command being tested
   - Verify expected behavior
   - Report PASS/FAIL with details
   - Clean up the temp directory

## Use Lua for Scaffolds

Lua is ideal for test scaffolds: no compilation, no deps, no LSP setup.

```lua
-- src/api.lua
local M = {}

function M.new_client(base_url)
   return { url = base_url, timeout = 30 }
end

function M.with_timeout(client, timeout)
   client.timeout = timeout
   return client
end

return M
```

## Subagent Prompt Template

```
Test [SCENARIO NAME] for [FEATURE].

IMPORTANT: Do NOT touch the main repo or build anything. The binary is already installed - use `llm-git` directly.

Create a temp git repo at /tmp/test-[name]-$$ with:
1. [Describe initial state - files, commits, config]
2. [Describe the change to stage/test]

Use Lua (.lua) for any source files - keeps scaffolds minimal.

Run `llm-git --dry-run --dir /tmp/test-[name]-$$` (or other flags as needed)

Verify: [Expected behavior]
Report: PASS/FAIL with details

Clean up the temp dir after.
```

## Example Edge Cases

For a changelog feature:
- Duplicate entry detection (existing entry matches staged change)
- Unrelated existing entries (should preserve + add new)
- Package-scoped changelog (no scope prefix)
- Multiple changelogs (changes route to correct one)
- Missing required section (should warn and skip)
- Internal-only changes (should produce no entries)
- Root-only fallback (nested files → root changelog)
- Semantic deduplication (same meaning, different words)

## Execution Pattern

```
<Task subagent_type="general-purpose" description="Test scenario 1">...</Task>
<Task subagent_type="general-purpose" description="Test scenario 2">...</Task>
...all in parallel (single message with multiple Task calls)
```

## Analyzing Results

After all subagents complete, summarize in a table:

| # | Scenario | Result | Notes |
|---|----------|--------|-------|
| 1 | ... | ✅/❌ | ... |

Investigate any failures by reproducing locally with debug output.

---

$ARGUMENTS
