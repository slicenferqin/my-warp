use super::{localized_action_label_in_locale, matched_indices_for_localized_label, Locale};

#[test]
fn localized_action_label_keeps_english_fallback_unchanged() {
    assert_eq!(
        localized_action_label_in_locale(
            Locale::En,
            "workspace:show_settings_billing_and_usage_page",
            "Open Settings: Billing And Usage",
        ),
        "Open Settings: Billing And Usage"
    );
}

#[test]
fn localized_action_label_translates_known_command_palette_actions() {
    assert_eq!(
        localized_action_label_in_locale(Locale::ZhCn, "workspace:show_settings", "Open Settings",),
        "打开设置"
    );
    assert_eq!(
        localized_action_label_in_locale(
            Locale::ZhCn,
            "workspace:show_settings_keyboard_shortcuts_page",
            "Open Settings: Keyboard Shortcuts",
        ),
        "打开设置：快捷键"
    );
    assert_eq!(
        localized_action_label_in_locale(
            Locale::ZhCn,
            "workspace:show_theme_chooser",
            "Open Theme Picker",
        ),
        "打开主题选择器"
    );
}

#[test]
fn localized_action_label_falls_back_for_unknown_bindings() {
    assert_eq!(
        localized_action_label_in_locale(
            Locale::ZhCn,
            "workspace:unknown_action",
            "Unknown Action",
        ),
        "Unknown Action"
    );
}

#[test]
fn localized_labels_do_not_reuse_source_highlight_indices() {
    assert_eq!(
        matched_indices_for_localized_label("打开设置", "Open Settings", &[0, 1, 6, 7]),
        Vec::<usize>::new()
    );
    assert_eq!(
        matched_indices_for_localized_label("Open Settings", "Open Settings", &[0, 1, 6, 7]),
        vec![0, 1, 6, 7]
    );
}
