# Big Picture

## Project Overview

**karabiner-rcmd-binder** - TUI for managing Karabiner-Elements right_command key bindings with app launching, URL tab focusing, and action cycling.

## Core Data Flow

```
User (TUI) --> config.toml --> Karabiner JSON --> macOS Shortcuts
     |              |
     |              +-- Helper Scripts (embedded in binary)
     v
  App State (in-memory)
```

## Key Integration Points

1. **Config Layer**: `~/.config/karabiner-rcmd-binder/config.toml` (simple TOML) -> generates complex Karabiner JSON
2. **Karabiner Integration**: Namespaced rules `[rcmdb]`, backup system (3 rotating), variable-based cycling
3. **Helper Scripts**: Embedded in binary via `include_str!`, installed to `~/.config/karabiner-rcmd-binder/scripts/`
4. **Mode System**: INPUT (text entry) vs NAV (commands) - affects title, border, status bar, key handlers

## Critical Dependencies (Change Impact)

| Change | Affects |
|--------|---------|
| New field in editor | Mode detection (`is_input_mode`), UI rendering, key routing |
| Command key added | Guard clauses in ALL editor handlers |
| Autocomplete change | Render order (must be LAST), positioning, discovery system |
| New action type | `Action` enum, `ActionType`, generator, UI editors |
| New browser | `Browser` enum, scripts, generator |

## Component Map

```
src/
├── main.rs           # Event loop, key dispatch, mpsc channel for discovery
├── app.rs            # App state, editor structs, autocomplete
├── app_discovery.rs  # Dynamic app scanning with bundle IDs
├── keycodes.rs       # Valid Karabiner key codes
├── validation.rs     # Bundle ID resolution and validation
├── config/
│   ├── model.rs      # Domain: Browser, Action, Binding, Config
│   └── persistence.rs # TOML load/save
├── karabiner/
│   ├── generator.rs  # JSON rule generation, cycling logic
│   └── backup.rs     # Timestamped backup rotation
├── scripts/          # Embedded shell scripts
│   └── mod.rs        # url-focus.sh for browser tab detection
└── ui/
    ├── mod.rs        # Main draw dispatcher
    ├── layout.rs     # Tabs, status bar, mode indicators
    ├── editor.rs     # Binding & action editors
    ├── bindings.rs   # Binding list view
    └── settings.rs   # Settings tab (default browser)
```

## Threading Model

```
Main Thread                    Discovery Thread
    |                               |
    +-- poll events (50ms) ----+    |
    |                          |    |
    +-- try_recv() <-----------+----+ mpsc::channel
    |                               |
    +-- render UI                   +-- osascript (running apps)
                                    +-- read_dir (installed apps)
```

## Index

- @knowledge/bigpicture/PLAN.md - Feature plan & progress
- @knowledge/domain/DOMAIN.md - Data models & workflows
- @knowledge/coding/CODING.md - Patterns & conventions
