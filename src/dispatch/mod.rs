pub mod base;
pub use base::Function;

pub mod object;
pub use object::Object as DispatchObject;

pub mod queue;
pub use queue::Concurent as ConcurentQueue;
pub use queue::Global as GlobalQueue;
pub use queue::Main as MainQueue;
pub use queue::Queue;

pub fn main() {
  let id = std::thread::current().id();
  println!("{:?}", id);
    unsafe { dispatch_main() }
}

#[cfg_attr(
    any(target_os = "macos", target_os = "ios"),
    link(name = "System", kind = "dylib")
)]
#[cfg_attr(
    not(any(target_os = "macos", target_os = "ios")),
    link(name = "dispatch", kind = "dylib")
)]
extern "C" {
    fn dispatch_main();
}
