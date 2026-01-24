# Quick Start Guide

## Running the App

The project is already built! Just run:

```bash
./run.sh
```

Or directly:

```bash
./target/release/karabiner-switch-key
```

## First Time Setup

1. **Make sure Karabiner-Elements is installed**
   ```bash
   brew install --cask karabiner-elements
   ```

2. **Run the TUI**
   ```bash
   ./run.sh
   ```

3. **Create your first binding**
   - Press `a` to add a new binding
   - Enter a key (e.g., `t` for rcmd+t)
   - Add a description (optional)
   - Press Tab to move to Actions field
   - Press `a` to add an action
   - Select type (App/URL/Shell)
   - Enter the target (e.g., "Terminal" for App, or "https://github.com" for URL)
   - Press Enter to save the action
   - Press Enter again to save the binding

4. **Save to Karabiner**
   - Press `s` to save and apply to Karabiner
   - Your bindings are now active!

## Example: Cycling Through Terminal Apps

1. Press `a` to add binding
2. Key: `t`
3. Description: `Terminal apps`
4. Tab to Actions, press `a`
5. Type: App, Target: `Terminal`, press Enter
6. Press `a` again for second action
7. Type: App, Target: `iTerm`, press Enter
8. Press `a` again for third action
9. Type: App, Target: `Warp`, press Enter
10. Press Enter to save binding
11. Press `s` to apply

Now `rcmd+t` will cycle: Terminal → iTerm → Warp → Terminal...

## Example: GitHub Tab Focus (with specific browser)

1. Press `a` to add binding
2. Key: `g`
3. Description: `GitHub`
4. Tab to Actions, press `a`
5. Type: URL (press `>` to change)
6. Tab to Target field
7. Enter: `https://github.com`
8. Tab to Match Type, press `>` until "domain"
9. Tab to Browser, press `>` to select "Chrome" (or any browser you want!)
10. Press Enter to save action
11. Press Enter to save binding
12. Press `s` to apply

Now `rcmd+g` will focus existing GitHub tab **in Chrome specifically** or open new one!

**Note**: If you leave Browser as "(use default)", it uses the browser from Settings tab.

## Key Bindings Reference

### Normal Mode
- `q` - Quit
- `Tab` - Switch tabs
- `s` - Save to Karabiner
- `a` - Add binding
- `e`/`Enter` - Edit binding
- `d` - Delete binding
- `j`/`k` or `↑`/`↓` - Navigate

### Editing Mode
- `Tab` - Next field
- `Shift+Tab` - Previous field
- `Enter` - Save/confirm
- `Esc` - Cancel

### In Action Editor
- `<`/`>` or `,`/`.` - Change type/options
- Type to enter text
- Backspace to delete

### In Actions List
- `a` - Add action
- `e` - Edit action
- `d` - Delete action
- `j`/`k` - Navigate
- `Shift+J`/`Shift+K` - Move up/down

## Troubleshooting

**TUI not starting?**
- Make sure you're running in a real terminal (not VS Code integrated terminal if it's acting up)
- Try: `TERM=xterm-256color ./run.sh`

**Keys not working in macOS?**
- Open Karabiner-Elements preferences
- Go to "Complex Modifications" tab
- Click "Add rule"
- You should see rules starting with `[switchkey]`
- Enable them if needed

**Want to rebuild?**
```bash
cargo clean
cargo build --release
```

## Config Location

Your bindings are saved to:
```
~/.config/karabiner-switch-key/config.toml
```

You can edit this file directly if you prefer!

## Next Steps

- See `README.md` for full documentation
- Check Settings tab (`Tab` key) to change default browser
- Experiment with URL match types (exact/domain/path/glob)
- Try shell commands for custom actions
