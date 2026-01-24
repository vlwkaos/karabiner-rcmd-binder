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

/// Install helper scripts to the config directory
pub fn install_scripts() -> Result<PathBuf> {
    let scripts_dir = ensure_scripts_dir()?;

    // Install url-focus.sh
    let url_focus_path = scripts_dir.join("url-focus.sh");
    fs::write(&url_focus_path, URL_FOCUS_SCRIPT)
        .with_context(|| format!("Failed to write {:?}", url_focus_path))?;

    // Make executable
    let mut perms = fs::metadata(&url_focus_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&url_focus_path, perms)?;

    Ok(scripts_dir)
}
