use std::ffi::c_void;

use crate::{arc, cf, define_cf_type, os};

define_cf_type!(
    #[doc(alias = "TISInputSourceRef")]
    #[doc(alias = "TISInputSource")]
    InputSrc(cf::Type)
);

impl InputSrc {
    #[doc(alias = "TISInputSourceGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { TISInputSourceGetTypeID() }
    }

    #[doc(alias = "TISGetInputSourceProperty")]
    #[inline]
    pub fn prop(&self, key: &cf::String) -> Option<&cf::Type> {
        unsafe { self.prop_as(key) }
    }

    /// `kTISPropertyIconRef` is not a Core Foundation object. Use `prop_as`
    /// with the appropriate target type for that key.
    #[doc(alias = "TISGetInputSourceProperty")]
    #[inline]
    pub unsafe fn prop_as<T>(&self, key: &cf::String) -> Option<&T> {
        unsafe { TISGetInputSourceProperty(self, key).cast::<T>().as_ref() }
    }

    #[doc(alias = "TISSelectInputSource")]
    #[inline]
    pub fn select(&self) -> os::Result {
        unsafe { TISSelectInputSource(self).result() }
    }

    #[doc(alias = "TISDeselectInputSource")]
    #[inline]
    pub fn deselect(&self) -> os::Result {
        unsafe { TISDeselectInputSource(self).result() }
    }

    #[doc(alias = "TISEnableInputSource")]
    #[inline]
    pub fn enable(&self) -> os::Result {
        unsafe { TISEnableInputSource(self).result() }
    }

    #[doc(alias = "TISDisableInputSource")]
    #[inline]
    pub fn disable(&self) -> os::Result {
        unsafe { TISDisableInputSource(self).result() }
    }

    #[doc(alias = "TISCreateInputSourceList")]
    #[inline]
    pub fn list(
        props: Option<&cf::DictionaryOf<cf::String, cf::Type>>,
        include_all_installed: bool,
    ) -> Option<arc::R<cf::ArrayOf<InputSrc>>> {
        unsafe { TISCreateInputSourceList(props, include_all_installed) }
    }

    #[doc(alias = "TISCopyCurrentKeyboardInputSource")]
    #[inline]
    pub fn current_kb() -> Option<arc::R<InputSrc>> {
        unsafe { TISCopyCurrentKeyboardInputSource() }
    }

    #[doc(alias = "TISCopyCurrentKeyboardLayoutInputSource")]
    #[inline]
    pub fn current_kb_layout() -> Option<arc::R<InputSrc>> {
        unsafe { TISCopyCurrentKeyboardLayoutInputSource() }
    }

    #[doc(alias = "TISCopyCurrentASCIICapableKeyboardInputSource")]
    #[inline]
    pub fn current_ascii_capable_kb() -> Option<arc::R<InputSrc>> {
        unsafe { TISCopyCurrentASCIICapableKeyboardInputSource() }
    }

    #[doc(alias = "TISCopyCurrentASCIICapableKeyboardLayoutInputSource")]
    #[inline]
    pub fn current_ascii_capable_kd_layout() -> Option<arc::R<InputSrc>> {
        unsafe { TISCopyCurrentASCIICapableKeyboardLayoutInputSource() }
    }

    #[doc(alias = "TISCopyInputSourceForLanguage")]
    #[inline]
    pub fn for_lang(language: &cf::String) -> Option<arc::R<InputSrc>> {
        unsafe { TISCopyInputSourceForLanguage(language) }
    }

    #[doc(alias = "TISCreateASCIICapableInputSourceList")]
    #[inline]
    pub fn list_ascii_capable() -> Option<arc::R<cf::ArrayOf<InputSrc>>> {
        unsafe { TISCreateASCIICapableInputSourceList() }
    }

    #[doc(alias = "TISRegisterInputSource")]
    #[inline]
    pub fn register(location: &cf::Url) -> os::Result {
        unsafe { TISRegisterInputSource(location).result() }
    }
}
/// Typed props accessors
impl InputSrc {
    #[inline]
    pub fn id(&self) -> Option<&cf::String> {
        unsafe { self.prop_as(props::input_src_id()) }
    }

    #[inline]
    pub fn is_selected(&self) -> Option<bool> {
        let val = unsafe { self.prop_as::<cf::Boolean>(props::input_src_is_selected()) };
        val.map(|v| v.value())
    }

    #[inline]
    pub fn langs(&self) -> Option<&cf::ArrayOf<cf::String>> {
        unsafe { self.prop_as(props::input_src_langs()) }
    }
}

#[doc(alias = "TISSetInputMethodKeyboardLayoutOverride")]
#[inline]
pub fn set_input_method_kb_layout_override(keyboard_layout: &InputSrc) -> os::Result {
    unsafe { TISSetInputMethodKeyboardLayoutOverride(keyboard_layout).result() }
}

#[doc(alias = "TISCopyInputMethodKeyboardLayoutOverride")]
#[inline]
pub fn input_method_kb_layout_override() -> Option<arc::R<InputSrc>> {
    unsafe { TISCopyInputMethodKeyboardLayoutOverride() }
}

pub mod props {
    use crate::cf;

    #[doc(alias = "kTISPropertyInputSourceCategory")]
    #[inline]
    pub fn input_src_category() -> &'static cf::String {
        unsafe { kTISPropertyInputSourceCategory }
    }

    #[doc(alias = "kTISPropertyInputSourceType")]
    #[inline]
    pub fn input_src_type() -> &'static cf::String {
        unsafe { kTISPropertyInputSourceType }
    }

    #[doc(alias = "kTISPropertyInputSourceIsASCIICapable")]
    #[inline]
    pub fn input_src_is_ascii_capable() -> &'static cf::String {
        unsafe { kTISPropertyInputSourceIsASCIICapable }
    }

    #[doc(alias = "kTISPropertyInputSourceIsEnableCapable")]
    #[inline]
    pub fn input_src_is_enable_capable() -> &'static cf::String {
        unsafe { kTISPropertyInputSourceIsEnableCapable }
    }

    #[doc(alias = "kTISPropertyInputSourceIsSelectCapable")]
    #[inline]
    pub fn input_src_is_select_capable() -> &'static cf::String {
        unsafe { kTISPropertyInputSourceIsSelectCapable }
    }

    #[doc(alias = "kTISPropertyInputSourceIsEnabled")]
    #[inline]
    pub fn input_src_is_enabled() -> &'static cf::String {
        unsafe { kTISPropertyInputSourceIsEnabled }
    }

    #[doc(alias = "kTISPropertyInputSourceIsSelected")]
    #[inline]
    pub fn input_src_is_selected() -> &'static cf::String {
        unsafe { kTISPropertyInputSourceIsSelected }
    }

    #[doc(alias = "kTISPropertyInputSourceID")]
    #[inline]
    pub fn input_src_id() -> &'static cf::String {
        unsafe { kTISPropertyInputSourceID }
    }

    #[doc(alias = "kTISPropertyBundleID")]
    #[inline]
    pub fn bundle_id() -> &'static cf::String {
        unsafe { kTISPropertyBundleID }
    }

    #[doc(alias = "kTISPropertyInputModeID")]
    #[inline]
    pub fn input_mode_id() -> &'static cf::String {
        unsafe { kTISPropertyInputModeID }
    }

    #[doc(alias = "kTISPropertyLocalizedName")]
    #[inline]
    pub fn localized_name() -> &'static cf::String {
        unsafe { kTISPropertyLocalizedName }
    }

    #[doc(alias = "kTISPropertyInputSourceLanguages")]
    #[inline]
    pub fn input_src_langs() -> &'static cf::String {
        unsafe { kTISPropertyInputSourceLanguages }
    }

    #[doc(alias = "kTISPropertyUnicodeKeyLayoutData")]
    #[inline]
    pub fn unicode_key_layout_data() -> &'static cf::String {
        unsafe { kTISPropertyUnicodeKeyLayoutData }
    }

    #[doc(alias = "kTISPropertyIconRef")]
    #[inline]
    pub fn icon_ref() -> &'static cf::String {
        unsafe { kTISPropertyIconRef }
    }

    #[doc(alias = "kTISPropertyIconImageURL")]
    #[inline]
    pub fn icon_image_url() -> &'static cf::String {
        unsafe { kTISPropertyIconImageURL }
    }

    unsafe extern "C" {
        static kTISPropertyInputSourceCategory: &'static cf::String;
        static kTISPropertyInputSourceType: &'static cf::String;
        static kTISPropertyInputSourceIsASCIICapable: &'static cf::String;
        static kTISPropertyInputSourceIsEnableCapable: &'static cf::String;
        static kTISPropertyInputSourceIsSelectCapable: &'static cf::String;
        static kTISPropertyInputSourceIsEnabled: &'static cf::String;
        static kTISPropertyInputSourceIsSelected: &'static cf::String;
        static kTISPropertyInputSourceID: &'static cf::String;
        static kTISPropertyBundleID: &'static cf::String;
        static kTISPropertyInputModeID: &'static cf::String;
        static kTISPropertyLocalizedName: &'static cf::String;
        static kTISPropertyInputSourceLanguages: &'static cf::String;
        static kTISPropertyUnicodeKeyLayoutData: &'static cf::String;
        static kTISPropertyIconRef: &'static cf::String;
        static kTISPropertyIconImageURL: &'static cf::String;
    }
}

pub mod categories {
    use crate::cf;

    #[doc(alias = "kTISCategoryKeyboardInputSource")]
    #[inline]
    pub fn kb() -> &'static cf::String {
        unsafe { kTISCategoryKeyboardInputSource }
    }

    #[doc(alias = "kTISCategoryPaletteInputSource")]
    #[inline]
    pub fn palette() -> &'static cf::String {
        unsafe { kTISCategoryPaletteInputSource }
    }

    #[doc(alias = "kTISCategoryInkInputSource")]
    #[inline]
    pub fn ink() -> &'static cf::String {
        unsafe { kTISCategoryInkInputSource }
    }

    unsafe extern "C" {
        static kTISCategoryKeyboardInputSource: &'static cf::String;
        static kTISCategoryPaletteInputSource: &'static cf::String;
        static kTISCategoryInkInputSource: &'static cf::String;
    }
}

pub mod src_types {
    use crate::cf;

    #[doc(alias = "kTISTypeKeyboardLayout")]
    #[inline]
    pub fn kb_layout() -> &'static cf::String {
        unsafe { kTISTypeKeyboardLayout }
    }

    #[doc(alias = "kTISTypeKeyboardInputMethodWithoutModes")]
    #[inline]
    pub fn kb_input_method_without_modes() -> &'static cf::String {
        unsafe { kTISTypeKeyboardInputMethodWithoutModes }
    }

    #[doc(alias = "kTISTypeKeyboardInputMethodModeEnabled")]
    #[inline]
    pub fn kb_input_method_mode_enabled() -> &'static cf::String {
        unsafe { kTISTypeKeyboardInputMethodModeEnabled }
    }

    #[doc(alias = "kTISTypeKeyboardInputMode")]
    #[inline]
    pub fn kb_input_mode() -> &'static cf::String {
        unsafe { kTISTypeKeyboardInputMode }
    }

    #[doc(alias = "kTISTypeCharacterPalette")]
    #[inline]
    pub fn character_palette() -> &'static cf::String {
        unsafe { kTISTypeCharacterPalette }
    }

    #[doc(alias = "kTISTypeKeyboardViewer")]
    #[inline]
    pub fn kb_viewer() -> &'static cf::String {
        unsafe { kTISTypeKeyboardViewer }
    }

    #[doc(alias = "kTISTypeInk")]
    #[inline]
    pub fn ink() -> &'static cf::String {
        unsafe { kTISTypeInk }
    }

    unsafe extern "C" {
        static kTISTypeKeyboardLayout: &'static cf::String;
        static kTISTypeKeyboardInputMethodWithoutModes: &'static cf::String;
        static kTISTypeKeyboardInputMethodModeEnabled: &'static cf::String;
        static kTISTypeKeyboardInputMode: &'static cf::String;
        static kTISTypeCharacterPalette: &'static cf::String;
        static kTISTypeKeyboardViewer: &'static cf::String;
        static kTISTypeInk: &'static cf::String;
    }
}

pub mod notifications {
    use crate::cf;

    #[doc(alias = "kTISNotifySelectedKeyboardInputSourceChanged")]
    #[inline]
    pub fn selected_kb_input_src_changed() -> &'static cf::NotificationName {
        unsafe { kTISNotifySelectedKeyboardInputSourceChanged }
    }

    #[doc(alias = "kTISNotifyEnabledKeyboardInputSourcesChanged")]
    #[inline]
    pub fn enabled_kb_input_srcs_changed() -> &'static cf::NotificationName {
        unsafe { kTISNotifyEnabledKeyboardInputSourcesChanged }
    }

    unsafe extern "C" {
        static kTISNotifySelectedKeyboardInputSourceChanged: &'static cf::NotificationName;
        static kTISNotifyEnabledKeyboardInputSourcesChanged: &'static cf::NotificationName;
    }
}

unsafe extern "C-unwind" {
    fn TISInputSourceGetTypeID() -> cf::TypeId;

    fn TISGetInputSourceProperty(
        input_source: &InputSrc,
        property_key: &cf::String,
    ) -> *const c_void;

    fn TISCreateInputSourceList(
        properties: Option<&cf::DictionaryOf<cf::String, cf::Type>>,
        include_all_installed: bool,
    ) -> Option<arc::R<cf::ArrayOf<InputSrc>>>;

    fn TISCopyCurrentKeyboardInputSource() -> Option<arc::R<InputSrc>>;
    fn TISCopyCurrentKeyboardLayoutInputSource() -> Option<arc::R<InputSrc>>;
    fn TISCopyCurrentASCIICapableKeyboardInputSource() -> Option<arc::R<InputSrc>>;
    fn TISCopyCurrentASCIICapableKeyboardLayoutInputSource() -> Option<arc::R<InputSrc>>;
    fn TISCopyInputSourceForLanguage(language: &cf::String) -> Option<arc::R<InputSrc>>;
    fn TISCreateASCIICapableInputSourceList() -> Option<arc::R<cf::ArrayOf<InputSrc>>>;

    fn TISSelectInputSource(input_source: &InputSrc) -> os::Status;
    fn TISDeselectInputSource(input_source: &InputSrc) -> os::Status;
    fn TISEnableInputSource(input_source: &InputSrc) -> os::Status;
    fn TISDisableInputSource(input_source: &InputSrc) -> os::Status;

    fn TISSetInputMethodKeyboardLayoutOverride(keyboard_layout: &InputSrc) -> os::Status;
    fn TISCopyInputMethodKeyboardLayoutOverride() -> Option<arc::R<InputSrc>>;

    fn TISRegisterInputSource(location: &cf::Url) -> os::Status;
}

#[link(name = "Carbon", kind = "framework")]
unsafe extern "C" {}

#[cfg(test)]
mod tests {
    use crate::{cf, tis};

    #[test]
    fn basics() {
        let props = cf::DictionaryOf::with_keys_values(
            &[
                tis::props::input_src_category(),
                tis::props::input_src_is_enable_capable(),
                tis::props::input_src_is_select_capable(),
            ],
            &[
                tis::categories::kb().as_type_ref(),
                cf::Boolean::value_true(),
                cf::Boolean::value_true(),
            ],
        );
        let list = tis::InputSrc::list(Some(props.as_ref()), false).unwrap();
        assert!(!list.is_empty());

        let current = tis::InputSrc::current_kb_layout().unwrap();
        assert!(current.id().is_some());
        assert!(current.is_selected().unwrap());
    }
}
