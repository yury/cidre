use cidre::{ns, objc};

#[test]
fn caught_exception_survives_pool() {
    let reason = ns::str!(c"retained exception");
    let exception = {
        let _pool = objc::AutoreleasePoolPage::push();
        ns::try_catch(|| ns::Exception::raise(reason)).expect_err("result")
    };

    assert!(exception.reason().unwrap().eq(reason));
}

#[test]
fn caught_obj_survives_pool() {
    let expected = ns::str!(c"this is longer string so it is not tagged");
    let exception = {
        let _pool = objc::AutoreleasePoolPage::push();
        let message = ns::String::with_str("this is longer string so it is not tagged");
        let exception = objc::try_catch(|| objc::throw(&message)).expect_err("result");
        drop(message);
        exception
    };

    assert!(expected.is_equal(&exception));
}
