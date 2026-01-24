# karabiner-rcmd-binder

TUI for easily configuring Karabiner-Elements right_command key bindings with support for app launching, URL focusing, and action cycling.

## Features

- **Simple TUI**: Easy-to-navigate terminal interface
- **Action Types**: 
  - App: Launch/focus applications
  - URL: Open URLs with smart browser tab focusing
  - Shell: Run custom shell commands
- **Cycling Support**: Assign multiple actions to the same key (cycles in order)
- **Browser Tab Matching**: exact, domain, path, or glob patterns
- **Multi-browser Support**: Firefox, Chrome, Safari, Arc, Edge (set per-action! See [BROWSER-SUPPORT.md](BROWSER-SUPPORT.md))
- **Key Autocomplete**: Type partial key names, get suggestions
- **Safe Updates**: Automatic backups (keeps last 3) before modifying karabiner.json

## Installation

### 1. Install Rust (if not already installed)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 2. Build and Run

```bash
cd karabiner-rcmd-binder
cargo run --release
```

Or install globally:

```bash
cargo install --path .
# Then run from anywhere:
karabiner-rcmd-binder
```

## Usage

### TUI Navigation

**Normal Mode:**
- `q` - Quit
- `Tab` - Switch between Bindings/Settings tabs
- `s` - Save configuration to karabiner.json

**Bindings Tab:**
- `j`/`k` or `↑`/`↓` - Navigate list
- `a` - Add new binding
- `e` or `Enter` - Edit selected binding
- `d` - Delete selected binding

**Editing Mode:**
- `Tab` - Next field
- `Shift+Tab` - Previous field
- `Enter` - Save/confirm
- `Esc` - Cancel

**In Action Editor:**
- `<`/`>` or `,`/`.` - Change type/match type/browser
- Type to enter text in input fields
- `a` - Add action (when in Actions field)
- `e` - Edit action (when in Actions field)
- `d` - Delete action (when in Actions field)
- `j`/`k` - Navigate actions
- `Shift+J`/`Shift+K` - Move action up/down
- **Note**: For URL actions, each can have its own browser override!

### Configuration

Your configuration is stored in `~/.config/karabiner-rcmd-binder/config.toml`:

```toml
[settings]
default_browser = "firefox"

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

### How It Works

1. **Edit bindings** in the TUI
2. **Press `s`** to save
3. The app:
   - Saves your config to `~/.config/karabiner-rcmd-binder/config.toml`
   - Creates a backup of `karabiner.json` (timestamped, keeps last 3)
   - Generates Karabiner rules with `[rcmdb]` prefix
   - Installs helper scripts to `~/.config/karabiner-rcmd-binder/scripts/`
   - Updates `~/.config/karabiner/karabiner.json`
4. **Karabiner automatically reloads** the config
5. **Use your bindings**: `rcmd+<key>` triggers the action(s)

### Example: Cycling Through Apps

Add a binding with key `t` and multiple App actions:
1. Terminal
2. iTerm
3. Warp

Now pressing `rcmd+t` will cycle: Terminal → iTerm → Warp → Terminal → ...

### Example: Smart URL Opening with Browser Override

Add a binding with key `g` and URL action:
- **Target**: `https://github.com/notifications`
- **Match**: `domain`
- **Browser**: `chrome` (overrides default!)

Now pressing `rcmd+g` will:
- If Chrome has a tab open with `github.com/*`, focus that tab
- Otherwise, open the URL in a new Chrome tab
- **Always uses Chrome** for this action, regardless of default browser setting

### Example: Multiple URLs with Different Browsers

Add a binding with key `w` and multiple URL actions:
1. `https://mail.google.com` - Browser: Chrome
2. `https://github.com` - Browser: Firefox
3. `https://linear.app` - Browser: Arc

Now pressing `rcmd+w` cycles through Gmail (Chrome) → GitHub (Firefox) → Linear (Arc)

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

- macOS
- [Karabiner-Elements](https://karabiner-elements.pqrs.org/) installed
- Rust (for building)

## FAQ

**Q: Can I use different browsers for different URLs?**  
**A: YES!** Each URL action has its own browser field. When editing a URL action, Tab to the Browser field and press `<`/`>` to select a specific browser. This overrides the default browser from Settings. See `BROWSER-SUPPORT.md` for details.

**Q: What's the difference between default browser and per-action browser?**  
**A:** 
- **Default browser** (in Settings tab): Used when URL action's browser is "(use default)"
- **Per-action browser**: Overrides default for that specific URL action
- You can mix both! Some URLs use default, others use specific browsers.

## Troubleshooting

**Keys not working?**
- Check Karabiner-Elements is running
- Open Karabiner-Elements preferences → Complex Modifications
- Verify `[rcmdb]` rules are present

**Browser tab not focusing?**
- Firefox has limited tab detection, works best with exact URL matches
- Chrome/Safari/Arc/Edge have full tab search support

**Build errors?**
- Make sure you have the latest Rust: `rustup update`
- Try `cargo clean` then `cargo build --release`

## License

MIT
