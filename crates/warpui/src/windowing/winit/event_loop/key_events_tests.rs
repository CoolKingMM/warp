use winit::keyboard::Key::Character;
use winit::keyboard::{KeyCode, ModifiersState, NativeKeyCode, PhysicalKey, SmolStr};

use super::{
    effective_alt_key, get_input_key, should_suppress_alt_modified_control_chars,
    should_suppress_windows_ctrl_c_keydown, should_suppress_windows_ctrl_c_text,
    us_qwerty_fallback_for_chord,
};

#[test]
fn test_get_input_key() {
    // Tests all visible ASCII characters
    // TODO: it would be nice to test the following:
    // - non-Character keys (ex: named keys, dead keys)
    // - non-ascii characters to ensure shift behavior is appropriate
    for ascii_code in 32u8..127u8 {
        let input = ascii_code as char;
        let key = Character(SmolStr::from(input.to_string()));

        for shift in [false, true] {
            match get_input_key(&key, shift) {
                Character(new_value) => {
                    let new_char = new_value
                        .chars()
                        .next()
                        .expect("string should be non-empty");

                    let expected = match (input, shift) {
                        ('A'..='Z', false) => input
                            .to_lowercase()
                            .next()
                            .expect("string should be non-empty"),
                        // Case 2: a lower case letter when shift is true
                        // Should turn into upper case version
                        ('a'..='z', true) => input
                            .to_uppercase()
                            .next()
                            .expect("string should be non-empty"),
                        // Case 3: a character that should be unchanged by caps lock
                        // - An upper-case letter when shift is true
                        // - A lower-case letter when shift is false,
                        // - A non-alpha character
                        _ => input,
                    };
                    assert_eq!(
                        expected, new_char,
                        "Expected '{input}' -> '{expected}' when shift={shift}, but got '{new_char}'"
                    )
                }
                unexpected => {
                    panic!("Key '{key:?}' somehow became non-character {unexpected:?}")
                }
            }
        }
    }
}

#[test]
fn us_qwerty_fallback_maps_letters() {
    // Letters return lowercase regardless of shift; `get_input_key` applies the
    // uppercase transform downstream.
    let cases = [
        (KeyCode::KeyA, "a"),
        (KeyCode::KeyC, "c"),
        (KeyCode::KeyV, "v"),
        (KeyCode::KeyZ, "z"),
    ];
    for (code, expected) in cases {
        for shift in [false, true] {
            assert_eq!(
                us_qwerty_fallback_for_chord(&PhysicalKey::Code(code), shift),
                Some(expected),
                "expected {code:?} -> {expected} (shift={shift})",
            );
        }
    }
}

#[test]
fn us_qwerty_fallback_maps_digits_and_punctuation() {
    let cases = [
        (KeyCode::Digit0, "0"),
        (KeyCode::Digit9, "9"),
        (KeyCode::Minus, "-"),
        (KeyCode::Equal, "="),
        (KeyCode::Slash, "/"),
        (KeyCode::Backquote, "`"),
        (KeyCode::Semicolon, ";"),
        (KeyCode::Comma, ","),
    ];
    for (code, expected) in cases {
        assert_eq!(
            us_qwerty_fallback_for_chord(&PhysicalKey::Code(code), false),
            Some(expected),
            "expected {code:?} -> {expected}",
        );
    }
}

#[test]
fn us_qwerty_fallback_maps_shifted_digits_and_punctuation() {
    let cases = [
        (KeyCode::Digit1, "!"),
        (KeyCode::Digit2, "@"),
        (KeyCode::Digit6, "^"),
        (KeyCode::Digit9, "("),
        (KeyCode::Digit0, ")"),
        (KeyCode::Minus, "_"),
        (KeyCode::Equal, "+"),
        (KeyCode::BracketLeft, "{"),
        (KeyCode::BracketRight, "}"),
        (KeyCode::Backslash, "|"),
        (KeyCode::Semicolon, ":"),
        (KeyCode::Quote, "\""),
        (KeyCode::Comma, "<"),
        (KeyCode::Period, ">"),
        (KeyCode::Slash, "?"),
        (KeyCode::Backquote, "~"),
    ];
    for (code, expected) in cases {
        assert_eq!(
            us_qwerty_fallback_for_chord(&PhysicalKey::Code(code), true),
            Some(expected),
            "expected {code:?} + shift -> {expected}",
        );
    }
}

#[test]
fn us_qwerty_fallback_returns_none_for_unmapped_keys() {
    // Keys outside the chord-shortcut set should fall through so the original
    // logical_key is preserved.
    let unmapped = [
        KeyCode::F1,
        KeyCode::F13,
        KeyCode::AltLeft,
        KeyCode::ShiftRight,
        KeyCode::ControlLeft,
        KeyCode::Enter,
        KeyCode::Escape,
        KeyCode::ArrowUp,
        KeyCode::Tab,
    ];
    for code in unmapped {
        for shift in [false, true] {
            assert_eq!(
                us_qwerty_fallback_for_chord(&PhysicalKey::Code(code), shift),
                None,
                "{code:?} should not have a chord fallback (shift={shift})",
            );
        }
    }
}

#[test]
fn us_qwerty_fallback_returns_none_for_unidentified_physical_key() {
    let unidentified = PhysicalKey::Unidentified(NativeKeyCode::Unidentified);
    for shift in [false, true] {
        assert_eq!(
            us_qwerty_fallback_for_chord(&unidentified, shift),
            None,
            "unidentified key should not have a chord fallback (shift={shift})",
        );
    }
}

#[test]
fn effective_alt_key_includes_tracked_physical_alt() {
    assert!(!effective_alt_key(ModifiersState::empty(), false, false));
    assert!(effective_alt_key(ModifiersState::ALT, false, false));
    assert!(effective_alt_key(ModifiersState::empty(), true, false));
    assert!(effective_alt_key(ModifiersState::empty(), false, true));
}

#[test]
fn suppresses_alt_modified_control_chars_only() {
    assert!(should_suppress_alt_modified_control_chars(
        "\x03",
        ModifiersState::CONTROL,
        true,
    ));
    assert!(!should_suppress_alt_modified_control_chars(
        "\x03",
        ModifiersState::CONTROL,
        false,
    ));
    assert!(!should_suppress_alt_modified_control_chars(
        "\x03",
        ModifiersState::empty(),
        true,
    ));
    assert!(!should_suppress_alt_modified_control_chars(
        "c",
        ModifiersState::CONTROL,
        true,
    ));
    assert!(!should_suppress_alt_modified_control_chars(
        "",
        ModifiersState::CONTROL,
        true,
    ));
}

#[test]
fn suppresses_windows_ctrl_c_keydown_after_recent_alt_or_non_hardware_source() {
    assert_eq!(
        should_suppress_windows_ctrl_c_keydown("c", ModifiersState::CONTROL, true, false),
        cfg!(windows),
    );
    assert_eq!(
        should_suppress_windows_ctrl_c_keydown("c", ModifiersState::CONTROL, false, true),
        cfg!(windows),
    );
    assert!(!should_suppress_windows_ctrl_c_keydown(
        "c",
        ModifiersState::CONTROL,
        false,
        false,
    ));
    assert!(!should_suppress_windows_ctrl_c_keydown(
        "c",
        ModifiersState::empty(),
        true,
        true,
    ));
    assert!(!should_suppress_windows_ctrl_c_keydown(
        "v",
        ModifiersState::CONTROL,
        true,
        true,
    ));
}

#[test]
fn suppresses_windows_ctrl_c_text_after_recent_alt_or_non_hardware_source() {
    assert_eq!(
        should_suppress_windows_ctrl_c_text("\x03", ModifiersState::CONTROL, true, false),
        cfg!(windows),
    );
    assert_eq!(
        should_suppress_windows_ctrl_c_text("\x03", ModifiersState::empty(), false, true),
        cfg!(windows),
    );
    assert!(!should_suppress_windows_ctrl_c_text(
        "\x03",
        ModifiersState::CONTROL,
        false,
        false,
    ));
    assert!(!should_suppress_windows_ctrl_c_text(
        "\x03",
        ModifiersState::empty(),
        true,
        false,
    ));
    assert!(!should_suppress_windows_ctrl_c_text(
        "\x16",
        ModifiersState::CONTROL,
        true,
        true,
    ));
    assert!(!should_suppress_windows_ctrl_c_text(
        "c",
        ModifiersState::CONTROL,
        true,
        true,
    ));
}

#[cfg(windows)]
#[test]
fn drops_windows_alt_c_reported_as_ctrl_c_only() {
    assert!(super::should_drop_windows_alt_c_control_event(
        "c",
        "\x03",
        ModifiersState::CONTROL,
        true,
        false,
        false,
    ));
    assert!(!super::should_drop_windows_alt_c_control_event(
        "c",
        "\x03",
        ModifiersState::CONTROL,
        false,
        false,
        false,
    ));
    assert!(super::should_drop_windows_alt_c_control_event(
        "c",
        "",
        ModifiersState::CONTROL,
        false,
        false,
        true,
    ));
    assert!(super::should_drop_windows_alt_c_control_event(
        "c",
        "\x03",
        ModifiersState::CONTROL,
        false,
        true,
        false,
    ));
    assert!(super::should_drop_windows_alt_c_control_event(
        "c",
        "\x03",
        ModifiersState::empty(),
        false,
        false,
        true,
    ));
    assert!(!super::should_drop_windows_alt_c_control_event(
        "v",
        "\x16",
        ModifiersState::CONTROL,
        true,
        true,
        false,
    ));
    assert!(!super::should_drop_windows_alt_c_control_event(
        "c",
        "c",
        ModifiersState::CONTROL,
        true,
        true,
        true,
    ));
}
