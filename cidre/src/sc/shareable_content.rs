use std::ffi::c_void;

use crate::{
    arc, blocks, cg, define_obj_type, ns,
    objc::{msg_send, Obj},
    sys,
};

define_obj_type!(RunningApplication(ns::Id));

impl RunningApplication {
    pub fn bundle_identifier(&self) -> ns::String {
        unsafe { rsel_bundleIdentifier(self) }
    }

    pub fn application_name(&self) -> ns::String {
        unsafe { rsel_applicationName(self) }
    }

    pub fn process_id(&self) -> sys::Pid {
        unsafe { rsel_processID(self) }
    }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn rsel_bundleIdentifier(id: &ns::Id) -> ns::String;
    fn rsel_applicationName(id: &ns::Id) -> ns::String;
    fn rsel_processID(id: &ns::Id) -> sys::Pid;
}

define_obj_type!(Display(ns::Id));

impl Display {
    pub fn display_id(&self) -> cg::DirectDisplayID {
        todo!()
    }

    pub fn width(&self) -> isize {
        unsafe { self.call0(msg_send::width) }
    }

    pub fn height(&self) -> isize {
        unsafe { self.call0(msg_send::height) }
    }

    pub fn frame(&self) -> cg::Rect {
        unsafe { self.call0(msg_send::frame) }
    }
}

define_obj_type!(Window(ns::Id));

impl Window {
    pub fn id(&self) -> cg::WindowID {
        unsafe { rsel_windowID(self) }
    }

    pub fn frame(&self) -> cg::Rect {
        unsafe { self.call0(msg_send::frame) }
    }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn rsel_windowID(id: &ns::Id) -> cg::WindowID;
}

define_obj_type!(ShareableContent(ns::Id));

impl ShareableContent {
    pub fn windows(&self) -> &ns::Array<Window> {
        unsafe { rsel_windows(self) }
    }

    pub fn displays(&self) -> &ns::Array<Display> {
        unsafe { rsel_displays(self) }
    }

    pub fn applications(&self) -> &ns::Array<RunningApplication> {
        unsafe { rsel_applications(self) }
    }

    pub fn current_with_completion<'ar, F>(b: &'static mut blocks::Block<F>)
    where
        F: FnOnce(Option<&'ar ShareableContent>, Option<&'ar ns::Error>),
    {
        unsafe {
            cs_shareable(b.as_ptr());
        }
    }

    pub async fn current() -> Result<arc::R<Self>, arc::R<ns::Error>> {
        let (future, block) = blocks::result();

        Self::current_with_completion(block.escape());

        future.await
    }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn rsel_windows(id: &ns::Id) -> &ns::Array<Window>;
    fn rsel_displays(id: &ns::Id) -> &ns::Array<Display>;
    fn rsel_applications(id: &ns::Id) -> &ns::Array<RunningApplication>;

    fn cs_shareable(block: *mut c_void);
}

#[cfg(test)]
mod tests {

    use crate::{
        blocks, dispatch, ns,
        sc::{
            self,
            stream::{StreamDelegate, StreamOutput},
        },
    };

    use super::ShareableContent;

    #[repr(C)]
    struct Foo {
        bla: u32,
    }

    impl StreamOutput for Foo {
        extern "C" fn stream_did_output_sample_buffer_of_type(
            &mut self,
            _stream: &sc::Stream,
            _sample_buffer: &mut crate::cm::SampleBuffer,
            _of_type: sc::OutputType,
        ) {
            self.bla += 1;
            println!("nice {0}", self.bla);
        }
    }

    #[repr(C)]
    struct Foo2 {
        bla: u32,
    }

    impl StreamDelegate for Foo2 {
        extern "C" fn stream_did_stop_with_error(
            &mut self,
            _stream: &sc::Stream,
            _error: Option<&ns::Error>,
        ) {
            println!("!!!!")
        }
    }
    #[tokio::test]
    pub async fn current() {
        let f = sc::ShareableContent::current().await.expect("result");
        assert!(!f.windows().is_empty());
        println!(
            "current retain count {:?} {:?}",
            f.as_type_ref().retain_count(),
            f.windows().len()
        );
    }

    #[tokio::test]
    pub async fn current2() {
        let f = sc::ShareableContent::current().await.expect("result");
        assert!(!f.windows().is_empty());
        println!(
            "current retain count {:?} {:?}",
            f.as_type_ref().retain_count(),
            f.windows().len()
        );
    }

    #[test]
    pub fn current_with_completion() {
        let sema = dispatch::Semaphore::new(0);

        let signal_guard = sema.signal_guard();
        let bl = blocks::once2(move |content, error| {
            signal_guard.consume();
            println!("nice {:?} {:?}", content, error);
        });

        dispatch::Queue::global(0).unwrap().async_once(move || {
            ShareableContent::current_with_completion(bl.escape());
        });

        sema.wait_forever();
    }
}