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
