use crate::{arc, cf};

/// Returns the window server session dictionary for the current Quartz GUI session.
///
/// Returns `None` when the caller is not running within a Quartz GUI session or the
/// window server is disabled.
#[doc(alias = "CGSessionCopyCurrentDictionary")]
#[inline]
pub fn current_dictionary() -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Type>>> {
    unsafe { CGSessionCopyCurrentDictionary() }
}

pub mod keys {
    use crate::cf;

    /// The current session user ID as a `cf::Number` 32-bit unsigned integer value.
    #[doc(alias = "kCGSessionUserIDKey")]
    #[inline]
    pub fn user_id() -> &'static cf::String {
        cf::str!(c"kCGSSessionUserIDKey")
    }

    /// The current session short user name as a `cf::String` value.
    #[doc(alias = "kCGSessionUserNameKey")]
    #[inline]
    pub fn user_name() -> &'static cf::String {
        cf::str!(c"kCGSSessionUserNameKey")
    }

    /// The console set as a `cf::Number` 32-bit unsigned integer value.
    #[doc(alias = "kCGSessionConsoleSetKey")]
    #[inline]
    pub fn console_set() -> &'static cf::String {
        cf::str!(c"kCGSSessionConsoleSetKey")
    }

    /// Whether the session is on a console as a `cf::Boolean` value.
    #[doc(alias = "kCGSessionOnConsoleKey")]
    #[inline]
    pub fn on_console() -> &'static cf::String {
        cf::str!(c"kCGSSessionOnConsoleKey")
    }

    /// Whether the login operation has completed as a `cf::Boolean` value.
    #[doc(alias = "kCGSessionLoginDoneKey")]
    #[inline]
    pub fn login_done() -> &'static cf::String {
        cf::str!(c"kCGSessionLoginDoneKey")
    }
}

pub mod notifications {
    use std::ffi::CStr;

    /// Posted via `notify_post` when the GUI session on a console changes.
    #[doc(alias = "kCGNotifyGUIConsoleSessionChanged")]
    #[inline]
    pub const fn gui_console_session_changed() -> &'static CStr {
        c"com.apple.coregraphics.GUIConsoleSessionChanged"
    }

    /// Posted via `notify_post` when a user logs in or out of a GUI session.
    #[doc(alias = "kCGNotifyGUISessionUserChanged")]
    #[inline]
    pub const fn gui_session_user_changed() -> &'static CStr {
        c"com.apple.coregraphics.GUISessionUserChanged"
    }
}

unsafe extern "C-unwind" {
    fn CGSessionCopyCurrentDictionary() -> Option<arc::R<cf::DictionaryOf<cf::String, cf::Type>>>;
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use crate::cg;

    #[test]
    fn basics() {
        let _ = cg::session_notifications::gui_console_session_changed();
        let _ = cg::session_notifications::gui_session_user_changed();

        if let Some(dict) = cg::session_current_dictionary() {
            assert!(!dict.is_empty());
            let _ = dict.get(cg::session_keys::user_id());
            let _ = dict.get(cg::session_keys::user_name());
            let _ = dict.get(cg::session_keys::console_set());
            let _ = dict.get(cg::session_keys::on_console());
            let _ = dict.get(cg::session_keys::login_done());
        }
    }
}
