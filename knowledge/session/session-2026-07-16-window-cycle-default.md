---
slug: window-cycle-default
kind: coding
title: Window cycling default-on for single-app configs
description: Made cycle_windows default true for single App actions, gated on bundle_id at every set/persist/render/generate site.
keywords: [cycle_windows, window cycling, ActionEditor, has_bundle, bundle_id gating, to_action, single app default, karabiner generator, opt-out, flag gated on prerequisite]
created: 2026-07-16
modified: 2026-07-16
---

## window-cycle-default

Made window cycling the default for single-app configs (was opt-in false).

### cycle_windows semantics (prerequisite)
`cycle_windows` is only honored by the karabiner generator when an App action is a binding's **sole action** AND has a **non-empty `bundle_id`**. Multi-action cycling (`actions.len() > 1`) always ignores it (`allow_window_cycle=false`); a bundle-less App falls back to `open -a` and drops the flag.

### Change (src/app.rs)
- `ActionEditor::new()`: `cycle_windows` default flipped `false` → `true`.
- New `ActionEditor::has_bundle(&self) -> bool` = `bundle_id.as_ref().is_some_and(|b| !b.is_empty())`. Single authoritative predicate.
- `to_action()` now persists `cycle_windows: self.has_bundle() && self.cycle_windows` — a bundle-less App action can never persist a state the generator silently drops.
- `src/main.rs` (input handler) + `src/ui/editor.rs` (render) refactored to call `has_bundle()` instead of the duplicated `bundle_id.as_ref().map(|b|!b.is_empty()).unwrap_or(false)`.
- 16 T1 unit tests added in `#[cfg(test)] mod tests` (default, has_bundle boundaries incl. `Some("")`, to_action gating truth table, field preservation, Url/Shell dispatch, semantic-garbage bundle). Suite: 45 → 61 passing.

### Live config migration
`~/.config/karabiner-rcmd-binder/config.toml`: set `cycle_windows = true` on all 12 single-app bindings that lacked it (chrome/firefox already had it). 14 bindings total now cycle. Done via one-shot python script (removed after use); only touched `[[bindings.actions]]` app blocks, left `[[cached_apps]]` untouched.

### Docs
- README.md: `cycle_windows` example annotated "default on; set false to opt out".
- CHANGELOG.md: new `[Unreleased]` Changed entry.
- AGENTS.md good-to-go axis "Flag gated on prerequisite" tightened: gate must live at every set/persist site (input handler AND `to_action` AND `new` default), not just the input handler.

## Trials & Solutions
- Tried flipping only `new()` default → good-to-go project axis flagged a regression: a freeform app with no autocomplete match (bundle_id None) would serialize `cycle_windows = true`, a state the generator drops → added the `has_bundle()` gate in `to_action()`.
- `cargo build` wrapper prints "0 crates compiled" even after edits (custom wrapper), but `cargo test` recompiles and is the reliable signal.
- 8 clippy warnings exist but all in pre-existing untouched files (app_discovery.rs, keycodes.rs, validation.rs); none in changed files — out of scope.

## pending
- [x] Default flip + gating + tests + config migration + docs
- [ ] Not committed — user has not requested a commit yet. Changed: AGENTS.md, CHANGELOG.md, README.md, src/app.rs, src/main.rs, src/ui/editor.rs.
