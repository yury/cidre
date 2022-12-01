mod base;
use std::ffi::c_void;

pub use base::Function;

mod object;
pub use object::Object;

mod group;
pub use group::Group;

mod time;
pub use time::Time;

pub mod queue;
pub use queue::Attr;
pub use queue::AutoreleaseFrequency;
pub use queue::Concurent as ConcurentQueue;
pub use queue::Global as GlobalQueue;
pub use queue::Main as MainQueue;
pub use queue::Priority as QueuePriority;
pub use queue::QOSClass;
pub use queue::Queue;

mod semaphore;
pub use semaphore::Semaphore;

pub mod source;
pub use source::MachRecvFlags as SourceMachRecvFlags;
pub use source::MachSendFlags as SourceMachSendFlags;
pub use source::MemoryPressureFlags as SourceMemoryPressureFlags;
pub use source::ProcFlags as SourceProcFlags;
pub use source::Source;
pub use source::TimerFlags as SourceTimerFlags;
pub use source::TimerSource;
pub use source::Type as SourceType;
pub use source::TypeDataAdd as SourceDataAdd;

use crate::objc::blocks::bl;
use crate::objc::blocks::BlMut;
use crate::objc::blocks::Block as ObjcBlock;

pub trait Block<F> {
    unsafe fn ptr(&mut self) -> *mut c_void;
}

impl<F> Block<F> for ObjcBlock<F>
where
    F: FnOnce() + 'static,
{
    #[inline]
    unsafe fn ptr(&mut self) -> *mut c_void {
        self.as_ptr()
    }
}

impl<F> Block<F> for BlMut<F>
where
    F: FnOnce() + 'static,
{
    #[inline]
    unsafe fn ptr(&mut self) -> *mut c_void {
        self.as_ptr()
    }
}

impl Block<extern "C" fn(*const c_void)> for ObjcBlock<extern "C" fn(*const c_void)> {
    #[inline]
    unsafe fn ptr(&mut self) -> *mut c_void {
        self.as_ptr()
    }
}

impl Block<extern "C" fn(*const c_void)> for bl<extern "C" fn(*const c_void)> {
    #[inline]
    unsafe fn ptr(&mut self) -> *mut c_void {
        self.as_ptr()
    }
}

/// This function "parks" the main thread and waits for blocks to be submitted to the main queue.
/// Applications that call UIApplicationMain (iOS), NSApplicationMain (macOS), or CFRunLoopRun
/// on the main thread must not call `dispatch::main()`.
#[inline]
pub fn main() {
    unsafe { dispatch_main() }
}

#[link(name = "System", kind = "dylib")]
extern "C" {
    fn dispatch_main();
}
