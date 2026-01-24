# Browser Configuration Examples

## ✅ YES - You Can Set Different Browsers Per URL!

Each URL action has its own independent browser setting.

## Visual Example 1: Work Tabs in Different Browsers

```
Binding: rcmd+w (Work tools)
├─ Action 1: Gmail → Chrome
├─ Action 2: GitHub → Firefox
└─ Action 3: Figma → Arc

Press rcmd+w once  → Gmail opens in Chrome
Press rcmd+w twice → GitHub opens in Firefox
Press rcmd+w third → Figma opens in Arc
```

### Config File
```toml
[[bindings]]
key = "w"
description = "Work tools"

[[bindings.actions]]
type = "url"
target = "https://mail.google.com"
match = "domain"
browser = "chrome"    # ← Chrome for Gmail

[[bindings.actions]]
type = "url"
target = "https://github.com"
match = "domain"
browser = "firefox"   # ← Firefox for GitHub

[[bindings.actions]]
type = "url"
target = "https://figma.com"
match = "domain"
browser = "arc"       # ← Arc for Figma
```

## Visual Example 2: Personal vs Work Email

```
Binding: rcmd+e (Email)
├─ Action 1: Personal Gmail → Firefox
└─ Action 2: Work Gmail → Chrome

Press rcmd+e once  → Personal email in Firefox
Press rcmd+e twice → Work email in Chrome
```

### Config File
```toml
[[bindings]]
key = "e"
description = "Email accounts"

[[bindings.actions]]
type = "url"
target = "https://mail.google.com/mail/u/0"
match = "exact"
browser = "firefox"   # ← Personal in Firefox

[[bindings.actions]]
type = "url"
target = "https://mail.google.com/mail/u/1"
match = "exact"
browser = "chrome"    # ← Work in Chrome
```

## Visual Example 3: Testing Across Browsers

```
Binding: rcmd+l (Localhost test)
├─ Action 1: localhost:3000 → Chrome
├─ Action 2: localhost:3000 → Firefox
└─ Action 3: localhost:3000 → Safari

Press rcmd+l to cycle through same URL in different browsers!
```

### Config File
```toml
[[bindings]]
key = "l"
description = "Test localhost in all browsers"

[[bindings.actions]]
type = "url"
target = "http://localhost:3000"
match = "exact"
browser = "chrome"

[[bindings.actions]]
type = "url"
target = "http://localhost:3000"
match = "exact"
browser = "firefox"

[[bindings.actions]]
type = "url"
target = "http://localhost:3000"
match = "exact"
browser = "safari"
```

## How to Set Browser in TUI

### Step-by-Step
1. Add or edit a binding (press `a` or `e`)
2. Navigate to Actions field (press `Tab`)
3. Add a URL action (press `a`)
4. Change type to URL (press `>`)
5. Tab to Target, enter URL
6. Tab to Match Type, select type
7. **Tab to Browser** ← HERE!
8. Press `<` or `>` to cycle:
   ```
   (use default) → Firefox → Chrome → Safari → Arc → Edge → (use default)
   ```
9. Press Enter to save

## Browser Field Options

| Display | Meaning |
|---------|---------|
| `(use default)` | Uses browser from Settings tab |
| `Firefox` | Always opens in Firefox |
| `Chrome` | Always opens in Chrome |
| `Safari` | Always opens in Safari |
| `Arc` | Always opens in Arc |
| `Edge` | Always opens in Edge |

## Mix and Match Strategy

### Strategy 1: One Default, Some Overrides
```toml
[settings]
default_browser = "firefox"  # Most URLs use Firefox

[[bindings]]
key = "g"

[[bindings.actions]]
type = "url"
target = "https://github.com"
match = "domain"
# No browser specified → uses Firefox (default)

[[bindings.actions]]
type = "url"
target = "https://mail.google.com"
match = "domain"
browser = "chrome"  # Override: Gmail always in Chrome
```

### Strategy 2: All Explicit
```toml
[settings]
default_browser = "firefox"  # Rarely used

[[bindings]]
key = "s"

[[bindings.actions]]
type = "url"
target = "https://slack.com"
browser = "chrome"  # Explicit

[[bindings.actions]]
type = "url"
target = "https://linear.app"
browser = "arc"     # Explicit
```

## Real-World Examples

### Developer Setup
```toml
# GitHub in Arc (for clean UI)
[[bindings.actions]]
type = "url"
target = "https://github.com"
browser = "arc"

# Localhost in Chrome (for DevTools)
[[bindings.actions]]
type = "url"
target = "http://localhost:3000"
browser = "chrome"

# Docs in Safari (for Reading Mode)
[[bindings.actions]]
type = "url"
target = "https://developer.mozilla.org"
browser = "safari"
```

### Designer Setup
```toml
# Figma in Arc
[[bindings.actions]]
type = "url"
target = "https://figma.com"
browser = "arc"

# Dribbble in Safari
[[bindings.actions]]
type = "url"
target = "https://dribbble.com"
browser = "safari"

# Client sites in Chrome (for testing)
[[bindings.actions]]
type = "url"
target = "https://client-site.com"
browser = "chrome"
```

## Summary

✅ **Each URL action** = independent browser setting  
✅ **Override default** = per-action browser beats Settings browser  
✅ **Mix strategies** = some use default, some override  
✅ **TUI support** = Tab to Browser field, press `<`/`>`  
✅ **Config format** = `browser = "chrome"` (or omit for default)

**The answer is YES** - full per-action browser control! 🎉
