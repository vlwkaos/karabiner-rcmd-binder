# karabiner-rcmd-binder

TUI for Karabiner-Elements right_command key bindings.

## Project Knowledge

MUST use knowledge-user skill when needing context about plan, big picture, architecture, domain, data models, or coding patterns.

For session continue see @knowledge/session/
For big picture (how one change affects others, plan), see @knowledge/bigpicture/BIGPICTURE.md and @knowledge/bigpicture/PLAN.md
For semantic understanding of domain concepts (data model, entities, relationships, workflows), see @knowledge/domain/DOMAIN.md
For project-specific coding patterns, standards, and best practices, see @knowledge/coding/CODING.md

## Quick Start

```bash
./run.sh          # Run TUI
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
