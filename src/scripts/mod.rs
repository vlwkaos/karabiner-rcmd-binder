use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

use crate::config::persistence::ensure_scripts_dir;

/// Embedded url-focus.sh script
const URL_FOCUS_SCRIPT: &str = r#"#!/bin/bash
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
const CENTER_MOUSE_SCRIPT: &str = r#"#!/bin/bash
# center-mouse.sh <bundle_id>
# Polls until target app is frontmost (up to 0.5s), then centers mouse on its window.
# Kills any prior instance for the same bundle_id (cancel + restart semantics).

BUNDLE_ID="$1"

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

osascript -l JavaScript - "$BUNDLE_ID" << 'JSEOF'
ObjC.import('CoreGraphics');

function run(argv) {
    var targetBundle = argv[0];
    var timeout = 0.5;
    var interval = 0.05;
    var elapsed = 0;
    var sysEvt = Application('System Events');

    while (elapsed < timeout) {
        try {
            var procs = sysEvt.processes.whose({ frontmost: true })();
            if (procs.length > 0) {
                var front = procs[0];
                if (front.bundleIdentifier() === targetBundle) {
                    var win = front.windows[0];
                    var pos = win.position();
                    var sz = win.size();
                    $.CGWarpMouseCursorPosition({
                        x: pos[0] + sz[0] / 2,
                        y: pos[1] + sz[1] / 2
                    });
                    return;
                }
            }
        } catch(e) {}
        delay(interval);
        elapsed += interval;
    }
}
JSEOF
"#;

/// Install helper scripts to the config directory
pub fn install_scripts() -> Result<PathBuf> {
    let scripts_dir = ensure_scripts_dir()?;

    // Install url-focus.sh
    let url_focus_path = scripts_dir.join("url-focus.sh");
    fs::write(&url_focus_path, URL_FOCUS_SCRIPT)
        .with_context(|| format!("Failed to write {:?}", url_focus_path))?;
    let mut perms = fs::metadata(&url_focus_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&url_focus_path, perms)?;

    // Install center-mouse.sh
    let center_mouse_path = scripts_dir.join("center-mouse.sh");
    fs::write(&center_mouse_path, CENTER_MOUSE_SCRIPT)
        .with_context(|| format!("Failed to write {:?}", center_mouse_path))?;
    let mut perms = fs::metadata(&center_mouse_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&center_mouse_path, perms)?;

    Ok(scripts_dir)
}
