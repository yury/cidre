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

    /// Picker mode for multiple application selection
    pub const MULTIPLE_APPS: Self = Self(1 << 3);

    /// picker mode for full display selection
    pub const SINGLE_DISPLAY: Self = Self(1 << 4);
}

define_obj_type!(
    /// Configuration used to customize the content sharing picker.
    #[doc(alias = "SCContentSharingPickerConfiguration")]
    pub Cfg(ns::Id),
    SC_CONTENT_SHARING_PICKER_CONFIGURATION,
    #[api::available(
        macos = 14.0,
        maccatalyst = 18.2,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
);

impl Cfg {
    /// The selection modes allowed by the picker.
    #[objc::msg_send(allowedPickerModes)]
    #[api::available(macos = 14.0, maccatalyst = 18.2)]
    pub fn allowed_picker_modes(&self) -> Mode;

    /// Sets the selection modes allowed by the picker.
    #[objc::msg_send(setAllowedPickerModes:)]
    #[api::available(macos = 14.0, maccatalyst = 18.2)]
    pub fn set_allowed_picker_modes(&mut self, val: Mode);

    /// Window IDs to exclude from picking.
    #[objc::msg_send(excludedWindowIDs)]
    #[api::available(macos = 14.0, maccatalyst = 18.2)]
    pub fn excluded_window_ids(&self) -> arc::R<ns::Array<ns::Number>>;

    /// Sets the window IDs to exclude from picking.
    #[objc::msg_send(setExcludedWindowIDs:)]
    #[api::available(macos = 14.0, maccatalyst = 18.2)]
    pub fn set_excluded_window_ids(&mut self, val: &ns::Array<ns::Number>);

    /// Bundle identifiers to exclude from picking.
    #[objc::msg_send(excludedBundleIDs)]
    #[api::available(macos = 14.0, maccatalyst = 18.2)]
    pub fn excluded_bundle_ids(&self) -> arc::R<ns::Array<ns::String>>;

    /// Sets the bundle identifiers to exclude from picking.
    #[objc::msg_send(setExcludedBundleIDs:)]
    #[api::available(macos = 14.0, maccatalyst = 18.2)]
    pub fn set_excluded_bundle_ids(&mut self, val: &ns::Array<ns::String>);

    /// Controls if the user can make updates to the content filter after the initial selection. Defaults is true.
    #[objc::msg_send(allowsChangingSelectedContent)]
    #[api::available(macos = 14.0, maccatalyst = 18.2)]
    pub fn allows_changing_selected_content(&self) -> bool;

    /// Sets whether the user can update the selected content after initial selection.
    #[objc::msg_send(setAllowsChangingSelectedContent:)]
    #[api::available(macos = 14.0, maccatalyst = 18.2)]
    pub fn set_allows_changing_selected_content(&mut self, val: bool);

    /// Whether the picker shows a microphone control.
    #[objc::msg_send(showsMicrophoneControl)]
    #[api::available(ios = 27.0, visionos = 27.0)]
    pub fn shows_mic_control(&self) -> bool;

    /// Sets whether the picker shows a microphone control.
    #[objc::msg_send(setShowsMicrophoneControl:)]
    #[api::available(ios = 27.0, visionos = 27.0)]
    pub fn set_shows_mic_control(&mut self, val: bool);

    /// Whether the picker shows a camera control.
    #[objc::msg_send(showsCameraControl)]
    #[api::available(ios = 27.0)]
    pub fn shows_camera_control(&self) -> bool;

    /// Sets whether the picker shows a camera control.
    #[objc::msg_send(setShowsCameraControl:)]
    #[api::available(ios = 27.0)]
    pub fn set_shows_camera_control(&mut self, val: bool);
}

define_obj_type!(
    /// The system content sharing picker.
    #[doc(alias = "SCContentSharingPicker")]
    pub Picker(ns::Id)
);

impl Picker {
    #[api::available(
        macos = 14.0,
        maccatalyst = 18.2,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    crate::define_cls!(SC_CONTENT_SHARING_PICKER);

    /// The shared content sharing picker.
    #[objc::msg_send(sharedPicker)]
    #[api::available(
        macos = 14.0,
        maccatalyst = 18.2,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    pub fn shared() -> arc::R<Self>;

    /// Default cfg for the content sharing picker. If a stream does not have a configuration, the default configuration will be used.
    #[objc::msg_send(defaultConfiguration)]
    pub fn default_cfg(&self) -> arc::R<Cfg>;

    #[objc::msg_send(setDefaultConfiguration:)]
    pub fn set_default_cfg(&mut self, val: &Cfg);

    /// The maximum number of streams the picker can configure.
    #[objc::msg_send(maximumStreamCount)]
    #[api::available(macos = 14.0, maccatalyst = 18.2)]
    pub fn max_stream_count(&self) -> Option<arc::R<ns::Number>>;

    /// Sets the maximum number of streams the picker can configure.
    #[objc::msg_send(setMaximumStreamCount:)]
    #[api::available(macos = 14.0, maccatalyst = 18.2)]
    pub fn set_max_stream_count(&mut self, val: Option<&ns::Number>);

    /// Whether the picker is active.
    #[objc::msg_send(active)]
    pub fn active(&self) -> bool;

    /// Sets whether the picker is active.
    #[objc::msg_send(setActive:)]
    pub fn set_active(&mut self, val: bool);

    /// Whether content sharing picker presentation is currently available.
    #[objc::msg_send(isAvailable)]
    #[api::available(
        macos = 27.0,
        maccatalyst = 27.0,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    pub fn is_available(&self) -> bool;

    /// Adds an observer for picker selection changes.
    #[objc::msg_send(addObserver:)]
    pub fn add_observer<O: Observer>(&mut self, val: &O);

    /// Removes an observer for picker selection changes.
    #[objc::msg_send(removeObserver:)]
    pub fn remove_observer<O: Observer>(&mut self, val: &O);

    /// Sets a picker configuration for a specific stream.
    #[objc::msg_send(setConfiguration:forStream:)]
    #[api::available(macos = 14.0, maccatalyst = 18.2)]
    pub fn set_cfg_for_stream(&mut self, cfg: Option<&Cfg>, stream: &sc::Stream);

    /// Presents the picker.
    #[objc::msg_send(present)]
    #[api::available(macos = 14.0, maccatalyst = 18.2, ios = 27.0, visionos = 27.0)]
    pub fn present(&mut self);

    /// Presents the picker using the requested content style.
    #[objc::msg_send(presentPickerUsingContentStyle:)]
    #[api::available(
        macos = 14.0,
        maccatalyst = 18.2,
        ios = 27.0,
        visionos = 27.0,
        tvos = 27.0
    )]
    pub fn present_using_content_style(&mut self, style: sc::ShareableContentStyle);

    /// Presents the picker for a specific stream.
    #[objc::msg_send(presentPickerForStream:)]
    #[api::available(macos = 14.0, maccatalyst = 18.2)]
    pub fn present_for_stream(&mut self, stream: &sc::Stream);

    /// Presents the picker for a specific stream and content style.
    #[objc::msg_send(presentPickerForStream:usingContentStyle:)]
    #[api::available(macos = 14.0, maccatalyst = 18.2)]
    pub fn present_for_stream_using_content_style(
        &mut self,
        stream: &sc::Stream,
        style: sc::ShareableContentStyle,
    );

    /// Presents the picker for the current application.
    #[objc::msg_send(presentPickerForCurrentApplication)]
    #[api::available(ios = 27.0, visionos = 27.0, tvos = 27.0)]
    pub fn present_for_current_app(&mut self);
}

#[objc::protocol(SCContentSharingPickerObserver)]
/// Receives content sharing picker selection callbacks.
pub trait Observer: objc::Obj {
    /// The observer callback method when the picker has been canceled with no selection.
    #[objc::msg_send(contentSharingPicker:didCancelForStream:)]
    fn picker_did_cancel_for_stream(&mut self, picker: &mut Picker, stream: Option<&sc::Stream>);

    /// Called when the picker updates the selected content filter.
    #[objc::msg_send(contentSharingPicker:didUpdateWithFilter:forStream:)]
    fn picker_did_update_with_filter_for_stream(
        &mut self,
        picker: &mut Picker,
        filter: &sc::ContentFilter,
        stream: Option<&sc::Stream>,
    );

    /// Called when picker startup fails.
    #[objc::msg_send(contentSharingPickerStartDidFailWithError:)]
    fn picker_start_did_fail_with_err(&mut self, err: &ns::Error);
}

unsafe extern "C" {
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
        assert!(
            cfg.allowed_picker_modes()
                .contains(sc::ContentSharingPickerMode::SINGLE_WINDOW)
        );
    }
}
