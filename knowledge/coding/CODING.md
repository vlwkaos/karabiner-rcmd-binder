# Coding Best Practices

Project-specific patterns, standards, and conventions.

## Patterns

### Early Returns Pattern
Use early returns to reduce nesting and improve readability.

```rust
// Prefer this:
if !output.status.success() {
    return Err(anyhow::anyhow!("Failed"));
}
let bundle_id = parse(output);
if bundle_id.is_empty() {
    return Err(anyhow::anyhow!("Empty"));
}
Ok(bundle_id)

// Instead of this:
if output.status.success() {
    let bundle_id = parse(output);
    if !bundle_id.is_empty() {
        Ok(bundle_id)
    } else {
        Err(anyhow::anyhow!("Empty"))
    }
} else {
    Err(anyhow::anyhow!("Failed"))
}
```

### Command Key Guards
All command keys must check field before executing.

```rust
// In action editor
KeyCode::Char('<') | KeyCode::Char(',') => {
    if action_editor.field != ActionEditorField::Target {
        // Execute command
    }
    // Otherwise: character goes to text field
}
```

### Autocomplete Render Order
Autocomplete MUST render last to appear on top.

```rust
// src/ui/editor.rs - at END of draw function
if app.show_autocomplete {
    draw_autocomplete(frame, app, autocomplete_area);
}
```

### Background Task Pattern
Use mpsc channel for async operations.

```rust
// Setup
let (tx, rx) = mpsc::channel();

// Spawn
thread::spawn(move || {
    let result = expensive_operation();
    let _ = tx.send(result);
});

// Poll (non-blocking)
if let Ok(result) = rx.try_recv() {
    app.handle_result(result);
}
```

### Flag Gated on Prerequisite
A flag the generator only honors under a prerequisite must be gated at **every**
set/persist site — the `new()` default, the input handler, AND `to_action()` — not
just the input handler. Otherwise a state the generator silently drops can be
serialized. Example: `cycle_windows` is only honored when an App is a binding's sole
action with a non-empty `bundle_id`; `to_action()` persists
`self.has_bundle() && self.cycle_windows` so a bundle-less App can never persist a
dropped state. Use one authoritative predicate (`has_bundle()`), not a duplicated
`bundle_id.as_ref().map(|b|!b.is_empty()).unwrap_or(false)` at each call site.

**Trials & Solutions (window-cycle-default, 2026-07-16):**
- Tried flipping only `new()` default → good-to-go project axis flagged a regression: a freeform app with no autocomplete match (bundle_id None) would serialize `cycle_windows = true`, a state the generator drops → added the `has_bundle()` gate in `to_action()`.
- `cargo build` wrapper prints "0 crates compiled" even after edits (custom wrapper), but `cargo test` recompiles and is the reliable signal.
- 8 clippy warnings exist but all in pre-existing untouched files (app_discovery.rs, keycodes.rs, validation.rs); none in changed files — out of scope.

## Architecture

### Event Loop
- 50ms poll timeout for responsive UI
- Single main thread for all rendering
- Background threads for slow operations (app discovery)

### State Management
- All state in `App` struct
- Sub-editors: `BindingEditor`, `ActionEditor`
- No global state

### UI Layers
1. Layout (tabs, status bar)
2. Content (lists, forms)
3. Overlays (autocomplete) - rendered last

## Standards

### Naming
- Files: `snake_case.rs`
- Types: `PascalCase`
- Functions: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE`

### Error Handling
- Use `anyhow::Result` for fallible operations
- `.context("message")` for error context
- Silent failures OK for optional features (discovery)

### Serde Conventions
- `#[serde(rename_all = "lowercase")]` for enums
- `#[serde(default)]` for optional fields
- `#[serde(skip_serializing_if = "Option::is_none")]` for cleaner TOML

## Testing

### Unit Tests
- In-module `#[cfg(test)]` blocks
- Focus on pure logic (generators, parsers)

### Manual Testing
- `./run.sh` for quick iteration
- `cargo build --release` for final binary

## Performance

### App Discovery
- osascript: ~450ms for running apps (background thread)
- plutil: varies by app count (optimized: limit 150 apps)
- Skip /System/Applications (slow, rarely needed)
- Solution: background thread + loading indicator
- Total time: ~0.5-1s (was 2-3s)

### UI Rendering
- 50ms poll = 20 FPS max
- No expensive operations in render path

### Window Cycling (keypress hot path)
- Current: native `rcmdb cycle-window` subcommand on the Accessibility C API —
  starts in a few ms, no interpreter start, no System Events middleman, sets
  `QOS_CLASS_USER_INTERACTIVE`. See `knowledge/coding/Native window cycling.md`.
- SUPERSEDES the earlier osascript/JXA + System Events approach (cold start ~100ms+,
  Apple Events queue backed up under load → cycling could get stuck). `cycle-window.sh`
  is now only a thin PATH-resolving launcher. History in
  `knowledge/history/Window Cycling Performance.md`.
- Accessibility (TCC) now attaches to the `rcmdb` binary, not osascript; a CLI invoked
  from a script gets no automatic prompt and untrusted AX calls fail silently — see
  `knowledge/domain/Accessibility permission model.md`.

## Code Quality

### Dead Code Elimination
- Remove unused functions immediately
- Use compiler warnings as guide
- Zero warnings policy for releases

### Simplification
- Prefer early returns over nested if-else
- Use guard clauses for validation
- Reduce nesting depth (target: 2-3 levels max)
