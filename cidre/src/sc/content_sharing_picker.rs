use crate::{api, arc, define_obj_type, define_opts, ns, objc, sc};

define_opts!(
    #[doc(alias = "SCContentSharingPickerMode")]
    pub Mode(usize)
);

impl Mode {
    /// Picker mode for single window selection
    pub const SINGLE_WINDOW: Self = Self(1 << 0);

    /// Picker mode for multiple window selection
    pub const MULTIPLE_WINDOWS: Self = Self(1 << 1);

    /// Picker mode for application selection
    pub const SINGLE_APP: Self = Self(1 << 2);

    pub const MULTIPLE_APPS: Self = Self(1 << 3);

    /// picker mode for full display selection
    pub const SINGLE_DISPLAY: Self = Self(1 << 4);
}

define_obj_type!(
    #[doc(alias = "SCContentSharingPickerConfiguration")]
    pub Cfg(ns::Id),
    SC_CONTENT_SHARING_PICKER_CONFIGURATION,
    #[api::available(macos = 14.0)]
);

impl Cfg {
    #[objc::msg_send(allowedPickerModes)]
    pub fn allowed_picker_modes(&self) -> Mode;

    #[objc::msg_send(setAllowedPickerModes:)]
    pub fn set_allowed_picker_modes(&mut self, val: Mode);

    /// cg::WindowIds for picking
    #[objc::msg_send(excludedWindowIDs)]
    pub fn excluded_window_ids(&self) -> arc::R<ns::Array<ns::Number>>;

    #[objc::msg_send(setExcludedWindowIDs:)]
    pub fn set_excluded_window_ids(&mut self, val: &ns::Array<ns::Number>);

    #[objc::msg_send(excludedBundleIDs)]
    pub fn excluded_bundle_ids(&self) -> arc::R<ns::Array<ns::String>>;

    #[objc::msg_send(setExcludedBundleIDs:)]
    pub fn set_excluded_bundle_ids(&mut self, val: &ns::Array<ns::String>);

    /// Controls if the user can make updates to the content filter after the initial selection. Defaults is true.
    #[objc::msg_send(allowsChangingSelectedContent)]
    pub fn allows_changing_selected_content(&self) -> bool;

    #[objc::msg_send(setAllowsChangingSelectedContent:)]
    pub fn set_allows_changing_selected_content(&mut self, val: bool);
}

define_obj_type!(
    #[doc(alias = "SCContentSharingPicker")]
    pub Picker(ns::Id)
);

impl Picker {
    #[api::available(macos = 14.0)]
    crate::define_cls!(SC_CONTENT_SHARING_PICKER);

    #[objc::msg_send(sharedPicker)]
    #[api::available(macos = 14.0)]
    pub fn shared() -> arc::R<Self>;

    /// Default cfg for the content sharing picker. If a stream does not have a configuration, the default configuration will be used.
    #[objc::msg_send(defaultConfiguration)]
    pub fn default_cfg(&self) -> arc::R<Cfg>;

    #[objc::msg_send(setDefaultConfiguration:)]
    pub fn set_default_cfg(&mut self, val: &Cfg);

    #[objc::msg_send(maximumStreamCount)]
    pub fn max_stream_count(&self) -> Option<arc::R<ns::Number>>;

    #[objc::msg_send(setMaximumStreamCount:)]
    pub fn set_max_stream_count(&mut self, val: Option<&ns::Number>);

    #[objc::msg_send(active)]
    pub fn active(&self) -> bool;

    #[objc::msg_send(setActive:)]
    pub fn set_active(&mut self, val: bool);

    #[objc::msg_send(addObserver:)]
    pub fn add_observer<O: Observer>(&mut self, val: &O);

    #[objc::msg_send(removeObserver:)]
    pub fn remove_observer<O: Observer>(&mut self, val: &O);

    #[objc::msg_send(setConfiguration:forStream:)]
    pub fn set_cfg_for_stream(&mut self, cfg: Option<&Cfg>, stream: &sc::Stream);

    #[objc::msg_send(present)]
    pub fn present(&mut self);

    #[objc::msg_send(presentPickerUsingContentStyle:)]
    pub fn present_using_content_style(&mut self, style: sc::ShareableContentStyle);

    #[objc::msg_send(presentPickerForStream:)]
    pub fn present_for_stream(&mut self, stream: &sc::Stream);

    #[objc::msg_send(presentPickerForStream:usingContentStyle:)]
    pub fn present_for_stream_using_content_style(
        &mut self,
        stream: &sc::Stream,
        style: sc::ShareableContentStyle,
    );
}

#[objc::protocol(SCContentSharingPickerObserver)]
pub trait Observer: objc::Obj {
    /// The observer callback method when the picker has been canceled with no selection.
    #[objc::msg_send(contentSharingPicker:didCancelForStream:)]
    fn picker_did_cancel_for_stream(&mut self, picker: &mut Picker, stream: Option<&sc::Stream>);

    #[objc::msg_send(contentSharingPicker:didUpdateWithFilter:forStream:)]
    fn picker_did_update_with_filter_for_stream(
        &mut self,
        picker: &mut Picker,
        filter: &sc::ContentFilter,
        stream: Option<&sc::Stream>,
    );

    #[objc::msg_send(contentSharingPickerStartDidFailWithError:)]
    fn picker_start_did_fail_with_err(&mut self, err: &ns::Error);
}

#[link(name = "sc", kind = "static")]
extern "C" {
    static SC_CONTENT_SHARING_PICKER_CONFIGURATION: &'static objc::Class<Cfg>;
    static SC_CONTENT_SHARING_PICKER: &'static objc::Class<Picker>;
}

#[cfg(test)]
mod tests {
    use crate::sc;

    #[test]
    fn basics() {
        let picker = sc::ContentSharingPicker::shared();
        let cfg = picker.default_cfg();
        assert!(cfg
            .allowed_picker_modes()
            .contains(sc::ContentSharingPickerMode::SINGLE_WINDOW));
    }
}
