---
slug: native-window-cycling
kind: coding
title: Native window cycling (rcmdb cycle-window)
description: Window cycling runs as a native rcmdb subcommand driving the Accessibility C API directly, replacing the old osascript/System Events JXA helper.
keywords: [native window cycling, rcmdb cycle-window subcommand, Accessibility C API, AXUIElementCreateApplication, AXUIElementPerformAction AXRaise, AXIsProcessTrustedWithOptions prompt, objc2 objc2-app-kit NSWorkspace frontmostApplication, cycle-window.sh thin launcher, osascript System Events removal, QoS user-interactive, extern C FFI CoreFoundation Boolean u8, CGWarpMouseCursorPosition centering]
created: 2026-08-01
modified: 2026-08-01
---

# Native window cycling (rcmdb cycle-window)

Window cycling is a **native subcommand of the `rcmdb` binary**, not an osascript
helper. This superseded the earlier osascript/JXA + System Events implementation
(see `knowledge/history/Window Cycling Performance.md` for the timeline).

## Invocation chain

```
keypress → Karabiner shell_command → cycle-window.sh (thin launcher)
        → exec rcmdb cycle-window <bundle_id> [center_mode]
```

- `cycle-window.sh` (body in `CYCLE_WINDOW_SCRIPT`, `src/scripts/mod.rs`) is now a
  thin launcher: `export PATH=/opt/homebrew/bin:/usr/local/bin:$HOME/.local/bin:$HOME/.cargo/bin:$PATH; exec rcmdb cycle-window "$@"`.
  PATH resolution (not a baked exe path) keeps it valid across `brew upgrade` and
  install method.
- `karabiner.json` + the generator are **UNCHANGED** — they still reference the
  stable script path with the same args, so **no re-apply / re-save** is needed
  after upgrading; only the script body + binary change.

## Implementation (`src/window.rs`)

| Step | Mechanism |
|------|-----------|
| Read frontmost app | `objc2-app-kit` `NSWorkspace::sharedWorkspace().frontmostApplication()` → bundleIdentifier + processIdentifier |
| First press (launch/focus) | `Command::new("open").arg("-b").arg(bundle)` — no AX needed |
| Repeat press (cycle) | AX C API: `AXUIElementCreateApplication(pid)` → `AXWindows` → raise the **backmost non-minimized** window (round-robins all windows; raising index 1 only toggles the top two — the old stuck bug) |
| Centering | read `AXPosition`/`AXSize` via `AXValueGetValue` + `CGWarpMouseCursorPosition` |

- `main.rs` dispatches the `cycle-window` and `accessibility` subcommands **BEFORE**
  any terminal setup (raw mode / alt screen), returning early. Empty bundle → `exit(1)`.
- Process is short-lived (one press, then exits): sets
  `pthread_set_qos_class_self_np(QOS_CLASS_USER_INTERACTIVE)` so a press is
  scheduler-prioritized under load. No resident daemon — strictly per-press.

## FFI gotchas

- FFI is hand-rolled `extern "C"` (no cached AX crate): ApplicationServices (AX*),
  CoreFoundation (CFArray/CFBoolean/CFDictionaryCreate + `kCFBooleanTrue` /
  `kCFTypeDictionary*CallBacks` / `kAXTrustedCheckOptionPrompt`), CoreGraphics (CGWarp).
- **CoreFoundation `Boolean` is `unsigned char`, NOT C `_Bool`.** Declare AX/CF
  bool-returning fns as `-> u8` and compare `!= 0`; declaring `-> bool` is UB if a
  byte other than 0/1 is returned.
- AX `Copy*` +1-retained refs are deliberately **LEAKED** rather than CFRelease'd —
  the OS reclaims on process exit; array-element Get-semantics stay valid because
  the owning array is never released.
- Deps added (exact-pinned, offline cache): `objc2=0.6.4`, `objc2-foundation=0.3.2`,
  `objc2-app-kit=0.3.2`.

## Testing seams

- `center_wanted()` is the pure/testable seam — unit tests in `src/window.rs` pass
  `mtm=None` to avoid NSScreen.
- `src/scripts/mod.rs` `test_cycle_window_is_thin_launcher_to_native_binary` guards
  `exec rcmdb cycle-window "$@"` + `export PATH=` + no-osascript.
- The AX window-raising itself is only verifiable live (needs granted permission +
  real windows).
