# Plan & Progress

## Version History

### v0.2.1 - Bundle ID Support & Code Quality (Current)
- [x] Replace static app list with dynamic discovery
- [x] Scan running apps via osascript  
- [x] Scan installed apps from /Applications, ~/Applications (optimized)
- [x] Background thread with loading indicator
- [x] Bundle ID metadata for reliable app launching
- [x] Auto-validation and resolution of bundle IDs on save
- [x] Key validation (prevent invalid multi-char keys)
- [x] Performance optimization (150 app limit, 2-3x faster)
- [x] Code cleanup (removed 117 lines, zero warnings)
- [x] Script security audit (removed hardcoded credentials)

### v0.2.0 - Mode Indicators
- [x] Visual distinction between INPUT and NAV modes
- [x] Title prefix: `[INPUT]` / `[NAV]`
- [x] Border color: Green (INPUT) / Cyan (NAV)
- [x] Status bar mode hints

### v0.1.4 - Input Architecture Fix
- [x] Guard all command keys in text fields
- [x] Field-based key routing

### v0.1.x - Core Features
- [x] TUI with Ratatui
- [x] Binding CRUD
- [x] Action types: App, URL, Shell
- [x] Action cycling (set_variable)
- [x] Per-action browser override
- [x] Key autocomplete
- [x] URL match types: exact, domain, path, glob
- [x] Karabiner JSON generation
- [x] Backup rotation (3 files)
- [x] Helper scripts embedded

## Potential Future Features

### High Priority
- [ ] Fuzzy search for app autocomplete
- [ ] URL autocomplete from browser history
- [ ] Import existing Karabiner rules

### Medium Priority
- [ ] Custom app list additions
- [ ] Binding groups/folders
- [ ] Export/import config
- [ ] Undo/redo

### Low Priority
- [ ] Multiple modifier support (not just rcmd)
- [ ] Conditional bindings (app-specific)
- [ ] Remote config sync
