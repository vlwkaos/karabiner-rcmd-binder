# Domain Knowledge

Project domain knowledge index.

## Entities

### Config
Root configuration structure.
- `settings: Settings` - Global settings
- `bindings: Vec<Binding>` - Key bindings list

### Settings
Global configuration.
- `default_browser: Browser` - Fallback browser for URL actions

### Binding
A key-to-action mapping.
- `key: String` - Karabiner key code (e.g., "t", "f1", "spacebar")
- `description: String` - Human-readable label
- `actions: Vec<Action>` - One or more actions (cycles if >1)

### Action (enum)
What happens when binding triggers.

| Variant | Fields | Purpose |
|---------|--------|---------|
| `App` | `target: String, bundle_id: Option<String>` | Open/focus application via bundle ID (more reliable) |
| `Url` | `target, match_type, browser` | Open URL with tab focus |
| `Shell` | `command: String` | Run shell command |

### DiscoveredApp
App metadata from system scanning.
- `name: String` - Display name (e.g., "KakaoWork")
- `bundle_id: String` - macOS bundle identifier (e.g., "com.kakaoenterprise.macos.kakaowork")
- `last_component: String` - Last part of bundle ID for disambiguation (e.g., "kakaowork")

### Browser (enum)
Supported browsers: Firefox, Chrome, Safari, Arc, Edge

### UrlMatchType (enum)
How to match existing browser tabs:
- `Exact` - Full URL match
- `Domain` - Any page on domain
- `Path` - Domain + path, ignore query
- `Glob` - Wildcard with `*`

## Relationships

```
Config 1--1 Settings
Config 1--* Binding
Binding 1--* Action
Action(Url) *--1 Browser (optional, falls back to Settings.default_browser)
Action(Url) 1--1 UrlMatchType
```

## Workflows

### Binding Creation
1. Press `a` in Bindings tab
2. Enter key code (autocomplete available)
3. Enter description
4. Add actions (Tab to Actions field, press `a`)
5. Tab to save, or Esc to cancel

### Action Cycling
When binding has multiple actions:
- Karabiner variable `rcmdb_<key>_cycle` tracks position
- Each press increments modulo action count
- Wraps around: 0 -> 1 -> 2 -> 0

### Window Cycling (`cycle_windows`)
Distinct from Action Cycling: rotates through the **windows of one app**, not through
multiple actions.
- `cycle_windows` is only honored by the generator when an App action is a binding's
  **sole action** AND has a **non-empty `bundle_id`**. Multi-action (`actions.len() > 1`)
  always ignores it; a bundle-less App falls back to `open -a` and drops the flag.
- Default is **on** for single-app configs (`ActionEditor::new()` sets `true`); set
  `cycle_windows = false` to opt out.
- Runtime: `keypress → cycle-window.sh (thin launcher) → rcmdb cycle-window <bundle> [mode]`.
  First press launches/focuses via `open -b`; repeat presses raise the backmost
  non-minimized window (round-robins all windows) via the Accessibility C API.
  See `knowledge/coding/Native window cycling.md`.

### URL Tab Focus
1. Script checks if browser has matching tab
2. Match based on UrlMatchType
3. If found: focus tab
4. If not found: open new tab

## Terminology

| Term | Meaning |
|------|---------|
| rcmd | right_command modifier key |
| rcmdb | Project namespace prefix in Karabiner rules |
| cycling | Rotating through multiple actions on same key |
| window cycling | Rotating through one app's windows on repeat presses (`cycle_windows`) |
| tab focus | Finding and activating existing browser tab |

## Business Rules

1. One binding per key (no duplicate keys)
2. Empty actions list = binding does nothing
3. Per-action browser overrides Settings.default_browser
4. Browser field `None` = use default
5. Backups rotate: keep newest 3
6. Bundle IDs preferred over app names for launching (more reliable)
7. Bundle IDs auto-resolved on save if missing
8. Key validation: must be valid Karabiner keycode or use autocomplete
9. App discovery limited to 150 apps for performance
10. `cycle_windows` only honored for a sole App action with a non-empty `bundle_id`; default on for single-app configs
11. Window cycling requires Accessibility (TCC) granted to the `rcmdb` binary; Center Mouse (non-cycling) still uses osascript's grant — see `knowledge/domain/Accessibility permission model.md`
