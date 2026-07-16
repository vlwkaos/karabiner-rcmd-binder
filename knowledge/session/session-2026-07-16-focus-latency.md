---
slug: app-focus-latency
kind: coding
title: Speeding up app focus / window cycling (NSWorkspace vs System Events)
description: Removed System Events Apple-Events round-trips from the keypress hot path in cycle-window.sh and center-mouse.sh.
keywords: [app focus latency, window cycling speed, osascript cold start, NSWorkspace frontmostApplication, System Events whose frontmost, center-mouse poll, JXA performance, cycle-window.sh, center-mouse.sh, Apple Events overhead]
created: 2026-07-16
modified: 2026-07-16
---

## app-focus-latency

Investigated perceived delay on first app focus + window cycle. Fix = drop System Events from the hot-path frontmost check.

### Where the delay comes from (measured on this machine)
A single-app keypress runs, via Karabiner `shell_command`: `cycle-window.sh <bundle> && center-mouse.sh <bundle> <mode>` — TWO separate `osascript` (JXA) processes, serial.
- osascript cold start (2 ObjC imports): ~50-80ms.
- Single-shot frontmost check: `System Events processes.whose({frontmost:true})` ~100-140ms vs `$.NSWorkspace.sharedWorkspace.frontmostApplication.bundleIdentifier.js` ~70-90ms. System Events adds a separate-process Apple-Events round-trip.
- IMPORTANT correction: inside ONE running osascript, repeated System Events frontmost calls are ~1ms each (50 calls ≈ 0.08s). The ~80ms figure is single-shot, dominated by cold start + Apple Events warmup, NOT per-iteration. Do not claim per-poll System Events is 80ms.
- Dominant fixed cost = the TWO osascript cold starts (~100-160ms combined). Only removable by merging both scripts into one osascript (option B, not done — user declined).

### What changed (src/scripts/mod.rs)
- `cycle-window.sh` + `center-mouse.sh`: frontmost read now `$.NSWorkspace.sharedWorkspace.frontmostApplication.bundleIdentifier.js` (in-process ObjC) instead of `System Events whose({frontmost:true})`. Saves ~30-50ms/press on the single-shot check.
- `center-mouse.sh`: poll interval 0.05 -> 0.01, polling NSWorkspace in-process (near-free) instead of System Events; detects activation up to ~40ms sooner. AX (System Events windows[0] position/size) kept for the ONE-TIME geometry read only.
- Safe-default failure paths: `frontmostApplication` nil or bundle-less -> `.js` throws -> caught -> frontBundle=null -> cycle-window takes LAUNCH branch, center-mouse polls then returns (no warp).

### Verification
- 61 tests pass (incl. shebang/no-baked-path guards for both scripts).
- Runtime: frontmost detection decides correctly (frontmost->cycle branch, other->launch branch) via non-destructive probe.
- AX geometry read NOT verifiable from a shell without Accessibility permission (errors -1701 / hiservices-xpcservice). Works under Karabiner (has AX perm). That code path is unchanged from before.

## Trials & Solutions
- Tried timing single-shot System Events frontmost -> ~80-140ms -> assumed per-poll cost was 80ms -> WRONG. In-loop measurement (50x) showed ~1ms/call; the cost is cold-start + Apple Events warmup, not the query. Lesson: measure in-loop before attributing per-iteration cost.
- Considered true NSNotificationCenter didActivateApplication observer -> rejected: JXA run-loop + block observers are fragile and can hang on a per-keypress hot path -> used fast in-process NSWorkspace poll instead (effectively event-speed, robust).

### B (merge) — implemented
Cycling + centering now runs in ONE osascript. `cycle-window.sh` takes an optional 2nd arg `center_mode` and, after focus/cycle, centers the cursor itself (imports CoreGraphics). Generator: the `allow_window_cycle && cycle_windows` branch emits `cycle-window.sh <id> [mode]` and NO longer chains `center-mouse.sh`. The non-cycling path (`open -b`) still chains `center-mouse.sh` (open is native, so only one osascript there anyway). Saves ~50-80ms (one JXA start) on the cycle+center path — which is most bindings now that cycling defaults on.
- Cycle-branch centering targets the held `next` window reference (post-AXRaise), avoiding a windows[0] re-query race. Launch-branch waits for frontmost then centers on windows[0].
- Cost: centering JS is duplicated across cycle-window.sh and center-mouse.sh (two separate osascript heredocs — can't share). Accepted.
- Test contract change: `test_multi_monitor_only_with_cycle_window_base` now asserts single cycle-window.sh + mode arg, NO center-mouse.sh, NO ` && `.

## release-v0.6.1 (2026-07-16)
Released 0.6.0 -> 0.6.1. GitHub release + tag + Homebrew tap DONE. crates.io FAILED (expired token).

### RELEASE GOTCHAS (do not repeat)
- The project's `scripts/release.sh` + `package.sh` are STALE: they build arm64-only `rcmdb-VERSION-macos-arm64.tar.gz` with Apple codesign and NO bottle. They did NOT build v0.5.0+ (which are `-darwin-universal` + `.all.bottle.tar.gz`). DO NOT use them.
- The correct release path is the GLOBAL skill script (matches AGENTS `release.flow: rust`):
  `~/.claude/skills/rust-release/release.sh <VERSION> rcmdb vlwkaos/karabiner-rcmd-binder /Users/eliot/ws-ps/homebrew-tap`
  It builds a universal binary via lipo, GPG-signs (NO Apple codesign at all), makes the bottle, pushes main+tag, creates the GH release, runs `cargo publish`, and rewrites+pushes the tap formula.
- No Apple codesigning identity on this machine (0 valid, CODESIGN_IDENTITY unset). The global script never codesigns, so GPG-only just works. The project scripts would hard-fail here.
- Homebrew tap lives at `/Users/eliot/ws-ps/homebrew-tap` (NOT `../homebrew-tap`). Pass it as arg 4.
- crates.io token in `~/.cargo/credentials.toml` is EXPIRED/invalid -> `cargo publish` returned 403 Forbidden. The script runs cargo publish AFTER the GH release+tag push, so a bad token leaves a partial release (GH done, crates.io + Homebrew not). This time Homebrew was completed manually.

## pending
- [x] A + C + B implemented, verified (61 tests). CHANGELOG [0.6.1] dated. Two commits (feat + perf) on main.
- [x] GitHub release v0.6.1 live (tarball+asc+sha256+bottle). main + tag v0.6.1 pushed. Homebrew formula updated to 0.6.1 + pushed (tarball sha d0e029a1..., bottle sha 8080203560...).
- [ ] crates.io publish BLOCKED on expired token. Fix: `cargo login <new-token>` (from https://crates.io/settings/tokens), then `cargo publish` (tree clean, 0.6.1 committed). No other release step depends on it.
