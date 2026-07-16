## [0.6.1] - 2026-07-16

### Changed

- Window cycling (`cycle_windows`) now defaults **on** for single-App actions in the Action Editor: adding an App action and selecting an app with a bundle ID enables window cycling automatically. Set it to `false` (toggle in the editor) to opt out. Actions without a bundle ID are unaffected: the flag is still gated on a bundle ID and never persisted without one.

### Performance

- Faster app focus / window cycling. Three changes: (1) the helpers read the frontmost app in-process via `NSWorkspace` instead of a `System Events` Apple-Events query (~30-50ms saved per press); (2) cursor-centering polls at 10ms in-process instead of 50ms over Apple Events, so it centers the instant the app activates; (3) when a single-App binding both cycles windows and centers the mouse, it now runs in **one** `osascript` (cycle-window.sh centers itself) instead of chaining a second `center-mouse.sh` process, saving a whole JXA interpreter start (~50-80ms). Accessibility (AX) is used only for the one-time window-geometry read.

## [0.6.0] - 2026-07-02

### Features

- Add per-App-action **window cycling** (`cycle_windows`): when a binding has a single App action with a bundle ID, the first press focuses/launches the app and repeat presses rotate through its windows (wrapping). Coexists with multi-action cycling, which continues to own keys bound to more than one action. Toggle it in the Action Editor (`←`/`→`/`space`); requires the same Accessibility permission as Center Mouse.

### Security

- Shell-escape all user-supplied fields (`bundle_id`, app/URL `target`) interpolated into generated Karabiner `shell_command` strings, using POSIX single-quote escaping. Also quotes the previously bare `open -b <bundle_id>` argument. Prevents a crafted config or discovered app name containing shell metacharacters from breaking out of the generated command.

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
