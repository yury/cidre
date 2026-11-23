#![cfg(all(target_os = "macos", feature = "ns"))]

use cidre::{ns, objc};

#[test]
fn missing_defaults_returns_none() {
    objc::ar_pool(|| {
        let suite_name = ns::String::with_str(&format!(
            "org.cidre.tests.nullable-user-defaults.{}",
            std::process::id()
        ));
        let defaults = ns::UserDefaults::with_suite_name(Some(&suite_name))
            .expect("failed to create isolated defaults suite");
        let key = ns::str!(c"cidreMissingValue");
        assert!(
            defaults.get_string_ar(key).is_none(),
            "expected Foundation to return nil for the isolated key"
        );

        eprintln!("checking missing UserDefaults value");
        assert!(
            defaults.get_string(key).is_none(),
            "missing stringForKey must return None"
        );
        assert!(
            defaults.get_obj(key).is_none(),
            "missing objectForKey must return None"
        );
    });
}
