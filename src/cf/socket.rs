use std::ffi::c_void;
use std::intrinsics::transmute;
use std::ptr::NonNull;

use crate::cf;
use crate::define_cf_type;
use crate::define_options;

define_cf_type!(Socket(cf::Type));

#[repr(transparent)]
pub struct Error(cf::Index);

impl Error {
    pub const SUCCESS: Self = Error(0);
    pub const ERROR: Self = Error(-1);
    pub const TIMEOUT: Self = Error(-2);
}

#[repr(C)]
pub struct Signature {
    pub protocol_family: i32,
    pub socket_type: i32,
    pub protocol: i32,
    pub address: cf::Retained<'static, cf::Data>,
}

define_options!(CallBackType(usize));

impl CallBackType {
    pub const NO: Self = Self(0);
    pub const READ: Self = Self(1);
    pub const ACCEPT: Self = Self(2);
    pub const DATA: Self = Self(3);
    pub const CONNECT: Self = Self(4);
    pub const WRITE: Self = Self(8);
}

define_options!(Flags(usize));

impl Flags {
    pub const AUTOMATICALLY_REENABLE_READ_CALL_BACK: Self = Self(1);
    pub const AUTOMATICALLY_REENABLE_ACCEPT_CALL_BACK: Self = Self(2);
    pub const AUTOMATICALLY_REENABLE_DATA_CALL_BACK: Self = Self(3);
    pub const AUTOMATICALLY_REENABLE_WRITE_CALL_BACK: Self = Self(8);
    pub const LEAVE_ERRORS: Self = Self(64);
    pub const CLOSE_ON_INVALIDATE: Self = Self(128);
}

pub type CallBack<T> = extern "C" fn(
    s: &Socket,
    cb_type: CallBackType,
    address: &cf::Data,
    data: *const u8,
    info: *mut T,
);

#[repr(C)]
pub struct Context<T> {
    pub version: cf::Index,
    pub info: *mut T,
    pub retain: Option<extern "C" fn(info: *const T)>,
    pub release: Option<extern "C" fn(info: *const T)>,
    pub copy_description:
        Option<extern "C" fn(info: *const T) -> Option<cf::Retained<'static, cf::String>>>,
}

pub type NativeHandle = i32;

impl Socket {
    pub unsafe fn create<'a, T>(
        allocator: Option<&cf::Allocator>,
        protocol_family: i32,
        socket_type: i32,
        protocol: i32,
        cb_types: CallBackType,
        callout: CallBack<T>,
        context: Option<NonNull<Context<c_void>>>,
    ) -> Option<cf::Retained<'a, Socket>> {
        CFSocketCreate(
            allocator,
            protocol_family,
            socket_type,
            protocol,
            cb_types,
            transmute(callout),
            transmute(context),
        )
    }
    pub unsafe fn create_with_native<'a, T>(
        allocator: Option<&cf::Allocator>,
        sock: NativeHandle,
        cb_types: CallBackType,
        callout: CallBack<T>,
        context: Option<NonNull<Context<c_void>>>,
    ) -> Option<cf::Retained<'a, Socket>> {
        CFSocketCreateWithNative(
            allocator,
            sock,
            cb_types,
            transmute(callout),
            transmute(context),
        )
    }

    pub fn native(&self) -> NativeHandle {
        unsafe { CFSocketGetNative(self) }
    }

    pub fn invalidate(&self) {
        unsafe { CFSocketInvalidate(self) }
    }

    pub fn is_valid(&self) -> bool {
        unsafe { CFSocketIsValid(self) }
    }

    pub fn create_runloop_source<'a>(
        &self,
        allocator: Option<&cf::Allocator>,
        order: cf::Index,
    ) -> Option<cf::Retained<'a, cf::RunLoopSource>> {
        unsafe { CFSocketCreateRunLoopSource(allocator, self, order) }
    }

    pub fn flags(&self) -> Flags {
        unsafe { CFSocketGetSocketFlags(self) }
    }

    pub fn set_flags(&self, flags: Flags) {
        unsafe { CFSocketSetSocketFlags(self, flags) }
    }

    pub fn enable_callbacks(&self, cb_types: CallBackType) {
        unsafe { CFSocketEnableCallBacks(self, cb_types) }
    }

    pub fn disable_callbacks(&self, cb_types: CallBackType) {
        unsafe { CFSocketDisableCallBacks(self, cb_types) }
    }

    pub fn send_data(
        &self,
        address: Option<&cf::Data>,
        data: &cf::Data,
        timeout: cf::TimeInterval,
    ) -> Error {
        unsafe { CFSocketSendData(self, address, data, timeout) }
    }
}

extern "C" {
    fn CFSocketCreate<'a>(
        allocator: Option<&cf::Allocator>,
        protocol_family: i32,
        socket_type: i32,
        protocol: i32,
        cb_types: CallBackType,
        callout: CallBack<c_void>,
        context: Option<NonNull<Context<c_void>>>,
    ) -> Option<cf::Retained<'a, Socket>>;

    fn CFSocketCreateWithNative<'a>(
        allocator: Option<&cf::Allocator>,
        sock: NativeHandle,
        cb_types: CallBackType,
        callout: CallBack<c_void>,
        context: Option<NonNull<Context<c_void>>>,
    ) -> Option<cf::Retained<'a, Socket>>;

    fn CFSocketGetNative(s: &Socket) -> NativeHandle;

    fn CFSocketInvalidate(s: &Socket);

    fn CFSocketIsValid(s: &Socket) -> bool;

    fn CFSocketCreateRunLoopSource<'a>(
        allocator: Option<&cf::Allocator>,
        s: &Socket,
        order: cf::Index,
    ) -> Option<cf::Retained<'a, cf::RunLoopSource>>;

    fn CFSocketGetSocketFlags(s: &Socket) -> Flags;
    fn CFSocketSetSocketFlags(s: &Socket, flags: Flags);

    fn CFSocketDisableCallBacks(s: &Socket, cb_types: CallBackType);
    fn CFSocketEnableCallBacks(s: &Socket, cb_types: CallBackType);

    fn CFSocketSendData(
        s: &Socket,
        address: Option<&cf::Data>,
        data: &cf::Data,
        timeout: cf::TimeInterval,
    ) -> Error;

}
