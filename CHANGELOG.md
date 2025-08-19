# 0.3.0

- Update chain-registry git ref to latest (e458ac19d3428e35f8b2ba634eb01ae1d359b46a - 2025-08-19)
- Fix error handling: nonexistent chains/paths now return `None` instead of errors
- Remove debug output from parse_json
- Add comprehensive integration tests
- Add tokio as dev dependency for examples

# 0.2.0-rc3

- Remove `deny_unknown_fields` serde directive

# 0.2.0-rc2

- Add missing fields to chain model

# 0.2.0-rc1

- Improve docs by adding examples on the front page
- Support commit 350840e766f7574a120760a13eda4c466413308a
