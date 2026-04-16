# karabiner-rcmd-binder

TUI for easily configuring Karabiner-Elements right_command key bindings with support for app launching, URL focusing, and action cycling.

<img width="750" height="809" alt="image" src="https://github.com/user-attachments/assets/7a382a77-228c-4649-b8df-06e25dbda7fb" />

## Features

- **Nav/Edit Mode System**: Clear two-mode interaction - shortcuts in Nav mode, text input in Edit mode
- **Dynamic Bindings**: Auto-suggestions for unassigned rcmd+{letter} keys based on installed apps
- **Action Types**: App launch, URL with smart tab focusing, or shell commands
- **Action Cycling**: Multiple actions per key cycle in order
- **Browser Control**: Per-action browser override with tab matching (exact, domain, path, glob)
- **App Discovery**: Autocomplete from running + installed apps with 30-day cache
- **Center Mouse on Focus**: Automatically moves mouse to the center of the focused app's window
- **Safe Updates**: Automatic backups (keeps last 3) before modifying karabiner.json

## Installation

### Homebrew (Recommended)

```bash
brew tap vlwkaos/tap
brew install rcmdb
rcmdb
```

### Manual Download

Download from [releases](https://github.com/vlwkaos/karabiner-rcmd-binder/releases):

```bash
tar -xzf rcmdb-0.2.1-macos-arm64.tar.gz
cd rcmdb-0.2.1-macos-arm64
./install.sh
rcmdb
```

### Build from Source

```bash
git clone https://github.com/vlwkaos/karabiner-rcmd-binder
cd karabiner-rcmd-binder
cargo build --release
./target/release/rcmdb
```

## Usage

### Quick Start: Nav/Edit Modes

The TUI uses **two modes** (like vim):
- **Nav Mode** (cyan border, `[NAV]`): Shortcuts active (s/a/e/d), navigate with j/k
- **Edit Mode** (green border, `[EDIT]`): Text input active, type freely

**Toggle**: Press `Enter` to start editing, `Enter` again to finish

### Keyboard Shortcuts

**Global** (all modes):
- `q` - Quit
- `Tab` / `Shift+Tab` - Switch tabs or navigate fields
- `s` - Save to karabiner.json

**Bindings Tab** (Nav mode):
- `j`/`k` or `↑`/`↓` - Navigate bindings
- `a` - Add new binding
- `e` or `Enter` - Edit selected binding
- `d` - Delete binding

**Binding Editor**:
- Nav mode: `Enter` to edit field, `s` to save, `Tab` to switch fields
- Edit mode: Type text, `Enter` to finish, `Esc` to cancel

**Actions Field**:
- `a` - Add action
- `e` / `Enter` - Edit action
- `d` - Delete action
- `j`/`k` - Navigate actions
- `J`/`K` - Move action up/down

**Action Editor**:
- Nav mode: `Enter` to edit Target, `←`/`→` to cycle Type/Browser, `s` to save
- Edit mode: Type target (app name/URL/command), `Enter` to finish

**Tip**: Status bar (bottom) shows all available shortcuts for current context

### Settings Tab

Switch to the Settings tab (`Tab`) to configure global options:

- **Anchor Key** (`</>`): `Right Command` or `Right Option`
- **Default Browser** (`</>`): Browser used when a URL action has no browser override
- **Center Mouse on App Focus** (`space`): When enabled, every app-launch binding moves the mouse to the center of the focused window after the app comes to the foreground. Polls up to 0.5s for the app to become frontmost - no fixed delay.

> **Permission**: The first time Center Mouse fires, macOS will prompt to grant **Accessibility** access to `osascript` (Privacy & Security > Accessibility). This is required to read window positions. Karabiner-Elements itself already needs Accessibility, but `osascript` is a separate binary and needs its own grant.

### Configuration

Your configuration is stored in `~/.config/karabiner-rcmd-binder/config.toml`:

```toml
[settings]
default_browser = "firefox"
center_mouse = true   # optional, omitted when false

[[bindings]]
key = "t"
description = "Terminal apps"

[[bindings.actions]]
type = "app"
target = "Terminal"

[[bindings.actions]]
type = "app"
target = "iTerm"

[[bindings]]
key = "g"
description = "GitHub"

[[bindings.actions]]
type = "url"
target = "https://github.com"
match = "domain"
browser = "chrome"
```

### Dynamic Bindings

On startup, the TUI auto-generates **suggestions** for unassigned `rcmd+{letter}` keys:
- Appears in **darker gray** below your saved bindings
- Matches first installed app starting with that letter (rcmd+s → Slack, rcmd+c → Chrome)
- **Edit a suggestion** (press `e`) to convert it to a saved binding
- **Delete a suggestion** (press `d`) to remove it (doesn't affect saved bindings)
- Regenerates automatically when gaps appear

### How It Works

1. **Edit bindings** in the TUI (Nav mode: shortcuts, Edit mode: text input)
2. **Press `s`** to save
3. Creates backup of `karabiner.json` and updates with `[rcmdb]` rules
4. **Karabiner auto-reloads** - your bindings work immediately
5. **Use your bindings**: `rcmd+<key>` triggers the action(s)

### Examples

**Cycling Apps**: Add multiple App actions to one key
```
rcmd+t → Terminal → iTerm → Warp → (cycles)
```

**Smart URL with Browser**: URL action with browser override
```
rcmd+g → https://github.com (Chrome)
  - Focuses existing github.com tab if open
  - Opens in Chrome regardless of default browser
```

**Multi-Browser URLs**: Different browsers per URL
```
rcmd+w → Gmail (Chrome) → GitHub (Firefox) → Linear (Arc) → (cycles)
```

**Center Mouse**: Enable in Settings tab, then every app binding warps the cursor to the window center
```
rcmd+t → Terminal focuses → mouse moves to center of Terminal window
```

## Match Types for URLs

- **exact**: Match full URL exactly
- **domain**: Match any URL on the same domain (e.g., `github.com/*`)
- **path**: Match domain + path (ignores query params)
- **glob**: Simple wildcard matching with `*`

## Files & Locations

- **Config**: `~/.config/karabiner-rcmd-binder/config.toml`
- **Scripts**: `~/.config/karabiner-rcmd-binder/scripts/`
- **Karabiner**: `~/.config/karabiner/karabiner.json`
- **Backups**: `~/.config/karabiner/karabiner.json.rcmdb-backup-YYYYMMDD-HHMMSS`

## Requirements

- [Karabiner-Elements](https://karabiner-elements.pqrs.org/) installed
- Rust (for building)

## FAQ

**Q: What are dynamic bindings?**
Suggestions for unassigned rcmd+{letter} keys (shown in gray). Edit them to convert to saved bindings.

**Q: How do Nav/Edit modes work?**
Nav mode (cyan): shortcuts active. Edit mode (green): text input active. Press Enter to toggle.

**Q: Can I use different browsers for different URLs?**
Yes! Each URL action has its own browser field (Tab to Browser, `←`/`→` to change).

**Q: What is Center Mouse on App Focus?**
When enabled in Settings, every app-launch binding automatically moves the mouse to the center of the focused window after the app comes to the foreground. Useful when using keyboard-driven app switching without touching the mouse. Requires Accessibility permission for `osascript` (macOS will prompt once).

## Troubleshooting

**Keys not working?**
- Check Karabiner-Elements is running
- Open Karabiner-Elements preferences → Complex Modifications
- Verify `[rcmdb]` rules are present

**Browser tab not focusing?**
- Firefox has limited tab detection, works best with exact URL matches
- Chrome/Safari/Arc/Edge have full tab search support

**Center Mouse not working?**
- macOS will prompt for **Accessibility** access the first time - approve it in Privacy & Security > Accessibility
- Only works for App bindings that have a bundle ID (set via autocomplete). Apps added by name only are skipped.
- If the app takes more than 0.5s to come to the foreground (cold launch), the poll times out silently
- Re-pressing the binding while the script is still polling cancels the previous attempt and starts fresh

**Build errors?**
- Make sure you have the latest Rust: `rustup update`
- Try `cargo clean` then `cargo build --release`

## License

MIT
