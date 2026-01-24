---
name: session-2025-01-24-dynamic-discovery
description: Implemented dynamic app discovery replacing static list with running/installed app scanning
keywords: app discovery, osascript, threading, mpsc, autocomplete, loading indicator
---

# Session: 2025-01-24 - Dynamic App Discovery

**Summary**: Replaced static app list (~70 hardcoded apps) with dynamic discovery that scans running and installed applications on the system.

## What Worked

- Background threading with mpsc channel for non-blocking discovery
- osascript for running apps query (System Events)
- read_dir for installed apps (/Applications, /System/Applications, ~/Applications)
- Case-insensitive deduplication (running apps first)
- Loading indicator in field title "(Loading...)"

## What Failed

- Initial approach tried to pass receiver to wrong function
- osascript takes ~450ms which would block UI if done synchronously

## Design Decisions

| Decision | Options Considered | Chosen | Rationale |
|----------|-------------------|--------|-----------|
| Discovery timing | On startup / On editor open / On-demand | On editor open | Fresh data when needed, no startup delay |
| Threading | Async runtime / std::thread | std::thread | Simple, no new dependencies |
| Communication | Shared state / mpsc channel | mpsc channel | Clean ownership, no locks |
| Static fallback | Keep static list as backup / Pure dynamic | Pure dynamic | User requested, cleaner code |

## Open Questions

None - implementation complete.

## Context to Continue

Dynamic app discovery is complete and working. The static `app_names.rs` file was deleted. Apps are discovered fresh each time the binding editor opens.

Key files:
- `src/app_discovery.rs` - Discovery functions
- `src/main.rs:504` - `spawn_app_discovery()` function
- `src/app.rs:413-435` - App state methods for discovery

## Detailed Plan

All items completed:
- [x] Create `src/app_discovery.rs` with `discover_running_apps()`, `discover_installed_apps()`, `discover_all_apps()`
- [x] Add `discovered_apps: Vec<String>` and `apps_loading: bool` to App struct
- [x] Create mpsc channel in main()
- [x] Spawn discovery thread when editor opens (`start_new_binding`, `start_edit_binding`)
- [x] Poll channel with `try_recv()` in event loop
- [x] Update `update_app_autocomplete()` to use `discovered_apps`
- [x] Show "(Loading...)" indicator in Target field title
- [x] Delete `src/app_names.rs`
- [x] All tests pass
