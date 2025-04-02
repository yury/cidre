use crate::{arc, ax, cf, define_cf_type, os, sys::Pid};

pub fn is_process_trusted() -> bool {
    is_process_trusted_with_opts(None)
}

pub fn is_process_trusted_with_prompt(prompt: bool) -> bool {
    let opts =
        cf::DictionaryOf::with_keys_values(&[trusted_check_option_prompt()], &[prompt.into()]);
    is_process_trusted_with_opts(Some(&opts))
}

#[doc(alias = "AXIsProcessTrustedWithOptions")]
pub fn is_process_trusted_with_opts(
    options: Option<&cf::DictionaryOf<cf::String, cf::Type>>,
) -> bool {
    unsafe { AXIsProcessTrustedWithOptions(options) }
}

pub fn trusted_check_option_prompt() -> &'static cf::String {
    unsafe { kAXTrustedCheckOptionPrompt }
}

define_cf_type!(
    #[doc(alias = "AXUIElementRef")]
    UiElement(cf::Type)
);

unsafe impl Send for UiElement {}

impl UiElement {
    #[doc(alias = "AXUIElementGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { AXUIElementGetTypeID() }
    }

    #[doc(alias = "AXUIElementCreateApplication")]
    #[inline]
    pub fn with_app_pid(pid: Pid) -> arc::R<Self> {
        unsafe { AXUIElementCreateApplication(pid) }
    }

    #[doc(alias = "AXUIElementCreateSystemWide")]
    #[inline]
    pub fn sys_wide() -> arc::R<Self> {
        unsafe { AXUIElementCreateSystemWide() }
    }

    #[doc(alias = "AXUIElementGetPid")]
    #[inline]
    pub fn pid(&self) -> os::Result<Pid> {
        os::result_init(|res| unsafe { AXUIElementGetPid(self, res) })
    }

    #[doc(alias = "AXUIElementCopyAttributeNames")]
    #[inline]
    pub fn attrs(&self) -> os::Result<arc::R<cf::ArrayOf<ax::Attr>>> {
        unsafe { os::result_unchecked(|res| AXUIElementCopyAttributeNames(self, res)) }
    }

    #[doc(alias = "AXUIElementCopyParameterizedAttributeNames")]
    #[inline]
    pub fn param_attrs(&self) -> os::Result<arc::R<cf::ArrayOf<ax::ParamAttr>>> {
        unsafe { os::result_unchecked(|res| AXUIElementCopyParameterizedAttributeNames(self, res)) }
    }

    #[doc(alias = "AXUIElementCopyAttributeValue")]
    #[inline]
    pub fn attr_value(&self, name: &ax::Attr) -> os::Result<arc::R<cf::Type>> {
        unsafe { os::result_unchecked(|res| AXUIElementCopyAttributeValue(self, name, res)) }
    }

    unsafe fn attr<T: arc::Retain>(&self, name: &ax::Attr) -> os::Result<arc::R<T>> {
        unsafe {
            std::mem::transmute(os::result_unchecked(|res| {
                AXUIElementCopyAttributeValue(self, name, res)
            }))
        }
    }

    #[doc(alias = "AXUIElementCopyActionNames")]
    #[inline]
    pub fn actions(&self) -> os::Result<arc::R<cf::ArrayOf<ax::Action>>> {
        unsafe { os::result_unchecked(|res| AXUIElementCopyActionNames(self, res)) }
    }

    pub fn action_desc(&self, action: &ax::Action) -> os::Result<arc::R<cf::String>> {
        unsafe { os::result_unchecked(|res| AXUIElementCopyActionDescription(self, action, res)) }
    }

    #[doc(alias = "AXUIElementSetMessagingTimeout")]
    #[inline]
    pub fn set_messaging_timeout_secs(&self, timeout_in_seconds: f32) -> os::Result {
        unsafe { AXUIElementSetMessagingTimeout(self, timeout_in_seconds).result() }
    }

    #[inline]
    pub fn set_messaging_timeout(&self, val: std::time::Duration) -> os::Result {
        self.set_messaging_timeout_secs(val.as_secs() as _)
    }

    #[doc(alias = "AXUIElementIsAttributeSettable")]
    #[inline]
    pub fn is_settable(&self, attr: &ax::Attr) -> os::Result<bool> {
        os::result_init(|res| unsafe { AXUIElementIsAttributeSettable(self, attr, res) })
    }

    #[doc(alias = "AXUIElementPerformAction")]
    pub fn perform_action(&self, action: &ax::Action) -> os::Result {
        unsafe { AXUIElementPerformAction(self, action).result() }
    }

    #[doc(alias = "AXUIElementCopyElementAtPosition")]
    pub fn element_at_pos(&self, x: f32, y: f32) -> os::Result<arc::R<Self>> {
        unsafe { os::result_unchecked(|res| AXUIElementCopyElementAtPosition(self, x, y, res)) }
    }
}

/// Shortcuts for attributes
impl UiElement {
    pub fn ax_value(&self, attr: &ax::Attr) -> os::Result<arc::R<ax::Value>> {
        unsafe { self.attr(attr) }
    }

    pub fn role(&self) -> os::Result<arc::R<ax::Role>> {
        unsafe { self.attr(ax::attr::role()) }
    }

    pub fn role_desc(&self) -> os::Result<arc::R<cf::String>> {
        unsafe { self.attr(ax::attr::role_desc()) }
    }

    pub fn focused_app(&self) -> os::Result<arc::R<Self>> {
        unsafe { self.attr(ax::attr::focused_app()) }
    }

    pub fn focused_ui_element(&self) -> os::Result<arc::R<Self>> {
        unsafe { self.attr(ax::attr::focused_ui_element()) }
    }

    pub fn children(&self) -> os::Result<arc::R<cf::ArrayOf<Self>>> {
        unsafe { self.attr(ax::attr::children()) }
    }

    pub fn frame(&self) -> os::Result<arc::R<ax::Value>> {
        unsafe { self.attr(ax::attr::frame()) }
    }
}

define_cf_type!(
    #[doc(alias = "AXTextMarkerRef")]
    TextMarker(cf::Type)
);

impl TextMarker {
    #[doc(alias = "AXTextMarkerGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { AXTextMarkerGetTypeID() }
    }

    #[inline]
    pub fn from_slice(bytes: &[u8]) -> Option<arc::R<Self>> {
        unsafe { Self::from_bytes_in(bytes.as_ptr(), bytes.len() as isize, None) }
    }

    pub unsafe fn from_bytes_in(
        bytes: *const u8,
        length: isize,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe { AXTextMarkerCreate(allocator, bytes, length) }
    }

    #[inline]
    pub fn length(&self) -> isize {
        unsafe { AXTextMarkerGetLength(self) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.length() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.length() == 0
    }

    #[inline]
    pub fn bytes(&self) -> &[u8] {
        unsafe { std::slice::from_raw_parts(self.bytes_ptr(), self.len()) }
    }

    #[inline]
    pub fn bytes_ptr(&self) -> *const u8 {
        unsafe { AXTextMarkerGetBytePtr(self) }
    }
}

define_cf_type!(
    #[doc(alias = "AXTextMarkerRangeRef")]
    TextMarkerRange(cf::Type)
);

impl TextMarkerRange {
    #[doc(alias = "AXTextMarkerRangeGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { AXTextMarkerRangeGetTypeID() }
    }

    pub unsafe fn from_bytes_in(
        start_marker_bytes: *const u8,
        start_marker_length: isize,
        end_marker_bytes: *const u8,
        end_marker_length: isize,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe {
            AXTextMarkerRangeCreateWithBytes(
                allocator,
                start_marker_bytes,
                start_marker_length,
                end_marker_bytes,
                end_marker_length,
            )
        }
    }

    pub fn with_markers(start: &TextMarker, end: &TextMarker) -> Option<arc::R<Self>> {
        Self::with_markers_in(start, end, None)
    }

    pub fn with_markers_in(
        start: &TextMarker,
        end: &TextMarker,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe { AXTextMarkerRangeCreate(allocator, start, end) }
    }

    pub fn start(&self) -> arc::R<TextMarker> {
        unsafe { AXTextMarkerRangeCopyStartMarker(self) }
    }

    pub fn end(&self) -> arc::R<TextMarker> {
        unsafe { AXTextMarkerRangeCopyEndMarker(self) }
    }
}

define_cf_type!(
    #[doc(alias = "AXObserverRef")]
    Observer(cf::Type)
);

#[doc(alias = "AXObserverCallback")]
pub type ObserverCb = extern "C" fn(
    observer: &mut ax::Observer,
    elem: &mut ax::UiElement,
    notification: &ax::Notification,
    context: *mut std::ffi::c_void,
);

#[doc(alias = "AXObserverCallbackWithInfo")]
pub type ObserverInfoCb = extern "C" fn(
    observer: &mut ax::Observer,
    elem: &mut ax::UiElement,
    notification: &ax::Notification,
    info: &cf::DictionaryOf<cf::String, cf::Type>,
    context: *mut std::ffi::c_void,
);

impl Observer {
    #[doc(alias = "AXObserverGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { AXObserverGetTypeID() }
    }

    pub fn with_cb(pid: Pid, cb: ObserverCb) -> os::Result<arc::R<Self>> {
        unsafe { os::result_unchecked(|res| AXObserverCreate(pid, cb, res)) }
    }

    pub fn with_info_cb(pid: Pid, cb: ObserverInfoCb) -> os::Result<arc::R<Self>> {
        unsafe { os::result_unchecked(|res| AXObserverCreateWithInfoCallback(pid, cb, res)) }
    }

    pub fn add_notification(
        &mut self,
        elem: &ax::UiElement,
        notification: &ax::Notification,
        context: *mut std::ffi::c_void,
    ) -> os::Result {
        unsafe { AXObserverAddNotification(self, elem, notification, context).result() }
    }
    pub fn remove_notification(
        &mut self,
        elem: &ax::UiElement,
        notification: &ax::Notification,
    ) -> os::Result {
        unsafe { AXObserverRemoveNotification(self, elem, notification).result() }
    }
    pub fn run_loop_src(&self) -> &cf::RunLoopSrc {
        unsafe { AXObserverGetRunLoopSource(self) }
    }
}

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C-unwind" {
    fn AXIsProcessTrustedWithOptions(
        options: Option<&cf::DictionaryOf<cf::String, cf::Type>>,
    ) -> bool;

    static kAXTrustedCheckOptionPrompt: &'static cf::String;

    fn AXUIElementGetTypeID() -> cf::TypeId;

    fn AXUIElementCreateApplication(pid: Pid) -> arc::R<UiElement>;
    fn AXUIElementCreateSystemWide() -> arc::R<UiElement>;
    fn AXUIElementGetPid(elem: &UiElement, pid: *mut Pid) -> ax::Error;
    fn AXUIElementCopyElementAtPosition(
        app: &UiElement,
        x: f32,
        y: f32,
        elem: *mut Option<arc::R<UiElement>>,
    ) -> ax::Error;
    fn AXUIElementCopyAttributeNames(
        elem: &UiElement,
        names: *mut Option<arc::R<cf::ArrayOf<ax::Attr>>>,
    ) -> ax::Error;

    fn AXUIElementCopyAttributeValue(
        elem: &UiElement,
        name: &cf::String,
        value: *mut Option<arc::R<cf::Type>>,
    ) -> ax::Error;

    fn AXUIElementCopyActionNames(
        elem: &UiElement,
        names: *mut Option<arc::R<cf::ArrayOf<ax::Action>>>,
    ) -> ax::Error;

    fn AXUIElementCopyActionDescription(
        elem: &UiElement,
        action: &ax::Action,
        desc: *mut Option<arc::R<cf::String>>,
    ) -> ax::Error;

    fn AXUIElementCopyParameterizedAttributeNames(
        elem: &UiElement,
        names: *mut Option<arc::R<cf::ArrayOf<ax::ParamAttr>>>,
    ) -> ax::Error;

    fn AXUIElementPerformAction(elem: &UiElement, action: &ax::Action) -> ax::Error;

    fn AXUIElementSetMessagingTimeout(elem: &UiElement, timeout_in_seconds: f32) -> ax::Error;

    fn AXUIElementIsAttributeSettable(
        elem: &UiElement,
        attr: &ax::Attr,
        val: *mut bool,
    ) -> ax::Error;

    fn AXTextMarkerGetTypeID() -> cf::TypeId;

    fn AXTextMarkerCreate(
        allocator: Option<&cf::Allocator>,
        bytes: *const u8,
        length: isize,
    ) -> Option<arc::R<TextMarker>>;

    fn AXTextMarkerGetLength(marker: &TextMarker) -> isize;
    fn AXTextMarkerGetBytePtr(marker: &TextMarker) -> *const u8;

    fn AXTextMarkerRangeGetTypeID() -> cf::TypeId;

    fn AXTextMarkerRangeCreate(
        allocator: Option<&cf::Allocator>,
        start_marker: &TextMarker,
        end_marker: &TextMarker,
    ) -> Option<arc::R<TextMarkerRange>>;

    fn AXTextMarkerRangeCreateWithBytes(
        allocator: Option<&cf::Allocator>,
        start_marker_bytes: *const u8,
        start_marker_length: isize,
        end_marker_bytes: *const u8,
        end_marker_length: isize,
    ) -> Option<arc::R<TextMarkerRange>>;

    fn AXTextMarkerRangeCopyStartMarker(range: &TextMarkerRange) -> arc::R<TextMarker>;
    fn AXTextMarkerRangeCopyEndMarker(range: &TextMarkerRange) -> arc::R<TextMarker>;

    fn AXObserverGetTypeID() -> cf::TypeId;

    fn AXObserverCreate(
        application: Pid,
        callback: ObserverCb,
        observer: *mut Option<arc::R<Observer>>,
    ) -> ax::Error;

    fn AXObserverCreateWithInfoCallback(
        application: Pid,
        callback: ObserverInfoCb,
        observer: *mut Option<arc::R<Observer>>,
    ) -> ax::Error;

    fn AXObserverAddNotification(
        observer: &mut Observer,
        element: &ax::UiElement,
        notification: &ax::Notification,
        context: *mut std::ffi::c_void,
    ) -> ax::Error;

    fn AXObserverRemoveNotification(
        observer: &mut Observer,
        element: &ax::UiElement,
        notification: &ax::Notification,
    ) -> ax::Error;

    fn AXObserverGetRunLoopSource(observer: &Observer) -> &cf::RunLoopSrc;
}

#[cfg(test)]
mod tests {
    use crate::{ax, cf};

    #[test]
    fn ui_element_basics() {
        let sys_wide = ax::UiElement::sys_wide();
        sys_wide
            .set_messaging_timeout(std::time::Duration::from_secs(10))
            .unwrap();

        assert!(!sys_wide.is_settable(ax::attr::role()).unwrap());
        assert!(!sys_wide.is_settable(ax::attr::role_desc()).unwrap());

        let err = sys_wide.pid().unwrap_err();
        assert_eq!(err, ax::err::INVALID_UI_ELEMENT);

        let attrs = sys_wide.attrs().unwrap();
        println!("attrs {attrs:?}");

        let role = sys_wide.attr_value(ax::attr::role()).unwrap();

        let role_desc = sys_wide.role_desc().unwrap();

        println!("role {role:?} {role_desc:?}");

        let actions = sys_wide.actions().unwrap();
        assert!(actions.is_empty());

        let fake_action = ax::Action::with_string(cf::str!(c"fake"));
        let err = sys_wide.action_desc(fake_action).err().unwrap();
        assert_eq!(err, ax::err::INVALID_UI_ELEMENT);

        let err = sys_wide.param_attrs().err().unwrap();
        assert_eq!(err, ax::err::INVALID_UI_ELEMENT);
    }

    #[test]
    fn text_marker_basics() {
        let bytes = b"hello";
        let marker = ax::TextMarker::from_slice(bytes).unwrap();

        println!("marker {marker:?}");

        assert_eq!(bytes, marker.bytes());
        assert_ne!(bytes.as_ptr(), marker.bytes_ptr());

        let marker = ax::TextMarker::from_slice(&[]);

        assert!(marker.is_none());
    }

    #[test]
    fn text_marker_range_basics() {
        let hello = b"hello";
        let world = b"world";
        let a = ax::TextMarker::from_slice(hello).unwrap();
        let b = ax::TextMarker::from_slice(world).unwrap();
        let range = ax::TextMarkerRange::with_markers(&a, &b).unwrap();

        let a_from_range = range.start();
        let b_from_range = range.end();
        unsafe {
            assert_ne!(a.as_type_ptr(), a_from_range.as_type_ptr());
            assert_ne!(b.as_type_ptr(), b_from_range.as_type_ptr());
        }
    }

    #[test]
    fn observer_basics() {
        extern "C" fn callback(
            _observer: &mut ax::Observer,
            _elem: &mut ax::UiElement,
            _notification: &ax::Notification,
            _context: *mut std::ffi::c_void,
        ) {
        }
        let pid = std::process::id() as i32;

        let _app = ax::UiElement::with_app_pid(pid);

        let _observer = ax::Observer::with_cb(pid as i32, callback).unwrap();

        // observer
        //     .add_notification(
        //         &element,
        //         ax::notification::app_shown(),
        //         std::ptr::null_mut(),
        //     )
        //     .unwrap();
    }
}
