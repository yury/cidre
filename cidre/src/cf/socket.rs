use std::ptr::NonNull;
use std::{ffi::c_void, num::NonZeroIsize};

use crate::{arc, cf, define_cf_type, define_opts};

define_cf_type!(
    #[doc(alias = "CFSocketRef")]
    Socket(cf::Type)
);

#[doc(alias = "CFSocketError")]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Error(NonZeroIsize);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(transparent)]
pub struct Status(cf::Index);

impl Status {
    #[inline]
    pub fn result(self) -> Result<(), Error> {
        if self == Self::SUCCESS {
            Ok(())
        } else {
            Err(Error(unsafe { NonZeroIsize::new_unchecked(self.0) }))
        }
    }

    #[doc(alias = "kCFSocketSuccess")]
    pub const SUCCESS: Self = Self(0);
}

impl Error {
    #[doc(alias = "kCFSocketError")]
    pub const ERROR: Self = Self(unsafe { NonZeroIsize::new_unchecked(-1) });

    #[doc(alias = "kCFSocketTimeout")]
    pub const TIMEOUT: Self = Self(unsafe { NonZeroIsize::new_unchecked(-2) });
}

#[derive(Debug)]
#[repr(C)]
pub struct Signature {
    pub protocol_family: i32,
    pub socket_type: i32,
    pub protocol: i32,
    pub address: arc::R<cf::Data>,
}

define_opts!(
    #[doc(alias = "CFSocketCallBackType")]
    pub CbType(usize)
);

impl CbType {
    pub const NO: Self = Self(0);
    pub const READ: Self = Self(1);
    pub const ACCEPT: Self = Self(2);
    pub const DATA: Self = Self(3);
    pub const CONNECT: Self = Self(4);
    pub const WRITE: Self = Self(8);
}

define_opts!(
    pub Flags(usize)
);

impl Flags {
    #[doc(alias = "kCFSocketAutomaticallyReenableReadCallBack")]
    pub const AUTOMATICALLY_REENABLE_READ_CALL_BACK: Self = Self(1);

    #[doc(alias = "kCFSocketAutomaticallyReenableAcceptCallBack")]
    pub const AUTOMATICALLY_REENABLE_ACCEPT_CALL_BACK: Self = Self(2);

    #[doc(alias = "kCFSocketAutomaticallyReenableDataCallBack")]
    pub const AUTOMATICALLY_REENABLE_DATA_CALL_BACK: Self = Self(3);

    #[doc(alias = "kCFSocketAutomaticallyReenableWriteCallBack")]
    pub const AUTOMATICALLY_REENABLE_WRITE_CALL_BACK: Self = Self(8);

    #[doc(alias = "kCFSocketLeaveErrors")]
    pub const LEAVE_ERRORS: Self = Self(64);

    #[doc(alias = "kCFSocketCloseOnInvalidate")]
    pub const CLOSE_ON_INVALIDATE: Self = Self(128);
}

#[doc(alias = "CFSocketCallBack")]
pub type Cb<T> =
    extern "C" fn(s: &Socket, cb_type: CbType, address: &cf::Data, data: *const u8, info: *mut T);

#[repr(C)]
pub struct Context<T> {
    pub version: cf::Index,
    pub info: *mut T,
    pub retain: Option<extern "C" fn(info: *const T)>,
    pub release: Option<extern "C" fn(info: *const T)>,
    pub copy_description: Option<extern "C" fn(info: *const T) -> Option<arc::R<cf::String>>>,
}

#[doc(alias = "CFSocketNativeHandle")]
pub type NativeHandle = i32;

impl Socket {
    pub unsafe fn create_in<T>(
        protocol_family: i32,
        socket_type: i32,
        protocol: i32,
        cb_types: CbType,
        callout: Cb<T>,
        context: Option<NonNull<Context<c_void>>>,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Socket>> {
        CFSocketCreate(
            allocator,
            protocol_family,
            socket_type,
            protocol,
            cb_types,
            std::mem::transmute(callout),
            context,
        )
    }
    pub unsafe fn create_with_native_in<T>(
        sock: NativeHandle,
        cb_types: CbType,
        callout: Cb<T>,
        context: Option<NonNull<Context<c_void>>>,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<Socket>> {
        CFSocketCreateWithNative(
            allocator,
            sock,
            cb_types,
            std::mem::transmute(callout),
            context,
        )
    }

    pub unsafe fn create_with_native<T>(
        sock: NativeHandle,
        cb_types: CbType,
        callout: Cb<T>,
        context: Option<NonNull<Context<c_void>>>,
    ) -> Option<arc::R<Socket>> {
        Self::create_with_native_in(sock, cb_types, callout, context, None)
    }

    #[inline]
    pub fn native(&self) -> NativeHandle {
        unsafe { CFSocketGetNative(self) }
    }

    #[inline]
    pub fn invalidate(&self) {
        unsafe { CFSocketInvalidate(self) }
    }

    #[inline]
    pub fn is_valid(&self) -> bool {
        unsafe { CFSocketIsValid(self) }
    }

    pub fn run_loop_src_in(
        &self,
        order: cf::Index,
        allocator: Option<&cf::Allocator>,
    ) -> Option<arc::R<cf::RunLoopSrc>> {
        unsafe { CFSocketCreateRunLoopSource(allocator, self, order) }
    }

    pub fn run_loop_src(&self, order: cf::Index) -> Option<arc::R<cf::RunLoopSrc>> {
        self.run_loop_src_in(order, None)
    }

    #[inline]
    pub fn flags(&self) -> Flags {
        unsafe { CFSocketGetSocketFlags(self) }
    }

    #[inline]
    pub fn set_flags(&self, flags: Flags) {
        unsafe { CFSocketSetSocketFlags(self, flags) }
    }

    #[inline]
    pub fn enable_callbacks(&self, cb_types: CbType) {
        unsafe { CFSocketEnableCallBacks(self, cb_types) }
    }

    #[inline]
    pub fn disable_callbacks(&self, cb_types: CbType) {
        unsafe { CFSocketDisableCallBacks(self, cb_types) }
    }

    #[inline]
    pub fn send_data(
        &self,
        address: Option<&cf::Data>,
        data: &cf::Data,
        timeout: cf::TimeInterval,
    ) -> Result<(), Error> {
        unsafe { CFSocketSendData(self, address, data, timeout).result() }
    }
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C-unwind" {
    fn CFSocketCreate(
        allocator: Option<&cf::Allocator>,
        protocol_family: i32,
        socket_type: i32,
        protocol: i32,
        cb_types: CbType,
        callout: Cb<c_void>,
        context: Option<NonNull<Context<c_void>>>,
    ) -> Option<arc::R<Socket>>;

    fn CFSocketCreateWithNative(
        allocator: Option<&cf::Allocator>,
        sock: NativeHandle,
        cb_types: CbType,
        callout: Cb<c_void>,
        context: Option<NonNull<Context<c_void>>>,
    ) -> Option<arc::R<Socket>>;

    fn CFSocketGetNative(s: &Socket) -> NativeHandle;

    fn CFSocketInvalidate(s: &Socket);

    fn CFSocketIsValid(s: &Socket) -> bool;

    fn CFSocketCreateRunLoopSource(
        allocator: Option<&cf::Allocator>,
        s: &Socket,
        order: cf::Index,
    ) -> Option<arc::R<cf::RunLoopSrc>>;

    fn CFSocketGetSocketFlags(s: &Socket) -> Flags;
    fn CFSocketSetSocketFlags(s: &Socket, flags: Flags);

    fn CFSocketDisableCallBacks(s: &Socket, cb_types: CbType);
    fn CFSocketEnableCallBacks(s: &Socket, cb_types: CbType);

    fn CFSocketSendData(
        s: &Socket,
        address: Option<&cf::Data>,
        data: &cf::Data,
        timeout: cf::TimeInterval,
    ) -> Status;

}
