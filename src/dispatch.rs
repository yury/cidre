mod base;
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
