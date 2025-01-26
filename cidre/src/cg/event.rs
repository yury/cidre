use crate::{arc, cf, cg, define_cf_type};

#[cfg(feature = "ns")]
use crate::ns;

impl cg::Event {
    pub fn get_type() -> cf::TypeId {
        unsafe { CGEventGetTypeID() }
    }

    #[cfg(feature = "ns")]
    pub fn as_ns(&self) -> &ns::Id {
        unsafe { std::mem::transmute(self) }
    }

    #[doc(alias = "CGEventCreate")]
    pub fn with_src(src: Option<&cg::EventSrc>) -> Option<arc::R<Self>> {
        unsafe { CGEventCreate(src) }
    }

    #[doc(alias = "CGEventCreateFromData")]
    pub fn with_data_in(
        data: &cf::Data,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Self>> {
        unsafe { CGEventCreateFromData(allocator, data) }
    }

    #[doc(alias = "CGEventCreateFromData")]
    pub fn with_data(data: &cf::Data) -> Option<arc::R<Self>> {
        unsafe { CGEventCreateFromData(None, data) }
    }

    #[doc(alias = "CGEventCreateData")]
    pub fn data_in(&self, allocator: Option<&cf::Allocator>) -> Option<arc::R<cf::Data>> {
        unsafe { CGEventCreateData(allocator, self) }
    }

    #[doc(alias = "CGEventCreateData")]
    pub fn data(&self) -> Option<arc::R<cf::Data>> {
        unsafe { CGEventCreateData(None, self) }
    }

    #[doc(alias = "CGEventCreateMouseEvent")]
    pub fn mouse(
        src: Option<&cg::EventSrc>,
        mouse_type: cg::EventType,
        mouse_cursor_pos: cg::Point,
        mouse_button: cg::MouseButton,
    ) -> Option<arc::R<Self>> {
        unsafe { CGEventCreateMouseEvent(src, mouse_type, mouse_cursor_pos, mouse_button) }
    }

    #[doc(alias = "CGEventCreateKeyboardEvent")]
    pub fn keyboard(
        src: Option<&cg::EventSrc>,
        virtual_key: cg::KeyCode,
        key_down: bool,
    ) -> Option<arc::R<Self>> {
        unsafe { CGEventCreateKeyboardEvent(src, virtual_key, key_down) }
    }

    #[doc(alias = "CGEventCreateScrollWheelEvent2")]
    pub fn wheel(
        src: Option<&cg::EventSrc>,
        units: cg::ScrollEventUnit,
        wheel_count: u32,
        wheel_1: u32,
        wheel_2: u32,
        wheel_3: u32,
    ) -> Option<arc::R<Self>> {
        unsafe {
            CGEventCreateScrollWheelEvent2(src, units, wheel_count, wheel_1, wheel_2, wheel_3)
        }
    }

    pub fn wheel_1(
        src: Option<&cg::EventSrc>,
        units: cg::ScrollEventUnit,
        wheel: u32,
    ) -> Option<arc::R<Self>> {
        unsafe { CGEventCreateScrollWheelEvent2(src, units, 1, wheel, 0, 0) }
    }

    pub fn wheel_2(
        src: Option<&cg::EventSrc>,
        units: cg::ScrollEventUnit,
        wheel_1: u32,
        wheel_2: u32,
    ) -> Option<arc::R<Self>> {
        unsafe { CGEventCreateScrollWheelEvent2(src, units, 2, wheel_1, wheel_2, 0) }
    }

    pub fn wheel_3(
        src: Option<&cg::EventSrc>,
        units: cg::ScrollEventUnit,
        wheel_1: u32,
        wheel_2: u32,
        wheel_3: u32,
    ) -> Option<arc::R<Self>> {
        unsafe { CGEventCreateScrollWheelEvent2(src, units, 3, wheel_1, wheel_2, wheel_3) }
    }

    #[doc(alias = "CGEventCreateCopy")]
    pub fn copy(&self) -> arc::R<Self> {
        unsafe { CGEventCreateCopy(self) }
    }

    #[doc(alias = "CGEventSetSource")]
    pub fn set_src(&mut self, val: Option<&cg::EventSrc>) {
        unsafe {
            CGEventSetSource(self, val);
        }
    }

    #[doc(alias = "CGEventGetType")]
    pub fn type_(&self) -> cg::EventType {
        unsafe { CGEventGetType(self) }
    }

    #[doc(alias = "CGEventSetType")]
    pub fn set_type(&mut self, val: cg::EventType) {
        unsafe { CGEventSetType(self, val) };
    }

    #[doc(alias = "CGEventGetTimestamp")]
    pub fn timestamp(&self) -> cf::TimeInterval {
        unsafe { CGEventGetTimestamp(self) }
    }

    #[doc(alias = "CGEventSetTimestamp")]
    pub fn set_timestamp(&mut self, val: cf::TimeInterval) {
        unsafe {
            CGEventSetTimestamp(self, val);
        }
    }

    #[doc(alias = "CGEventGetLocation")]
    pub fn location(&self) -> cg::Point {
        unsafe { CGEventGetLocation(self) }
    }

    #[doc(alias = "CGEventGetUnflippedLocation")]
    pub fn unflipped_location(&self) -> cg::Point {
        unsafe { CGEventGetUnflippedLocation(self) }
    }

    #[doc(alias = "CGEventSetLocation")]
    pub fn set_location(&mut self, val: cg::Point) {
        unsafe {
            CGEventSetLocation(self, val);
        }
    }

    #[doc(alias = "CGEventGetFlags")]
    pub fn flags(&self) -> cg::EventFlags {
        unsafe { CGEventGetFlags(self) }
    }

    #[doc(alias = "CGEventSetFlags")]
    pub fn set_flags(&mut self, val: cg::EventFlags) {
        unsafe { CGEventSetFlags(self, val) }
    }
}

impl cg::EventSrc {
    #[doc(alias = "CGEventCreateSourceFromEvent")]
    pub fn with_event(event: &cg::Event) -> Option<arc::R<Self>> {
        unsafe { CGEventCreateSourceFromEvent(event) }
    }
}

#[cfg(feature = "ns")]
impl AsRef<ns::Id> for cg::Event {
    fn as_ref(&self) -> &ns::Id {
        self.as_ns()
    }
}

define_cf_type!(EventTap(cf::MachPort));

impl EventTap {
    #[doc(alias = "CGEventTapCreate")]
    pub fn new<U>(
        tap: cg::EventTapLocation,
        place: cg::EventTapPlacement,
        events_of_interest: cg::EventMask,
        callback: cg::EventTapCb<U>,
        user_info: *mut U,
    ) -> Option<arc::R<EventTap>> {
        unsafe {
            CGEventTapCreate(
                tap,
                place,
                events_of_interest,
                std::mem::transmute(callback),
                std::mem::transmute(user_info),
            )
        }
    }

    #[doc(alias = "CGEventTapIsEnabled")]
    pub fn is_enabled(&self) -> bool {
        unsafe { CGEventTapIsEnabled(self) }
    }

    #[doc(alias = "CGEventTapEnable")]
    pub fn set_enabled(&mut self, val: bool) {
        unsafe {
            CGEventTapEnable(self, val);
        }
    }
}

pub mod access {
    #[doc(alias = "CGPreflightListenEventAccess")]
    pub fn listen_preflight() -> bool {
        unsafe { CGPreflightListenEventAccess() }
    }

    #[doc(alias = "CGRequestListenEventAccess")]
    pub fn listen_request() -> bool {
        unsafe { CGRequestListenEventAccess() }
    }

    #[doc(alias = "CGPreflightPostEventAccess")]
    pub fn post_preflight() -> bool {
        unsafe { CGPreflightPostEventAccess() }
    }

    #[doc(alias = "CGRequestPostEventAccess")]
    pub fn post_request() -> bool {
        unsafe { CGRequestPostEventAccess() }
    }

    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C-unwind" {
        fn CGPreflightListenEventAccess() -> bool;
        fn CGRequestListenEventAccess() -> bool;
        fn CGPreflightPostEventAccess() -> bool;
        fn CGRequestPostEventAccess() -> bool;
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C-unwind" {
    fn CGEventGetTypeID() -> cf::TypeId;
    fn CGEventCreate(source: Option<&cg::EventSrc>) -> Option<arc::R<cg::Event>>;
    fn CGEventCreateData(
        allocator: Option<&cf::Allocator>,
        event: &cg::Event,
    ) -> Option<arc::R<cf::Data>>;
    fn CGEventCreateFromData(
        allocator: Option<&cf::Allocator>,
        data: &cf::Data,
    ) -> Option<arc::R<cg::Event>>;

    fn CGEventCreateMouseEvent(
        source: Option<&cg::EventSrc>,
        mouse_type: cg::EventType,
        mouse_cursor_pos: cg::Point,
        mouse_button: cg::MouseButton,
    ) -> Option<arc::R<cg::Event>>;

    fn CGEventCreateKeyboardEvent(
        source: Option<&cg::EventSrc>,
        virtual_key: cg::KeyCode,
        key_down: bool,
    ) -> Option<arc::R<cg::Event>>;

    fn CGEventCreateScrollWheelEvent2(
        source: Option<&cg::EventSrc>,
        units: cg::ScrollEventUnit,
        wheel_count: u32,
        wheel_1: u32,
        wheel_2: u32,
        wheel_3: u32,
    ) -> Option<arc::R<cg::Event>>;

    fn CGEventCreateCopy(event: &cg::Event) -> arc::R<cg::Event>;

    fn CGEventCreateSourceFromEvent(event: &cg::Event) -> Option<arc::R<cg::EventSrc>>;

    fn CGEventSetSource(event: *mut cg::Event, source: Option<&cg::EventSrc>);
    fn CGEventGetType(event: *const cg::Event) -> cg::EventType;
    fn CGEventSetType(event: *mut cg::Event, val: cg::EventType);
    fn CGEventGetTimestamp(event: *const cg::Event) -> cf::TimeInterval;
    fn CGEventSetTimestamp(event: *mut cg::Event, val: cf::TimeInterval);
    fn CGEventGetLocation(event: *const cg::Event) -> cg::Point;
    fn CGEventGetUnflippedLocation(event: *const cg::Event) -> cg::Point;
    fn CGEventSetLocation(event: *mut cg::Event, val: cg::Point);

    fn CGEventGetFlags(event: *const cg::Event) -> cg::EventFlags;
    fn CGEventSetFlags(event: *mut cg::Event, val: cg::EventFlags);

    fn CGEventTapCreate(
        tap: cg::EventTapLocation,
        place: cg::EventTapPlacement,
        events_of_interest: cg::EventMask,
        callback: cg::EventTapCb,
        user_info: *mut std::ffi::c_void,
    ) -> Option<arc::R<EventTap>>;

    fn CGEventTapEnable(tap: &mut EventTap, val: bool);
    fn CGEventTapIsEnabled(tap: &EventTap) -> bool;
}
