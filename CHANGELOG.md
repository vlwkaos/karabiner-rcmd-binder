## [0.5.2] - 2026-04-25

### Features

- Add `multi_monitor_only` mode for Center Mouse: when selected, the cursor-warp runs only when multiple displays are connected; no-ops silently on single-monitor setups. Settings now cycle Off → ON → MULTI ONLY via `space`.

### Migration

`center_mouse` in `config.toml` changes from a boolean to a string enum (`"off"` | `"always"` | `"multi_monitor_only"`). Existing `center_mouse = true` configs are automatically migrated to `"always"` on next load.

---

## [0.5.1] - 2026-04-24

### Bug Fixes

- Fix script paths baked as absolute user paths at config-save time; now embed `$HOME` so the shell expands it at Karabiner runtime ([`7e4efa0`](https://github.com/vlwkaos/karabiner-rcmd-binder/commit/7e4efa0c9cfd2e7513ee7cddab0edf201bea9a44))
- Fix `#!/bin/bash` shebangs in embedded scripts to `#!/usr/bin/env bash` for portability ([`7e4efa0`](https://github.com/vlwkaos/karabiner-rcmd-binder/commit/7e4efa0c9cfd2e7513ee7cddab0edf201bea9a44))
- Fix case-sensitive bundle ID assertion in validation test ([`77d11e1`](https://github.com/vlwkaos/karabiner-rcmd-binder/commit/77d11e15ec186496337c4e52d07cd60a3feed668))

---

## [0.5.0] - 2026-04-16

### Features

- Add global "Center Mouse on App Focus" setting: when enabled, every app-launch binding moves the mouse to the center of the focused window after the app comes to the foreground ([`bb20149`](https://github.com/vlwkaos/karabiner-rcmd-binder/commit/bb20149fada067ae5a7807a86148eccc2d398848))
- Poll-until-frontmost: the embedded `center-mouse.sh` script polls System Events up to 0.5s for the target bundle ID to become frontmost — no fixed sleep, no blind wait
- Cancel and restart semantics: re-pressing the binding while the script is still polling kills the previous attempt via PID lock file
- No external dependencies: implemented via JXA + CoreGraphics ObjC bridge (`CGWarpMouseCursorPosition`)

### Migration

The `[settings]` section of `config.toml` now supports an optional `center_mouse` field (defaults to `false`, omitted from file when false). Existing configs load without changes. To activate, open the Settings tab and toggle with `space`, then press `s` to save.

> **macOS permission**: the first time Center Mouse fires, macOS will prompt to grant Accessibility access to `osascript` (Privacy & Security > Accessibility).

---

## [0.4.1] - 2025-01-01

### Bug Fixes

- Add suggested dynamic binding directly without opening editor
