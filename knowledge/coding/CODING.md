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

## Code Quality

### Dead Code Elimination
- Remove unused functions immediately
- Use compiler warnings as guide
- Zero warnings policy for releases

### Simplification
- Prefer early returns over nested if-else
- Use guard clauses for validation
- Reduce nesting depth (target: 2-3 levels max)
