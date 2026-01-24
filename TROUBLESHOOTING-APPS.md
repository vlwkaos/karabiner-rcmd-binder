# Troubleshooting: App Not Opening

## Problem: "My app doesn't open when I press the key!"

This usually means the app name in your binding doesn't match the actual application name on your Mac.

## Solution: Find the Exact App Name

### Method 1: Using Terminal
```bash
# List all applications
ls /Applications/

# Search for your app (case-insensitive)
ls /Applications/ | grep -i "firefox"
```

### Method 2: Using Finder
1. Open Finder
2. Go to Applications folder
3. Find your app
4. The name you see (including .app) is what you need
5. **Use the name WITHOUT .app extension**

### Common Examples

| You might type | Actual name | Why |
|----------------|-------------|-----|
| Firefox | Firefox Developer Edition | Developer edition has different name |
| VS Code | Visual Studio Code | Full name required |
| Chrome | Google Chrome | Full name required |
| Photoshop | Adobe Photoshop 2024 | Year might be in name |
| Word | Microsoft Word | Full name required |

## Test Your App Name

Before adding to the TUI, test in terminal:

```bash
open -a 'YourAppName'
```

If it works, that's the correct name to use in the binding!

### Examples:
```bash
# Works
open -a 'Firefox Developer Edition'

# Fails
open -a 'Firefox'
```

## Fix Your Binding

1. Run `./run.sh`
2. Press 'e' to edit your binding
3. Navigate to the action
4. Press 'e' to edit the action
5. Update the Target field to the correct app name
6. Press 's' to save
7. Press 's' again to save binding
8. Press 's' in main view to apply to Karabiner

## Common App Names

Here are some commonly installed apps with their exact names:

### Browsers
- `Firefox`
- `Firefox Developer Edition` ← Your case!
- `Google Chrome`
- `Safari`
- `Arc`
- `Microsoft Edge`
- `Brave Browser`

### Development
- `Visual Studio Code`
- `Xcode`
- `IntelliJ IDEA`
- `PyCharm`
- `Sublime Text`

### Communication
- `Slack`
- `Discord`
- `Zoom.us` (note the .us!)
- `Microsoft Teams`

### Terminal
- `Terminal`
- `iTerm`
- `Warp`

## Pro Tip: Use Autocomplete

When adding an App action in the TUI:
1. Start typing the app name
2. Autocomplete will show suggestions
3. Use ↑↓ to navigate
4. Press Enter to select

The autocomplete includes 70+ common apps with correct names!

## Still Not Working?

Check these:
1. **Is the app actually installed?** Check /Applications/
2. **Is it in /Applications/?** Some apps install elsewhere
3. **Spelling exact?** Case matters! "firefox" ≠ "Firefox"
4. **Try the bundle ID instead?** Some apps need bundle ID (advanced)

## Advanced: Using Bundle IDs

If the app name doesn't work, you can use its bundle ID:

```bash
# Find bundle ID
osascript -e 'id of app "Firefox Developer Edition"'
# Returns: org.mozilla.firefoxdeveloperedition

# Use in shell command instead
open -b org.mozilla.firefoxdeveloperedition
```

For this, use a Shell action instead of App action with:
```
open -b org.mozilla.firefoxdeveloperedition
```

## Your Specific Case

**Problem:** rcmd+i doesn't open Firefox

**Cause:** Your binding uses "Firefox" but you have "Firefox Developer Edition" installed

**Solution:** Update your binding to use `Firefox Developer Edition`

Quick fix:
```bash
./run.sh
# Edit your rcmd+i binding
# Change "Firefox" to "Firefox Developer Edition"
# Save and apply
```
