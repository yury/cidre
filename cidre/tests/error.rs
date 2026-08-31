use cidre::{ns, objc};

#[test]
fn nil_result_error_survives_pool() {
    let error = {
        let _pool = objc::AutoreleasePoolPage::push();
        ns::Regex::with_pattern(ns::str!(c"\\"), Default::default()).expect_err("invalid regex")
    };

    assert_eq!(error.code(), 2048);
    assert!(!error.localized_desc().is_empty());
}

#[test]
fn false_result_error_survives_pool() {
    let error = {
        let _pool = objc::AutoreleasePoolPage::push();
        ns::FileManager::default()
            .remove_item_at_path(ns::str!(c"/cidre/path/that/does/not/exist"))
            .expect_err("missing path")
    };

    assert!(!error.domain().is_empty());
    assert!(!error.localized_desc().is_empty());
}
