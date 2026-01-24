/// Valid Karabiner key codes for autocomplete
/// Reference: https://karabiner-elements.pqrs.org/docs/json/complex-modifications-manipulator-definition/from/

/// Letter keys (a-z)
pub const LETTER_KEYS: &[&str] = &[
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
    "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
];

/// Number keys (0-9)
pub const NUMBER_KEYS: &[&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
];

/// Function keys
pub const FUNCTION_KEYS: &[&str] = &[
    "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12",
    "f13", "f14", "f15", "f16", "f17", "f18", "f19", "f20",
];

/// Special keys
pub const SPECIAL_KEYS: &[&str] = &[
    "return_or_enter",
    "escape",
    "delete_or_backspace",
    "delete_forward",
    "tab",
    "spacebar",
    "hyphen",
    "equal_sign",
    "open_bracket",
    "close_bracket",
    "backslash",
    "non_us_pound",
    "semicolon",
    "quote",
    "grave_accent_and_tilde",
    "comma",
    "period",
    "slash",
    "caps_lock",
];

/// Arrow keys
pub const ARROW_KEYS: &[&str] = &[
    "up_arrow",
    "down_arrow",
    "left_arrow",
    "right_arrow",
];

/// Navigation keys
pub const NAVIGATION_KEYS: &[&str] = &[
    "page_up",
    "page_down",
    "home",
    "end",
    "insert",
];

/// Keypad keys
pub const KEYPAD_KEYS: &[&str] = &[
    "keypad_num_lock",
    "keypad_slash",
    "keypad_asterisk",
    "keypad_hyphen",
    "keypad_plus",
    "keypad_enter",
    "keypad_1",
    "keypad_2",
    "keypad_3",
    "keypad_4",
    "keypad_5",
    "keypad_6",
    "keypad_7",
    "keypad_8",
    "keypad_9",
    "keypad_0",
    "keypad_period",
    "keypad_equal_sign",
];

/// All valid key codes combined
pub fn all_key_codes() -> Vec<&'static str> {
    let mut codes = Vec::new();
    codes.extend_from_slice(LETTER_KEYS);
    codes.extend_from_slice(NUMBER_KEYS);
    codes.extend_from_slice(FUNCTION_KEYS);
    codes.extend_from_slice(SPECIAL_KEYS);
    codes.extend_from_slice(ARROW_KEYS);
    codes.extend_from_slice(NAVIGATION_KEYS);
    codes.extend_from_slice(KEYPAD_KEYS);
    codes
}

/// Check if a key code is valid
pub fn is_valid_key(key: &str) -> bool {
    all_key_codes().contains(&key)
}

/// Get autocomplete suggestions for a partial key input
pub fn autocomplete(partial: &str) -> Vec<&'static str> {
    if partial.is_empty() {
        return all_key_codes();
    }
    let lower = partial.to_lowercase();
    all_key_codes()
        .into_iter()
        .filter(|k| k.starts_with(&lower))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_keys() {
        assert!(is_valid_key("a"));
        assert!(is_valid_key("f1"));
        assert!(is_valid_key("return_or_enter"));
        assert!(!is_valid_key("invalid_key"));
    }

    #[test]
    fn test_autocomplete() {
        let suggestions = autocomplete("f1");
        assert!(suggestions.contains(&"f1"));
        assert!(suggestions.contains(&"f10"));
        assert!(suggestions.contains(&"f11"));
        assert!(suggestions.contains(&"f12"));
    }
}
