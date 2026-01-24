# Coding Best Practices

Project-specific patterns, standards, and conventions.

## Patterns

### Mode Detection
Field-based INPUT vs NAV mode determination.

```rust
// src/app.rs
pub fn is_input_mode(&self) -> bool {
    match &self.binding_editor {
        Some(editor) => match &editor.action_editor {
            Some(ae) => ae.field == ActionEditorField::Target,
            None => matches!(editor.field, EditorField::Key | EditorField::Description),
        },
        None => false,
    }
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
- osascript: ~450ms (blocking)
- read_dir: ~32ms (fast)
- Solution: background thread + loading indicator

### UI Rendering
- 50ms poll = 20 FPS max
- No expensive operations in render path
