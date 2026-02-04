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
