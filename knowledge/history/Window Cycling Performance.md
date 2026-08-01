---
plan_feature: window-cycling-performance
kind: history
title: Window Cycling Performance
description: History of making right_command window cycling fast and reliable — from osascript/System Events latency tweaks to a native rcmdb cycle-window subcommand on the Accessibility C API.
keywords: [window cycling performance, osascript cold start, NSWorkspace frontmostApplication, System Events Apple Events, native cycle-window, Accessibility C API, rcmdb subcommand]
created: 2026-08-01
modified: 2026-08-01
---

# Window Cycling Performance

Completed feature. Current implementation is native — see
`knowledge/coding/Native window cycling.md` and
`knowledge/domain/Accessibility permission model.md`. This note keeps the
chronology and every Trials & Solutions bullet.

## Timeline

### 1. osascript latency tweaks (2026-07-16, superseded)
The keypress hot path ran two serial `osascript` (JXA) processes:
`cycle-window.sh <bundle> && center-mouse.sh <bundle> <mode>`.
- osascript cold start (2 ObjC imports): ~50-80ms.
- Single-shot frontmost check: `System Events processes.whose({frontmost:true})`
  ~100-140ms vs `$.NSWorkspace...frontmostApplication.bundleIdentifier.js` ~70-90ms.
- Fix: frontmost read switched to in-process `NSWorkspace.frontmostApplication`;
  `center-mouse.sh` poll interval 0.05 → 0.01 polling NSWorkspace in-process.
- Option B (merge cycle + center into ONE osascript) implemented:
  `cycle-window.sh` took an optional 2nd `center_mode` arg and centered the cursor
  itself; generator stopped chaining `center-mouse.sh` on the cycle path.
- IMPORTANT correction from that session: inside ONE running osascript, repeated
  System Events frontmost calls are ~1ms each; the ~80ms figure is single-shot,
  dominated by cold start + Apple Events warmup, NOT per-iteration.

### 2. Native rewrite (2026-08-01, current)
Cycling moved out of osascript entirely into `rcmdb cycle-window`, driving the
Accessibility C API directly; `cycle-window.sh` became a thin launcher and
Accessibility permission attached to the `rcmdb` binary. Native start is a few ms
and talks to the target app's AX server with no System Events middleman.
- Follow-up option not taken: port Center Mouse's osascript path to native AX too.

## Trials & Solutions (verbatim)

### From native window cycling (2026-08-01)
- Tried hot-swapping only cycle-window.sh (like the prior round-robin fix) → fails: launcher execs `rcmdb cycle-window`, a subcommand the installed 0.6.1 binary lacks → had to install the new binary too (cp over the brew Cellar file; `brew reinstall rcmdb` reverts).
- Tried `!contains("osascript")` launcher-guard test → failed: the launcher's own comment said "no osascript…" → reworded comment to "scripting-interpreter" so the guard means "never invokes osascript".
- No AX popup appeared → root cause: new rcmdb binary not TCC-trusted and code wasn't calling the prompting trust check → added `AXIsProcessTrustedWithOptions(prompt)` + `rcmdb accessibility` command; user granted, cycling confirmed working live.
- Local test binary is arm64 dev build over brew's Cellar path; official universal build ships via 0.6.2 release.

### From app-focus-latency (2026-07-16, osascript era)
- Tried timing single-shot System Events frontmost -> ~80-140ms -> assumed per-poll cost was 80ms -> WRONG. In-loop measurement (50x) showed ~1ms/call; the cost is cold-start + Apple Events warmup, not the query. Lesson: measure in-loop before attributing per-iteration cost.
- Considered true NSNotificationCenter didActivateApplication observer -> rejected: JXA run-loop + block observers are fragile and can hang on a per-keypress hot path -> used fast in-process NSWorkspace poll instead (effectively event-speed, robust).

## Status
- [x] osascript latency tweaks (v0.6.1)
- [x] Native `rcmdb cycle-window` + `accessibility` subcommands, thin launcher, objc2 deps
- [x] /backpressure quality loop (good-to-go + sec + write-test-audit) — 65 tests pass, clippy clean
- [x] User verified cycling works live after granting Accessibility
- [ ] Release 0.6.2 (universal + bottle + crates.io + tap); user re-grants Accessibility to the new binary
