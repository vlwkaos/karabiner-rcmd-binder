# karabiner-rcmd-binder

TUI for Karabiner-Elements right_command key bindings.

## Project Knowledge

MUST use qmd search when needing context about plan, big picture, architecture, domain, or coding patterns.

**Big Picture**:
- @qmd://karabiner-rcmd-binder/bigpicture/BIGPICTURE.md - Project overview, architecture layers
- @qmd://karabiner-rcmd-binder/bigpicture/PLAN.md - Feature roadmap, decision log

**Domain**:
- @qmd://karabiner-rcmd-binder/domain/karabiner-integration.md - How TUI integrates with Karabiner-Elements
- @qmd://karabiner-rcmd-binder/domain/dynamic-bindings.md - Ephemeral suggestion system

**Coding**:
- @qmd://karabiner-rcmd-binder/coding/nav-edit-mode-pattern.md - Two-mode UX implementation
- @qmd://karabiner-rcmd-binder/coding/ratatui-architecture.md - State management, rendering separation
- @qmd://karabiner-rcmd-binder/coding/keyboard-handling.md - Event routing patterns

**Sessions**: @qmd://karabiner-rcmd-binder/session/ - Recent work context

## Quick Start

```bash
# For end users (published via Homebrew)
brew install karabiner-rcmd-binder
rcmdb

# For development
cargo run         # Run TUI in development
cargo build --release  # Build release binary
cargo test        # Run tests
```

## Key Patterns

1. **Mode Detection**: Check `is_input_mode()` before handling command keys
2. **Autocomplete**: Always render LAST in draw functions
3. **Background Tasks**: Use mpsc channel, poll with `try_recv()` in event loop

## Critical Files

- `src/main.rs` - Event loop, key handlers
- `src/app.rs` - State, mode detection, editors
- `src/config/model.rs` - Domain types (Action, Binding, Browser)
- `src/karabiner/generator.rs` - JSON rule generation

## Release

release.flow: rust

Releases go through the `/release` rust flow: it builds a universal (arm64 +
x86_64) binary via `lipo`, GPG-signs the tarball (no Apple codesign), creates a
Homebrew bottle, publishes the GitHub release, runs `cargo publish`, and updates
the Homebrew tap formula. Artifacts are `rcmdb-<ver>-darwin-universal.tar.gz` +
`rcmdb-<ver>.all.bottle.tar.gz`. There is intentionally no local `make release`:
the former `scripts/release.sh`/`package.sh`/`sign.sh` built arm64-only,
Apple-codesigned artifacts with no bottle and were removed.

## good-to-go

Project-specific axes (extend defaults, do not replace):

| Axis | What to check |
|------|---------------|
| **README config example** | `README.md` config block shows current `center_mouse` values (`"off"` \| `"always"` \| `"multi_monitor_only"`); update when enum variants change |
| **Embedded script args** | When `CenterMouseMode::as_str()` values change, verify they match the bash `case`/`if` strings inside `CENTER_MOUSE_SCRIPT` in `src/scripts/mod.rs` |
| **Serde legacy compat** | `CenterMouseMode` deserializer accepts both old bool and new string — must keep `visit_bool` if removing a variant that maps to an old bool value |
| **Runtime script embedded vs installed** | Every helper script referenced by the generator (`SCRIPTS_RUNTIME_DIR/*.sh`) must be embedded as a `const` in `src/scripts/mod.rs` AND written in `install_scripts()`. Adding a generator invocation without both = runtime "file not found". |
| **Flag gated on prerequisite** | A per-action bool that the generator ignores without a prerequisite (e.g. `cycle_windows` needs a non-empty `bundle_id`) must be gated everywhere the state is set OR persisted: input handler AND `ActionEditor::to_action` (and any default in `ActionEditor::new`), so the UI can neither show nor save a state the generator silently drops. |

- Uncertain about project term/schema/convention/prior decision → `/seek <topic>` first (lightweight KB lookup; same tier as grep/Glob).
