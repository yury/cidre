use crate::{arc, cf};

/// Scope for font registration. A uses session refers to a login session in macOS,
/// and the current booted session in iOS.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum Scope {
    /// The font is not registered and does not participate in font descriptor matching.
    /// This isn't a valid scope to specify while registering fonts.
    None = 0,

    /// The font is available to the current process for the duration of the process
    /// unless directly unregistered.
    Process = 1,

    /// The font is available to the current user session, and will not be available in
    /// subsequent sessions.
    /// Session scope is only available in macOS.
    #[cfg(target_os = "macos")]
    Session = 3,

    /// The font is available to all processes for the current user session and will be
    /// available in subsequent sessions unless unregistered.
    User = 2,
}

pub struct FontManager;

impl FontManager {
    #[doc(alias = "CTFontManagerCopyAvailablePostScriptNames")]
    #[inline]
    pub fn available_post_script_names() -> arc::R<cf::ArrayOf<cf::String>> {
        unsafe { CTFontManagerCopyAvailablePostScriptNames() }
    }

    #[doc(alias = "CTFontManagerCopyAvailableFontFamilyNames")]
    #[inline]
    pub fn available_font_family_names() -> arc::R<cf::ArrayOf<cf::String>> {
        unsafe { CTFontManagerCopyAvailableFontFamilyNames() }
    }

    /// Registers fonts from the specified font URL with the font manager.
    /// Registered fonts participate in font descriptor matching.
    #[doc(alias = "CTFontManagerRegisterFontsForURL")]
    pub fn register_fonts_for_url(
        font_url: &cf::Url,
        scope: Scope,
    ) -> Result<(), arc::R<cf::Error>> {
        cf::if_false(|err| unsafe { CTFontManagerRegisterFontsForURL(font_url, scope, err) })
    }

    #[doc(alias = "CTFontManagerUnregisterFontsForURL")]
    pub fn unregister_fonts_for_url(
        font_url: &cf::Url,
        scope: Scope,
    ) -> Result<(), arc::R<cf::Error>> {
        cf::if_false(|err| unsafe { CTFontManagerUnregisterFontsForURL(font_url, scope, err) })
    }
}

#[link(name = "CoreText", kind = "framework")]
extern "C-unwind" {
    fn CTFontManagerCopyAvailablePostScriptNames() -> arc::R<cf::ArrayOf<cf::String>>;
    fn CTFontManagerCopyAvailableFontFamilyNames() -> arc::R<cf::ArrayOf<cf::String>>;
    fn CTFontManagerRegisterFontsForURL(
        font_url: &cf::Url,
        scope: Scope,
        error: *mut arc::R<cf::Error>,
    ) -> bool;
    fn CTFontManagerUnregisterFontsForURL(
        font_url: &cf::Url,
        scope: Scope,
        error: *mut arc::R<cf::Error>,
    ) -> bool;

}

#[cfg(test)]
mod tests {
    use crate::{cf, ct};

    #[test]
    fn basics() {
        let fonts = ct::FontManager::available_post_script_names();
        assert!(fonts.len() > 0);
        // eprintln!("fonts {:?}\ntotal {}", fonts, fonts.len());

        let fonts = ct::FontManager::available_font_family_names();

        assert!(fonts.len() > 0);
        // eprintln!("fonts {:?}\ntotal {}", fonts, fonts.len());
    }

    #[test]
    fn register_error() {
        let res = ct::FontManager::register_fonts_for_url(
            cf::Url::from_str("file:///tmp").unwrap().as_ref(),
            ct::FontManagerScope::User,
        )
        .expect_err("Should fail");
        res.show();
    }
}
