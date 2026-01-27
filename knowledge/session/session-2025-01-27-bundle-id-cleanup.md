---
name: session-2025-01-27-bundle-id-cleanup
description: Implemented bundle ID support for reliable app launching, code cleanup, and performance optimization
keywords: bundle ID, validation, code cleanup, performance, app discovery, KakaoWork, script security
---

# Session: 2025-01-27 - Bundle ID Support & Code Cleanup

**Summary**: Implemented comprehensive bundle ID metadata support to fix apps like KakaoWork that can't launch by name. Cleaned up 117 lines of dead code, optimized performance, fixed security issues in scripts, and released v0.2.1.

## What Worked

- **Bundle ID Discovery**: osascript + plutil approach works perfectly
- **Auto-validation**: Resolving bundle IDs on save provides seamless backward compatibility
- **Deduplication**: Using bundle ID (not name) prevents duplicates
- **Autocomplete Disambiguation**: Shows "App (parent.component)" for name conflicts
- **Early Returns**: Dramatically improved code readability
- **Script Auto-detection**: Removed hardcoded credentials, now auto-detects from Keychain

## What Failed

- Initial attempt to block multi-char keys broke special keys like `left_arrow`
- Solution: Validate at save time instead of during typing

## Design Decisions

| Decision | Options Considered | Chosen | Rationale |
|----------|-------------------|--------|-----------|
| Bundle ID storage | Always required, Optional (backward compatible), Convert to Shell | Optional | Supports old configs, auto-migrates on save |
| Validation timing | During typing, At save time, Never | At save time | Allows typing for autocomplete search, validates before commit |
| Disambiguation format | `App (bundleid)`, `App [bundleid]`, `App (parent.component)` | `App (parent.component)` | Shows meaningful company name instead of full ID |
| Discovery limit | Unlimited, 200 apps, 150 apps | 150 apps | Balance between completeness and performance |
| System apps | Include, Skip | Skip | Rarely needed, significant performance cost |

## Open Questions

None - implementation complete and released.

## Context to Continue

v0.2.1 released with bundle ID support, code cleanup, and optimizations. All compiler warnings eliminated. Future work could include:
- Fuzzy search for app autocomplete
- URL autocomplete from browser history
- Import existing Karabiner rules

## Detailed Plan

**All items completed:**

### Phase 1: Bundle ID Infrastructure ✅
- [x] Create `DiscoveredApp` struct with name, bundle_id, last_component
- [x] Update `discover_running_apps()` to fetch bundle IDs via osascript
- [x] Update `discover_installed_apps()` to read Info.plist via plutil
- [x] Deduplicate by bundle_id instead of name

### Phase 2: Data Model Updates ✅
- [x] Add `bundle_id: Option<String>` to `Action::App`
- [x] Update `ActionEditor` to store bundle_id
- [x] Create `AutocompleteSuggestion` struct with display, value, bundle_id
- [x] Update all autocomplete usages

### Phase 3: Validation & Migration ✅
- [x] Create `validation.rs` module
- [x] Implement `validate_and_update_config()` to resolve missing bundle IDs
- [x] Integrate validation on save
- [x] Handle errors gracefully with warnings

### Phase 4: Generator & UI Updates ✅
- [x] Update `action_to_karabiner()` to use `open -b` when bundle_id present
- [x] Update autocomplete display to show disambiguated names
- [x] Add checkmark indicator for actions with bundle IDs

### Phase 5: Code Cleanup ✅
- [x] Remove 7 unused functions (ActionType::all, is_input_mode, mode_label, etc.)
- [x] Simplify validation logic with early returns
- [x] Simplify app_discovery logic
- [x] Optimize performance (150 app limit, skip /System/Applications)
- [x] Eliminate all compiler warnings

### Phase 6: Script Security ✅
- [x] Remove hardcoded email/cert ID from sign.sh
- [x] Auto-detect Apple Developer identity from Keychain
- [x] Add GPG check to package.sh
- [x] Add dependency validation to release.sh

### Phase 7: Release ✅
- [x] Build and sign binary
- [x] Create distribution package
- [x] Create git commits (2 organized commits)
- [x] Tag and push v0.2.1
- [x] Create GitHub release with artifacts
