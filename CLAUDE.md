# CLAUDE.md — Brochure

@~/.claude/shared/rust-conventions.md

---

## Keeping This File Current

**Update CLAUDE.md whenever you:**

- Add, rename, or restructure a module or directory
- Change a module's sole responsibility
- Add a new persisted type or data file
- Add or change a script in `scripts/`
- Change the testing workflow
- Establish a new non-negotiable rule
- **Change the task workflow**

**Do not let CLAUDE.md drift from the code.** If you notice a stale reference (wrong file path, removed type, changed
rule), fix it in the same commit.

---

## Module Map

Run `rust-ast-extractor dir src/` for a live index of all source files and their responsibilities.
Each file's `//!` module doc is the authoritative description — it is never out of date.

---

## Code Quality (non-negotiable, apply on every edit)

Whenever you read or edit a file — even for a small fix — also look for:

- **Duplicate or near-duplicate code** that belongs in a shared helper function.
- **Functions that are too long** and would be clearer if split into smaller, focused helpers.
- **Files that have grown too large** and would benefit from being split into sub-modules.
- **Obvious simplifications**: dead branches, redundant variables, needlessly verbose patterns.

Make these improvements in the same commit when they are clearly better and low-risk.
When the improvement is larger or uncertain, surface it to the user as a suggestion.