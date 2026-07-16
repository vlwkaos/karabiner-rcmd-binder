use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::config::persistence::ensure_scripts_dir;

/// Embedded url-focus.sh script
const URL_FOCUS_SCRIPT: &str = r#"#!/usr/bin/env bash
# url-focus.sh - Focus or open URL in browser with matching logic
# Usage: url-focus.sh <url> <match_type> <browser>
# match_type: exact | domain | path | glob

URL="$1"
MATCH_TYPE="$2"
BROWSER="$3"

# Extract domain from URL
get_domain() {
    echo "$1" | sed -E 's|^https?://||' | sed -E 's|/.*||' | sed -E 's|:.*||'
}

# Extract path from URL (domain + path, no query)
get_path() {
    echo "$1" | sed -E 's|^https?://||' | sed -E 's|\?.*||'
}

# Convert glob pattern to regex
glob_to_regex() {
    echo "$1" | sed -E 's/\./\\./g' | sed -E 's/\*/.*/g'
}

DOMAIN=$(get_domain "$URL")
PATH_PART=$(get_path "$URL")

case "$BROWSER" in
    chrome)
        # Chrome: Full AppleScript tab search
        osascript <<EOF
tell application "Google Chrome"
    set found to false
    repeat with w in windows
        set tabIndex to 0
        repeat with t in tabs of w
            set tabIndex to tabIndex + 1
            set tabUrl to URL of t
            set matched to false
            
            if "$MATCH_TYPE" is "exact" then
                if tabUrl is "$URL" then set matched to true
            else if "$MATCH_TYPE" is "domain" then
                if tabUrl contains "$DOMAIN" then set matched to true
            else if "$MATCH_TYPE" is "path" then
                if tabUrl contains "$PATH_PART" then set matched to true
            else if "$MATCH_TYPE" is "glob" then
                -- Simple glob: check if domain matches
                if tabUrl contains "$DOMAIN" then set matched to true
            end if
            
            if matched then
                set active tab index of w to tabIndex
                set index of w to 1
                activate
                set found to true
                exit repeat
            end if
        end repeat
        if found then exit repeat
    end repeat
    
    if not found then
        open location "$URL"
        activate
    end if
end tell
EOF
        ;;
    
    firefox)
        # Firefox: Limited AppleScript support, use open command
        # Firefox handles duplicate detection for exact URLs
        if pgrep -x "firefox" > /dev/null; then
            # Try to activate Firefox first
            osascript -e 'tell application "Firefox" to activate'
            # Open URL (Firefox may focus existing tab for exact match)
            open -a Firefox "$URL"
        else
            open -a Firefox "$URL"
        fi
        ;;
    
    safari)
        # Safari: Full AppleScript support
        osascript <<EOF
tell application "Safari"
    set found to false
    repeat with w in windows
        set tabIndex to 0
        repeat with t in tabs of w
            set tabIndex to tabIndex + 1
            set tabUrl to URL of t
            set matched to false
            
            if "$MATCH_TYPE" is "exact" then
                if tabUrl is "$URL" then set matched to true
            else if "$MATCH_TYPE" is "domain" then
                if tabUrl contains "$DOMAIN" then set matched to true
            else if "$MATCH_TYPE" is "path" then
                if tabUrl contains "$PATH_PART" then set matched to true
            else if "$MATCH_TYPE" is "glob" then
                if tabUrl contains "$DOMAIN" then set matched to true
            end if
            
            if matched then
                set current tab of w to t
                set index of w to 1
                activate
                set found to true
                exit repeat
            end if
        end repeat
        if found then exit repeat
    end repeat
    
    if not found then
        open location "$URL"
        activate
    end if
end tell
EOF
        ;;
    
    arc)
        # Arc: Similar to Chrome (Chromium-based)
        osascript <<EOF
tell application "Arc"
    set found to false
    repeat with w in windows
        set tabIndex to 0
        repeat with t in tabs of w
            set tabIndex to tabIndex + 1
            set tabUrl to URL of t
            set matched to false
            
            if "$MATCH_TYPE" is "exact" then
                if tabUrl is "$URL" then set matched to true
            else if "$MATCH_TYPE" is "domain" then
                if tabUrl contains "$DOMAIN" then set matched to true
            else if "$MATCH_TYPE" is "path" then
                if tabUrl contains "$PATH_PART" then set matched to true
            else if "$MATCH_TYPE" is "glob" then
                if tabUrl contains "$DOMAIN" then set matched to true
            end if
            
            if matched then
                set active tab index of w to tabIndex
                set index of w to 1
                activate
                set found to true
                exit repeat
            end if
        end repeat
        if found then exit repeat
    end repeat
    
    if not found then
        open location "$URL"
        activate
    end if
end tell
EOF
        ;;
    
    edge)
        # Edge: Similar to Chrome (Chromium-based)
        osascript <<EOF
tell application "Microsoft Edge"
    set found to false
    repeat with w in windows
        set tabIndex to 0
        repeat with t in tabs of w
            set tabIndex to tabIndex + 1
            set tabUrl to URL of t
            set matched to false
            
            if "$MATCH_TYPE" is "exact" then
                if tabUrl is "$URL" then set matched to true
            else if "$MATCH_TYPE" is "domain" then
                if tabUrl contains "$DOMAIN" then set matched to true
            else if "$MATCH_TYPE" is "path" then
                if tabUrl contains "$PATH_PART" then set matched to true
            else if "$MATCH_TYPE" is "glob" then
                if tabUrl contains "$DOMAIN" then set matched to true
            end if
            
            if matched then
                set active tab index of w to tabIndex
                set index of w to 1
                activate
                set found to true
                exit repeat
            end if
        end repeat
        if found then exit repeat
    end repeat
    
    if not found then
        open location "$URL"
        activate
    end if
end tell
EOF
        ;;
    
    *)
        # Fallback: just open the URL
        open "$URL"
        ;;
esac
"#;

/// Embedded center-mouse.sh script
const CENTER_MOUSE_SCRIPT: &str = r#"#!/usr/bin/env bash
# center-mouse.sh <bundle_id> [mode]
# mode: always (default) | multi_monitor_only
# Polls until target app is frontmost (up to 0.5s), then centers mouse on its window.
# Kills any prior instance for the same bundle_id (cancel + restart semantics).

BUNDLE_ID="$1"
MODE="${2:-always}"

if [ -z "$BUNDLE_ID" ]; then
    exit 1
fi

SAFE_ID=$(printf '%s' "$BUNDLE_ID" | tr '.' '_' | tr '/' '_')
LOCK_DIR="${TMPDIR%/}/rcmdb-center"
LOCK_FILE="${LOCK_DIR}/${SAFE_ID}.pid"

mkdir -p "$LOCK_DIR"

# Kill previous instance for the same app
if [ -f "$LOCK_FILE" ]; then
    OLD_PID=$(cat "$LOCK_FILE" 2>/dev/null)
    if [ -n "$OLD_PID" ] && kill -0 "$OLD_PID" 2>/dev/null; then
        # Kill children (e.g. osascript) then the shell itself
        for child in $(pgrep -P "$OLD_PID" 2>/dev/null); do
            kill "$child" 2>/dev/null
        done
        kill "$OLD_PID" 2>/dev/null
    fi
fi

printf '%d' $$ > "$LOCK_FILE"
trap 'rm -f "$LOCK_FILE"' EXIT

osascript -l JavaScript - "$BUNDLE_ID" "$MODE" << 'JSEOF'
ObjC.import('CoreGraphics');
ObjC.import('AppKit');

function run(argv) {
    var targetBundle = argv[0];
    var mode = argv[1] || 'always';

    if (mode === 'multi_monitor_only' && $.NSScreen.screens.count <= 1) {
        return;
    }

    var ws = $.NSWorkspace.sharedWorkspace;

    // Wait for the target app to become frontmost. Poll NSWorkspace in-process
    // (~microseconds/iteration) instead of System Events over Apple Events (~80ms),
    // so this resolves the instant the app activates rather than after poll rounds.
    var timeout = 0.5;
    var interval = 0.01;
    var elapsed = 0;
    var isFront = false;
    while (elapsed < timeout) {
        var frontBundle = null;
        try { frontBundle = ws.frontmostApplication.bundleIdentifier.js; } catch (e) {}
        if (frontBundle === targetBundle) { isFront = true; break; }
        delay(interval);
        elapsed += interval;
    }
    if (!isFront) return;

    // Frontmost: read the front window geometry once (AX) and center the cursor on it.
    try {
        var sysEvt = Application('System Events');
        var proc = sysEvt.processes.whose({ bundleIdentifier: targetBundle })()[0];
        if (!proc) return;
        var win = proc.windows[0];
        var pos = win.position();
        var sz = win.size();
        $.CGWarpMouseCursorPosition({
            x: pos[0] + sz[0] / 2,
            y: pos[1] + sz[1] / 2
        });
    } catch(e) {}
}
JSEOF
"#;

/// Embedded cycle-window.sh script
const CYCLE_WINDOW_SCRIPT: &str = r#"#!/usr/bin/env bash
# cycle-window.sh <bundle_id> [center_mode]
# First press (app not frontmost): launch/focus the app.
# Repeat press (app already frontmost): raise its next window, wrapping around.
# State is implicit in live window focus — no Karabiner variable needed.
# center_mode (always|multi_monitor_only), when set, centers the cursor on the
# resulting front window IN THE SAME osascript, so the cycling+center path pays a
# single JXA interpreter start instead of chaining a second center-mouse.sh process.

BUNDLE_ID="$1"
MODE="$2"

if [ -z "$BUNDLE_ID" ]; then
    exit 1
fi

osascript -l JavaScript - "$BUNDLE_ID" "$MODE" << 'JSEOF'
ObjC.import('CoreGraphics');
ObjC.import('AppKit');

function centerWanted(mode) {
    if (!mode) return false;
    if (mode === 'multi_monitor_only' && $.NSScreen.screens.count <= 1) return false;
    return true;
}

function warpToWindow(win) {
    try {
        var pos = win.position();
        var sz = win.size();
        $.CGWarpMouseCursorPosition({ x: pos[0] + sz[0] / 2, y: pos[1] + sz[1] / 2 });
    } catch (e) {}
}

function run(argv) {
    var targetBundle = argv[0];
    var mode = argv[1] || '';
    var wantCenter = centerWanted(mode);
    var ws = $.NSWorkspace.sharedWorkspace;

    // Is the target app already frontmost? Read it in-process via NSWorkspace
    // (~microseconds) instead of a System Events Apple-Events query (~80ms).
    var frontBundle = null;
    try { frontBundle = ws.frontmostApplication.bundleIdentifier.js; } catch (e) {}

    if (frontBundle !== targetBundle) {
        // First press: launch or focus the app.
        ws.launchAppWithBundleIdentifierOptionsAdditionalEventParamDescriptorLaunchIdentifier(
            targetBundle, $.NSWorkspaceLaunchDefault, $.NSAppleEventDescriptor.nullDescriptor, null
        );
        if (!wantCenter) return;
        // Wait for the app to become frontmost (polling NSWorkspace in-process, near-free),
        // then center on its front window. AX is only touched once, after it fronts.
        var timeout = 0.5, interval = 0.01, elapsed = 0, isFront = false;
        while (elapsed < timeout) {
            var fb = null;
            try { fb = ws.frontmostApplication.bundleIdentifier.js; } catch (e) {}
            if (fb === targetBundle) { isFront = true; break; }
            delay(interval);
            elapsed += interval;
        }
        if (!isFront) return;
        try {
            var sysEvt = Application('System Events');
            var proc = sysEvt.processes.whose({ bundleIdentifier: targetBundle })()[0];
            if (proc) warpToWindow(proc.windows[0]);
        } catch (e) {}
        return;
    }

    // App is frontmost: cycle to its next window (window geometry/raise needs AX).
    try {
        var sysEvt = Application('System Events');
        var proc = sysEvt.processes.whose({ bundleIdentifier: targetBundle })()[0];
        if (!proc) return;
        var wins = proc.windows();
        if (!wins || wins.length === 0) return;
        if (wins.length === 1) {
            // Nothing to cycle; still center on the sole window if asked.
            if (wantCenter) warpToWindow(wins[0]);
            return;
        }

        // The frontmost window is index 0; raise the next one.
        var next = wins[1];
        proc.frontmost = true;
        next.actions['AXRaise'].perform();
        // Center on the window we just raised (we hold its reference — no re-query race).
        if (wantCenter) warpToWindow(next);
    } catch (e) {}
}
JSEOF
"#;

fn write_executable(path: &std::path::Path, content: &str) -> Result<()> {
    fs::write(path, content).with_context(|| format!("Failed to write {:?}", path))?;
    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms)?;
    Ok(())
}

/// Install helper scripts to the config directory
pub fn install_scripts() -> Result<PathBuf> {
    let scripts_dir = ensure_scripts_dir()?;

    write_executable(&scripts_dir.join("url-focus.sh"), URL_FOCUS_SCRIPT)?;
    write_executable(&scripts_dir.join("center-mouse.sh"), CENTER_MOUSE_SCRIPT)?;
    write_executable(&scripts_dir.join("cycle-window.sh"), CYCLE_WINDOW_SCRIPT)?;

    Ok(scripts_dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scripts_use_env_bash_shebang() {
        // Must use #!/usr/bin/env bash for portability — #!/bin/bash breaks on NixOS, some Linux distros
        assert!(
            URL_FOCUS_SCRIPT.starts_with("#!/usr/bin/env bash"),
            "url-focus.sh must start with #!/usr/bin/env bash"
        );
        assert!(
            CENTER_MOUSE_SCRIPT.starts_with("#!/usr/bin/env bash"),
            "center-mouse.sh must start with #!/usr/bin/env bash"
        );
        assert!(
            CYCLE_WINDOW_SCRIPT.starts_with("#!/usr/bin/env bash"),
            "cycle-window.sh must start with #!/usr/bin/env bash"
        );
        assert!(
            !URL_FOCUS_SCRIPT.contains("#!/bin/bash"),
            "url-focus.sh must not contain #!/bin/bash"
        );
        assert!(
            !CENTER_MOUSE_SCRIPT.contains("#!/bin/bash"),
            "center-mouse.sh must not contain #!/bin/bash"
        );
        assert!(
            !CYCLE_WINDOW_SCRIPT.contains("#!/bin/bash"),
            "cycle-window.sh must not contain #!/bin/bash"
        );
    }

    #[test]
    fn test_scripts_no_baked_absolute_paths() {
        // Scripts must not embed save-time user paths — use $HOME or relative refs only
        assert!(
            !URL_FOCUS_SCRIPT.contains("/Users/"),
            "url-focus.sh must not embed absolute user path"
        );
        assert!(
            !CENTER_MOUSE_SCRIPT.contains("/Users/"),
            "center-mouse.sh must not embed absolute user path"
        );
        assert!(
            !CYCLE_WINDOW_SCRIPT.contains("/Users/"),
            "cycle-window.sh must not embed absolute user path"
        );
    }
}
