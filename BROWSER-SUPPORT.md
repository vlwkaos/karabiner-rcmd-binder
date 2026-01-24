# Browser Support Guide

## Per-Action Browser Selection

**YES! You can absolutely set different browsers for different URL actions!**

Each URL action has its own browser setting that **overrides** the default browser in Settings.

## How It Works

### Option 1: Use Default Browser (from Settings tab)
- In Settings tab, set your default browser (e.g., Firefox)
- When adding URL actions, leave browser as "(use default)"
- All URLs without specific browser will use Firefox

### Option 2: Override Per URL Action
- When adding a URL action, Tab to the Browser field
- Press `<`/`>` to cycle through browsers:
  - Firefox
  - Chrome
  - Safari
  - Arc
  - Edge
  - (use default) ← back to default
- This URL will **always** use the selected browser

## Example: Multiple URLs with Different Browsers

```toml
[[bindings]]
key = "w"
description = "Work tools"

# This opens in Chrome specifically
[[bindings.actions]]
type = "url"
target = "https://mail.google.com"
match = "domain"
browser = "chrome"

# This opens in Firefox specifically
[[bindings.actions]]
type = "url"
target = "https://github.com"
match = "domain"
browser = "firefox"

# This opens in Arc specifically
[[bindings.actions]]
type = "url"
target = "https://linear.app"
match = "domain"
browser = "arc"
```

Now pressing `rcmd+w` cycles through:
1. Gmail in Chrome
2. GitHub in Firefox  
3. Linear in Arc

## TUI Steps for Per-Action Browser

1. Add/edit a binding
2. Tab to Actions field
3. Press `a` to add URL action
4. Select type: URL (press `>`)
5. Tab to Target: enter URL
6. Tab to Match Type: select match type
7. **Tab to Browser: Press `<`/`>` to select specific browser!**
8. Press Enter to save

## Supported Browsers

| Browser | Status | Tab Focusing |
|---------|--------|--------------|
| Firefox | ✅ | Limited (best with exact URLs) |
| Chrome | ✅ | Full AppleScript support |
| Safari | ✅ | Full AppleScript support |
| Arc | ✅ | Full AppleScript support |
| Edge | ✅ | Full AppleScript support |

## Browser Field Behavior

- **Empty/Default**: Uses browser from Settings tab
- **Specific browser**: Always uses that browser for this URL
- **Cycling**: Press `>` repeatedly to cycle through all browsers, then back to "(use default)"

## Use Cases

### Personal vs Work
```toml
# Personal email - Firefox
[[bindings.actions]]
type = "url"
target = "https://mail.google.com/mail/u/0"
browser = "firefox"

# Work email - Chrome
[[bindings.actions]]
type = "url"
target = "https://mail.google.com/mail/u/1"
browser = "chrome"
```

### Different Tools in Different Browsers
```toml
# Development tools - Arc
[[bindings.actions]]
type = "url"
target = "https://github.com"
browser = "arc"

# Design tools - Chrome
[[bindings.actions]]
type = "url"
target = "https://figma.com"
browser = "chrome"

# Communication - Safari
[[bindings.actions]]
type = "url"
target = "https://slack.com"
browser = "safari"
```

### Testing Across Browsers
```toml
[[bindings]]
key = "l"
description = "Localhost in different browsers"

[[bindings.actions]]
type = "url"
target = "http://localhost:3000"
browser = "chrome"

[[bindings.actions]]
type = "url"
target = "http://localhost:3000"
browser = "firefox"

[[bindings.actions]]
type = "url"
target = "http://localhost:3000"
browser = "safari"
```

## Config File Format

```toml
[[bindings.actions]]
type = "url"
target = "https://example.com"
match = "domain"
browser = "chrome"  # ← This line is optional!
```

**If omitted**: Uses default browser from Settings  
**If specified**: Uses this browser specifically

## Summary

✅ **YES** - Each URL action can have its own browser  
✅ **Flexible** - Mix and match browsers per action  
✅ **Override** - Per-action browser overrides Settings default  
✅ **Optional** - Leave blank to use default browser  
✅ **TUI Support** - Full support in the editor with `<`/`>` keys
