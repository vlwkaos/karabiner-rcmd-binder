---
slug: native-window-cycling
kind: coding
title: Native window cycling (rcmdb cycle-window) replacing osascript/System Events
description: Window cycling moved out of an osascript JXA helper into a native rcmdb subcommand driving the Accessibility C API directly, for speed and reliability under load.
keywords: [native window cycling, rcmdb cycle-window subcommand, Accessibility C API, AXUIElementCreateApplication, AXUIElementPerformAction AXRaise, AXIsProcessTrustedWithOptions prompt, objc2 objc2-app-kit NSWorkspace frontmostApplication, cycle-window.sh thin launcher, osascript System Events removal, TCC accessibility permission rcmdb, QoS user-interactive, extern C FFI CoreFoundation Boolean u8, CGWarpMouseCursorPosition centering]
created: 2026-08-01
modified: 2026-08-01
---

targets:
- slug: native-window-cycling, title: "Native window cycling (rcmdb cycle-window)", kind: coding
- slug: accessibility-permission-model, title: "Accessibility permission model (rcmdb vs osascript)", kind: domain

plan_feature: window-cycling-performance

## Native window cycling (rcmdb cycle-window)
<!-- slug: native-window-cycling | target: knowledge/coding/Native window cycling.md | op: create | base: new -->
+add: Window cycling is now a native subcommand of the `rcmdb` binary, not an osascript helper. Chain: keypress → Karabiner `shell_command` → `cycle-window.sh` (thin launcher) → `exec rcmdb cycle-window <bundle> [mode]`.
+add: Why the rewrite — the old `cycle-window.sh` ran `osascript -l JavaScript` (JXA interpreter cold start ~100ms+ under load) and enumerated/raised windows through the shared **System Events** process over Apple Events, whose queue backs up under contention. That is what made cycling "stuck"/unresponsive. Native start is a few ms and talks to the target app's AX server directly with no System Events middleman.
+add: Implementation lives in `src/window.rs`. Reads frontmost app via `objc2-app-kit` `NSWorkspace::sharedWorkspace().frontmostApplication()` (bundleIdentifier + processIdentifier). Launch/focus (first press) = `Command::new("open").arg("-b").arg(bundle)` (no AX needed). Repeat press = AX C API: `AXUIElementCreateApplication(pid)` → `AXWindows` → raise the **backmost non-minimized** window (round-robins all windows; raising index 1 only toggles the top two — the old stuck bug). Centering = read `AXPosition`/`AXSize` via `AXValueGetValue` + `CGWarpMouseCursorPosition`.
+add: FFI is hand-rolled `extern "C"` (no cached AX crate): ApplicationServices (AX*), CoreFoundation (CFArray/CFBoolean/CFDictionaryCreate + `kCFBooleanTrue`/`kCFTypeDictionary*CallBacks`/`kAXTrustedCheckOptionPrompt`), CoreGraphics (CGWarp). Deps added (exact-pinned, offline cache): `objc2=0.6.4`, `objc2-foundation=0.3.2`, `objc2-app-kit=0.3.2`.
+add: GOTCHA — CoreFoundation `Boolean` is `unsigned char`, NOT C `_Bool`. Declare AX/CF bool-returning fns as `-> u8` and compare `!= 0`; declaring `-> bool` is UB if a byte other than 0/1 is returned.
+add: Process is short-lived (one press, then exits), so AX `Copy*` +1-retained refs are deliberately LEAKED rather than CFRelease'd — OS reclaims on exit; array-element Get-semantics stay valid because the owning array is never released. Also sets `pthread_set_qos_class_self_np(QOS_CLASS_USER_INTERACTIVE)` so a press is scheduler-prioritized under load. No resident daemon — strictly per-press.
+add: `main.rs` dispatches `cycle-window` and `accessibility` subcommands BEFORE any terminal setup (raw mode / alt screen), returning early. Empty bundle → `exit(1)`.
+add: `cycle-window.sh` is now a thin launcher (see `CYCLE_WINDOW_SCRIPT` in `src/scripts/mod.rs`): `export PATH=/opt/homebrew/bin:/usr/local/bin:$HOME/.local/bin:$HOME/.cargo/bin:$PATH; exec rcmdb cycle-window "$@"`. karabiner.json + generator are UNCHANGED (still reference the stable script path with the same args) — so NO re-apply/re-save is needed after upgrading; only the script body + binary change. PATH resolution (not a baked exe path) keeps it valid across `brew upgrade` and install method.
+add: Tests — `center_wanted()` is the pure/testable seam (unit tests in `src/window.rs` pass `mtm=None` to avoid NSScreen); `src/scripts/mod.rs` has `test_cycle_window_is_thin_launcher_to_native_binary` guarding `exec rcmdb cycle-window "$@"` + `export PATH=` + no-osascript. The AX window-raising itself is only verifiable live (needs granted permission + real windows).

## Accessibility permission model (rcmdb vs osascript)
<!-- slug: accessibility-permission-model | target: knowledge/domain/Accessibility permission model.md | op: create | base: new -->
+add: Window cycling now requires Accessibility (TCC) granted to the **`rcmdb`** binary; previously it rode on `osascript`'s grant. Center Mouse (non-cycling) still uses `osascript`, so the two features now depend on DIFFERENT binaries' grants.
+add: GOTCHA — a CLI binary invoked from a script gets NO automatic TCC prompt, and untrusted AX calls fail **silently** (cycling just does nothing; first-press `open -b` focus still works, which masks the problem). The only way to surface the dialog is calling `AXIsProcessTrustedWithOptions` with `kAXTrustedCheckOptionPrompt=true` — `prompt_accessibility_if_needed()` does this on first press.
+add: New observable onboarding command `rcmdb accessibility` triggers the prompt and prints granted/not-granted (+ the binary path to add manually via System Settings → Privacy & Security → Accessibility → +). README + CHANGELOG updated.
+add: On release, the grant is keyed to the binary's cdhash; a new build (e.g. 0.6.2 universal) requires re-approval.

## plan
- window-cycling-performance: native rewrite DONE (pending commit + 0.6.2 release). Follow-up option not taken: port Center Mouse's osascript path to native AX too.

## Trials & Solutions
- Tried hot-swapping only cycle-window.sh (like the prior round-robin fix) → fails: launcher execs `rcmdb cycle-window`, a subcommand the installed 0.6.1 binary lacks → had to install the new binary too (cp over the brew Cellar file; `brew reinstall rcmdb` reverts).
- Tried `!contains("osascript")` launcher-guard test → failed: the launcher's own comment said "no osascript…" → reworded comment to "scripting-interpreter" so the guard means "never invokes osascript".
- No AX popup appeared → root cause: new rcmdb binary not TCC-trusted and code wasn't calling the prompting trust check → added `AXIsProcessTrustedWithOptions(prompt)` + `rcmdb accessibility` command; user granted, cycling confirmed working live.
- Local test binary is arm64 dev build over brew's Cellar path; official universal build ships via 0.6.2 release.

## pending
- [x] Native `rcmdb cycle-window` + `accessibility` subcommands, thin launcher, objc2 deps
- [x] /backpressure quality loop (good-to-go + sec + write-test-audit; README fix, FFI u8 soundness, +4 tests) — 65 tests pass, clippy clean
- [x] User verified cycling works live after granting Accessibility
- [ ] Commit the change (no AI attribution)
- [ ] Release 0.6.2 via /release rust flow (universal + bottle + crates.io + tap); user re-grants Accessibility to the new binary
