use std::ffi::c_void;
use std::mem::transmute;

use crate::objc::block::{Completion, CompletionHandlerAB};
use crate::objc::blocks_runtime::Block;
use crate::{cf, cg, define_obj_type, dispatch, msg_send, ns, sys};

define_obj_type!(RunningApplication(ns::Id));

impl RunningApplication {
    pub fn bundle_identifier(&self) -> cf::String {
        unsafe { rsel_bundleIdentifier(self) }
    }

    pub fn application_name(&self) -> cf::String {
        unsafe { rsel_applicationName(self) }
    }

    pub fn process_id(&self) -> sys::Pid {
        unsafe { rsel_processID(self) }
    }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn rsel_bundleIdentifier(id: &ns::Id) -> cf::String;
    fn rsel_applicationName(id: &ns::Id) -> cf::String;
    fn rsel_processID(id: &ns::Id) -> sys::Pid;
}

define_obj_type!(Display(ns::Id));

impl Display {
    pub fn display_id(&self) -> cg::DirectDisplayID {
        todo!()
    }

    pub fn width(&self) -> isize {
        msg_send!("common", self, sel_width)
    }

    pub fn height(&self) -> isize {
        msg_send!("common", self, sel_height)
    }

    pub fn frame(&self) -> cg::Rect {
        msg_send!("common", self, sel_frame)
    }
}

define_obj_type!(Window(ns::Id));

impl Window {
    pub fn id(&self) -> cg::WindowID {
        unsafe { rsel_windowID(self) }
    }

    pub fn frame(&self) -> cg::Rect {
        msg_send!("common", self, sel_frame)
    }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn rsel_windowID(id: &ns::Id) -> cg::WindowID;
}

define_obj_type!(ShareableContent(ns::Id));

impl ShareableContent {
    pub fn windows(&self) -> &cf::ArrayOf<Window> {
        unsafe { rsel_windows(self) }
    }

    pub fn displays(&self) -> &cf::ArrayOf<Display> {
        unsafe { rsel_displays(self) }
    }

    pub fn applications(&self) -> &cf::ArrayOf<RunningApplication> {
        unsafe { rsel_applications(self) }
    }

    pub fn current_with_completion<B>(block: B)
    where
        B: FnOnce(Option<&Self>, Option<&cf::Error>) + Send + 'static,
    {
        unsafe { cs_shareable_content_with_completion_handler(block.into_raw()) }
    }

    pub async fn current() -> Result<cf::Retained<Self>, cf::Retained<cf::Error>> {
        let (future, block_ptr) = Completion::result_or_error();
        unsafe { cs_shareable_content_with_completion_handler(block_ptr) }
        future.await
    }

    pub fn current_with_completion2<'ar, F>(b: &'static mut Block<F>)
    where
        F: FnOnce(Option<&'ar ShareableContent>, Option<&'ar cf::Error>) -> (),
    {
        unsafe {
            cs_shareable(b as *mut Block<F> as *mut _);
        }
    }

    // pub async fn current2() -> Result<cf::Retained<Self>, cf::Retained<cf::Error>> {

    //     let mut bl = BlockFn::new2_mut(move |content, error| {

    //     });

    //     let (future, block_ptr) = Completion::result_or_error();
    //     unsafe { cs_shareable_content_with_completion_handler(block_ptr) }
    //     future.await
    // }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn rsel_windows(id: &ns::Id) -> &cf::ArrayOf<Window>;
    fn rsel_displays(id: &ns::Id) -> &cf::ArrayOf<Display>;
    fn rsel_applications(id: &ns::Id) -> &cf::ArrayOf<RunningApplication>;
    fn cs_shareable_content_with_completion_handler(rb: *const c_void);

    fn cs_shareable(block: *mut c_void);
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use tokio::sync::Semaphore;

    use crate::{
        cf, dispatch,
        objc::blocks_runtime::{self, Block},
        sc::{
            self,
            stream::{StreamDelegate, StreamOutput},
            Window,
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
            stream: &sc::Stream,
            sample_buffer: &crate::cm::SampleBuffer,
            of_type: sc::OutputType,
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
            stream: &sc::Stream,
            error: Option<&cf::Error>,
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

    #[test]
    pub fn current_with_completion() {
        let sema = dispatch::Semaphore::new(0);

        let signal_guard = sema.signal_guard();
        let bl = blocks_runtime::new2_once(move |content, error| {
            signal_guard.consume();
            println!("nice {:?} {:?}", content, error);
        });

        dispatch::Queue::global(0).unwrap().async_once(move || {
            ShareableContent::current_with_completion2(bl.escape());
        });

        sema.wait_forever();
    }
}
