//! Native macOS window cycling, invoked per keypress as
//! `rcmdb cycle-window <bundle_id> [center_mode]`.
//!
//! This replaces the former osascript + System Events helper. Two things made that
//! helper slow and flaky under load: every press paid a JXA interpreter cold start
//! (~100ms+ when the machine is busy), and every window read/raise was an Apple Event
//! routed through the shared System Events process, whose queue backs up under
//! contention. Here the logic is compiled (starts in a few ms) and drives the target
//! app's Accessibility server directly via the AX C API, with no intermediary process.
//!
//! It stays strictly per-press: the process starts, does one thing, exits. There is no
//! resident daemon, so nothing is "always hogging" the machine.
//!
//! The window-cycling behaviour matches the old script: first press (app not frontmost)
//! launches/focuses the app; a repeat press raises the backmost non-minimized window,
//! which round-robins through every window on successive presses. AXWindows is
//! front-to-back z-order (index 0 = frontmost), so raising the backmost reorders to
//! `[A,B,C,D] -> [D,A,B,C] -> [C,D,A,B] -> ...`; raising index 1 would only ever toggle
//! the top two.

use objc2::MainThreadMarker;
use objc2_app_kit::{NSScreen, NSWorkspace};
use objc2_foundation::NSString;
use std::ffi::c_void;
use std::process::Command;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;

// ---- Accessibility C API ----------------------------------------------------
// No cached objc2 crate covers Accessibility, so these are hand-rolled. All AX
// `Copy*` calls return +1-retained CoreFoundation objects; this process exits right
// after one operation, so we deliberately leak them rather than carry CFRelease
// bookkeeping the OS will do for us on exit.

type AXUIElementRef = *const c_void;
type AXValueRef = *const c_void;
type CFTypeRef = *const c_void;
type CFStringRef = *const c_void;

const KAX_ERROR_SUCCESS: i32 = 0;
const KAX_VALUE_TYPE_CGPOINT: u32 = 1;
const KAX_VALUE_TYPE_CGSIZE: u32 = 2;
const QOS_CLASS_USER_INTERACTIVE: u32 = 0x21;

#[repr(C)]
#[derive(Clone, Copy)]
struct CGPoint {
    x: f64,
    y: f64,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct CGSize {
    width: f64,
    height: f64,
}

/// Opaque CoreFoundation `CFDictionaryCallBacks` structs; we only ever take their
/// address to hand the standard CF-type callbacks to `CFDictionaryCreate`.
#[repr(C)]
struct CFDictionaryCallBacks {
    _private: [u8; 0],
}

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXUIElementCreateApplication(pid: i32) -> AXUIElementRef;
    fn AXUIElementCopyAttributeValue(
        element: AXUIElementRef,
        attribute: CFStringRef,
        value: *mut CFTypeRef,
    ) -> i32;
    fn AXUIElementPerformAction(element: AXUIElementRef, action: CFStringRef) -> i32;
    // These return the CoreFoundation `Boolean` (an `unsigned char`), not a C99 `_Bool`.
    // Declaring them as Rust `bool` would be UB if a byte other than 0/1 came back, so we
    // take them as `u8` and compare `!= 0` at the call sites.
    fn AXValueGetValue(value: AXValueRef, the_type: u32, value_ptr: *mut c_void) -> u8;
    fn AXIsProcessTrustedWithOptions(options: CFTypeRef) -> u8;
    static kAXTrustedCheckOptionPrompt: CFStringRef;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFArrayGetCount(array: CFTypeRef) -> isize;
    fn CFArrayGetValueAtIndex(array: CFTypeRef, idx: isize) -> *const c_void;
    // CoreFoundation `Boolean` (`unsigned char`); taken as `u8`, compared `!= 0`.
    fn CFBooleanGetValue(boolean: CFTypeRef) -> u8;
    fn CFDictionaryCreate(
        allocator: *const c_void,
        keys: *const *const c_void,
        values: *const *const c_void,
        num_values: isize,
        key_callbacks: *const CFDictionaryCallBacks,
        value_callbacks: *const CFDictionaryCallBacks,
    ) -> CFTypeRef;
    static kCFBooleanTrue: CFTypeRef;
    static kCFTypeDictionaryKeyCallBacks: CFDictionaryCallBacks;
    static kCFTypeDictionaryValueCallBacks: CFDictionaryCallBacks;
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGWarpMouseCursorPosition(new_cursor_position: CGPoint) -> i32;
}

extern "C" {
    fn pthread_set_qos_class_self_np(qos_class: u32, relative_priority: i32) -> i32;
}

/// Entry point for the `cycle-window` subcommand.
///
/// `center_mode` is the same enum string the generator passes (`always` |
/// `multi_monitor_only`); `None` means never center the cursor.
pub fn cycle_window(bundle_id: &str, center_mode: Option<&str>) {
    // Ask the scheduler to treat this short-lived process as user-interactive so a
    // press stays snappy even when the machine is under load. Best-effort; ignore the
    // result (it only fails on an invalid class, which this constant is not).
    unsafe { pthread_set_qos_class_self_np(QOS_CLASS_USER_INTERACTIVE, 0) };

    // Surface the Accessibility prompt on first use. A CLI binary invoked from a
    // script gets NO automatic prompt, and untrusted AX calls fail silently — only
    // this explicit prompting check makes macOS show the System Settings dialog.
    // When already trusted it returns true and shows nothing, so it is safe per-press.
    // (App focus via `open -b` still works without the grant; only cycling needs it.)
    prompt_accessibility_if_needed();

    let mtm = MainThreadMarker::new();
    let want_center = center_wanted(center_mode, mtm);

    let workspace = NSWorkspace::sharedWorkspace();
    let front = workspace.frontmostApplication();
    let front_bundle = front
        .as_ref()
        .and_then(|app| app.bundleIdentifier())
        .map(|s| s.to_string());

    if front_bundle.as_deref() != Some(bundle_id) {
        // First press: launch or focus. `open -b` is a native launch path (no AX).
        let _ = Command::new("open").arg("-b").arg(bundle_id).status();
        if !want_center {
            return;
        }
        // Poll NSWorkspace (in-process, cheap) until the app fronts, then center on its
        // front window. AX is touched only once the app is actually up.
        if let Some(pid) = poll_until_frontmost(&workspace, bundle_id) {
            let app = unsafe { AXUIElementCreateApplication(pid) };
            if !app.is_null() {
                let wins = ax_windows(app);
                if let Some(&win) = wins.first() {
                    center_on(win);
                }
            }
        }
        return;
    }

    // Repeat press: the target is frontmost, so cycle to its next window.
    let pid = front
        .expect("front is Some when front_bundle matched")
        .processIdentifier();
    let app = unsafe { AXUIElementCreateApplication(pid) };
    if app.is_null() {
        return;
    }
    let wins = ax_windows(app);
    match wins.len() {
        0 => {}
        1 => {
            if want_center {
                center_on(wins[0]);
            }
        }
        _ => {
            // Walk from the back, skipping minimized windows so a press never lands on a
            // Dock-minimized window (AXRaise won't restore it) and appears stuck.
            let target = (1..wins.len())
                .rev()
                .map(|i| wins[i])
                .find(|&win| !ax_is_minimized(win));
            match target {
                None => {
                    if want_center {
                        center_on(wins[0]);
                    }
                }
                Some(win) => {
                    let raise = NSString::from_str("AXRaise");
                    unsafe { AXUIElementPerformAction(win, cfstring_ref(&raise)) };
                    if want_center {
                        // We hold the raised window's reference, so no re-query race.
                        center_on(win);
                    }
                }
            }
        }
    }
}

/// Should the cursor be centered, given the mode and how many displays are attached?
fn center_wanted(mode: Option<&str>, mtm: Option<MainThreadMarker>) -> bool {
    match mode {
        None | Some("off") => false,
        Some("multi_monitor_only") => match mtm {
            // Only center when more than one display is connected.
            Some(mtm) => NSScreen::screens(mtm).count() > 1,
            // Can't read screens off the main thread; be conservative and skip.
            None => false,
        },
        Some(_) => true, // "always" (and any future non-off mode)
    }
}

/// Poll NSWorkspace up to 0.5s for `bundle_id` to become frontmost; return its pid.
fn poll_until_frontmost(workspace: &NSWorkspace, bundle_id: &str) -> Option<i32> {
    for _ in 0..50 {
        if let Some(app) = workspace.frontmostApplication() {
            if app.bundleIdentifier().map(|s| s.to_string()).as_deref() == Some(bundle_id) {
                return Some(app.processIdentifier());
            }
        }
        sleep(Duration::from_millis(10));
    }
    None
}

/// The app's windows in front-to-back z-order (index 0 = frontmost). Empty on failure.
fn ax_windows(app: AXUIElementRef) -> Vec<AXUIElementRef> {
    let attr = NSString::from_str("AXWindows");
    let mut value: CFTypeRef = ptr::null();
    let err = unsafe { AXUIElementCopyAttributeValue(app, cfstring_ref(&attr), &mut value) };
    if err != KAX_ERROR_SUCCESS || value.is_null() {
        return Vec::new();
    }
    let count = unsafe { CFArrayGetCount(value) };
    (0..count)
        .map(|i| unsafe { CFArrayGetValueAtIndex(value, i) })
        .filter(|p| !p.is_null())
        .collect()
}

/// Is this window minimized to the Dock? Treats any read failure as "not minimized".
fn ax_is_minimized(win: AXUIElementRef) -> bool {
    let attr = NSString::from_str("AXMinimized");
    let mut value: CFTypeRef = ptr::null();
    let err = unsafe { AXUIElementCopyAttributeValue(win, cfstring_ref(&attr), &mut value) };
    if err != KAX_ERROR_SUCCESS || value.is_null() {
        return false;
    }
    unsafe { CFBooleanGetValue(value) != 0 }
}

/// Warp the cursor to the center of `win`, if its geometry can be read.
fn center_on(win: AXUIElementRef) {
    let (Some(pos), Some(size)) = (
        ax_point(win, "AXPosition", KAX_VALUE_TYPE_CGPOINT),
        ax_point(win, "AXSize", KAX_VALUE_TYPE_CGSIZE),
    ) else {
        return;
    };
    // AXPosition is a CGPoint and AXSize a CGSize, but both marshal through AXValue as a
    // pair of f64s, so we read both into CGPoint and treat size's fields as width/height.
    unsafe {
        CGWarpMouseCursorPosition(CGPoint {
            x: pos.x + size.x / 2.0,
            y: pos.y + size.y / 2.0,
        })
    };
}

/// Read a two-f64 AXValue attribute (AXPosition/AXSize) as a CGPoint-shaped pair.
fn ax_point(win: AXUIElementRef, attr: &str, ax_type: u32) -> Option<CGPoint> {
    let name = NSString::from_str(attr);
    let mut value: CFTypeRef = ptr::null();
    let err = unsafe { AXUIElementCopyAttributeValue(win, cfstring_ref(&name), &mut value) };
    if err != KAX_ERROR_SUCCESS || value.is_null() {
        return None;
    }
    if ax_type == KAX_VALUE_TYPE_CGSIZE {
        let mut out = CGSize {
            width: 0.0,
            height: 0.0,
        };
        let ok = unsafe {
            AXValueGetValue(value, ax_type, &mut out as *mut CGSize as *mut c_void) != 0
        };
        return ok.then_some(CGPoint {
            x: out.width,
            y: out.height,
        });
    }
    let mut out = CGPoint { x: 0.0, y: 0.0 };
    let ok =
        unsafe { AXValueGetValue(value, ax_type, &mut out as *mut CGPoint as *mut c_void) != 0 };
    ok.then_some(out)
}

/// Prompt for Accessibility if this process isn't trusted yet, returning the current
/// trust state. A CLI binary launched from a script gets no automatic prompt and
/// untrusted AX calls fail silently, so this explicit prompting check is the only way
/// to surface the System Settings dialog. Already-trusted processes get no dialog.
pub fn prompt_accessibility_if_needed() -> bool {
    unsafe {
        let keys = [kAXTrustedCheckOptionPrompt];
        let values = [kCFBooleanTrue];
        let options = CFDictionaryCreate(
            ptr::null(),
            keys.as_ptr(),
            values.as_ptr(),
            1,
            &kCFTypeDictionaryKeyCallBacks,
            &kCFTypeDictionaryValueCallBacks,
        );
        AXIsProcessTrustedWithOptions(options) != 0
    }
}

/// NSString is toll-free bridged to CFString, so its pointer is a valid CFStringRef.
fn cfstring_ref(s: &NSString) -> CFStringRef {
    (s as *const NSString) as CFStringRef
}

#[cfg(test)]
mod tests {
    use super::center_wanted;

    // center_wanted's mode logic is pure and exercised here with `mtm = None`, which
    // never touches NSScreen (that path needs a live main thread + display and is only
    // reachable at runtime). These cover the decision every generated mode string hits.

    #[test]
    fn off_and_absent_modes_do_not_center() {
        assert!(!center_wanted(None, None));
        assert!(!center_wanted(Some("off"), None));
    }

    #[test]
    fn always_and_unknown_modes_center() {
        assert!(center_wanted(Some("always"), None));
        // Any future non-off mode string falls through to centering rather than silently
        // doing nothing.
        assert!(center_wanted(Some("something_new"), None));
    }

    #[test]
    fn multi_monitor_only_without_main_thread_is_conservative() {
        // Screen count can't be read off the main thread; multi_monitor_only must skip
        // centering rather than guess that multiple displays are attached.
        assert!(!center_wanted(Some("multi_monitor_only"), None));
    }
}
