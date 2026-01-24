# Keybindings Cheat Sheet

## Quick Reference

```
┌─────────────────────────────────────────────────────────┐
│                    NORMAL MODE                          │
├─────────────────────────────────────────────────────────┤
│  q              Quit application                        │
│  Tab            Switch between Bindings/Settings tabs   │
│  s              Save to karabiner.json                  │
│                                                         │
│  BINDINGS TAB:                                          │
│  j/k or ↑/↓     Navigate binding list                   │
│  a              Add new binding                         │
│  e or Enter     Edit selected binding                   │
│  d              Delete selected binding                 │
│                                                         │
│  SETTINGS TAB:                                          │
│  </> or ,/.     Change browser selection                │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│                   EDITING MODE                          │
├─────────────────────────────────────────────────────────┤
│  Tab            Next field                              │
│  Shift+Tab      Previous field                          │
│  Enter          Save/confirm                            │
│  Esc            Cancel                                  │
│                                                         │
│  KEY FIELD (with autocomplete):                         │
│  Type           Enter key name                          │
│  Backspace      Delete character                        │
│  ↑/↓            Navigate autocomplete                   │
│  →              Accept autocomplete suggestion          │
│                                                         │
│  ACTIONS FIELD:                                         │
│  a              Add new action                          │
│  e              Edit selected action                    │
│  d              Delete selected action                  │
│  j/k or ↑/↓     Navigate action list                    │
│  Shift+J        Move action up                          │
│  Shift+K        Move action down                        │
└─────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────┐
│                 ACTION EDITOR                           │
├─────────────────────────────────────────────────────────┤
│  Tab            Next field                              │
│  Shift+Tab      Previous field                          │
│  Enter          Save action                             │
│  Esc            Cancel                                  │
│                                                         │
│  TYPE FIELD:                                            │
│  </> or ,/.     Cycle: App → URL → Shell               │
│                                                         │
│  TARGET FIELD:                                          │
│  Type           Enter app name/URL/command              │
│  Backspace      Delete character                        │
│                                                         │
│  MATCH TYPE (URL only):                                 │
│  </> or ,/.     Cycle: exact → domain → path → glob    │
│                                                         │
│  BROWSER (URL only):                                    │
│  </> or ,/.     Cycle browsers or (use default)        │
└─────────────────────────────────────────────────────────┘

## Common Workflows

### Add Simple App Binding
1. Press `a` (add binding)
2. Type key (e.g., `s`)
3. Tab, type description (e.g., "Slack")
4. Tab, press `a` (add action)
5. Type app name (e.g., "Slack")
6. Press Enter twice
7. Press `s` to save to Karabiner

### Add Cycling Binding (3 Apps)
1. Press `a`
2. Type key (e.g., `t`)
3. Tab, type description (e.g., "Terminals")
4. Tab, press `a`, type "Terminal", Enter
5. Press `a`, type "iTerm", Enter
6. Press `a`, type "Warp", Enter
7. Press Enter
8. Press `s` to save

### Add URL with Tab Focusing
1. Press `a`
2. Type key (e.g., `g`)
3. Tab, type description (e.g., "GitHub")
4. Tab, press `a`
5. Press `>` to change type to URL
6. Tab, type URL (e.g., "https://github.com")
7. Tab, press `>` until "domain"
8. Tab, select browser or leave default
9. Press Enter twice
10. Press `s` to save

### Edit Existing Binding
1. Navigate to binding with `j`/`k`
2. Press `e` or Enter
3. Make changes
4. Press Enter to save
5. Press `s` to apply to Karabiner

### Delete Binding
1. Navigate to binding with `j`/`k`
2. Press `d`
3. Press `s` to apply to Karabiner

## Tips

- **Autocomplete**: Start typing a key name and suggestions appear
- **Cycling Order**: Actions execute in the order shown (top to bottom)
- **Move Actions**: Use Shift+J/K to reorder in the list
- **Browser Override**: Leave browser as "(use default)" to use Settings tab browser
- **Shell Commands**: Use full paths or commands in $PATH
- **Status Messages**: Watch the bottom bar for feedback

## Keyboard Layouts

### US QWERTY Valid Keys
Letters: `a-z`
Numbers: `0-9`
Function: `f1-f20`
Special: `return_or_enter`, `escape`, `tab`, `spacebar`, etc.
Arrows: `up_arrow`, `down_arrow`, `left_arrow`, `right_arrow`

See autocomplete for full list!

## Generated Keybindings in macOS

After saving with `s`, your bindings are active as:
- `rcmd+<key>` for each binding
- Example: `rcmd+t` for key "t"
- Right Command (⌘) only (not left command)
