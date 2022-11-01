use std::ffi::c_void;

use crate::objc::block::{Completion, CompletionHandlerAB};
use crate::{cf, cg, define_obj_type, msg_send, ns, sys};

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
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn rsel_windows(id: &ns::Id) -> &cf::ArrayOf<Window>;
    fn rsel_displays(id: &ns::Id) -> &cf::ArrayOf<Display>;
    fn rsel_applications(id: &ns::Id) -> &cf::ArrayOf<RunningApplication>;
    fn cs_shareable_content_with_completion_handler(rb: *const c_void);
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        cf, dispatch,
        sc::{
            self,
            stream::{StreamDelegate, StreamOutput},
            Window,
        },
    };

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
        // let sema = dispatch::Semaphore::new(0);
        // let queue = dispatch::Queue::serial_with_autoreleasepool();
        // let signal_guard = sema.signal_guard();

        // let bla = Foo { bla: 0 };
        // let bla2 = Foo2 { bla: 5 };

        // let d = Arc::new(bla.delegate());
        // let d2 = Arc::new(bla2.delegate());

        // let d3 = d.clone();
        // let d4 = d2.clone();

        // sc::ShareableContent::current_with_completion(move |content, error| {
        //     // signal_guard.consume();

        //     assert!(error.is_none());
        //     assert!(content.is_some());

        //     if let Some(c) = content {
        //         println!("apps {:?}", c.applications().len());
        //         println!("windows {:?}", c.windows().len());
        //         println!("displays {:?}", c.displays().len());

        //         let ref display = c.displays()[0];

        //         display.as_type_ref().show();
        //         let windows = cf::ArrayOf::<Window>::new().unwrap();

        //         let filter = sc::ContentFilter::with_display_excluding_windows(&display, &windows);
        //         filter.as_type_ref().show();

        //         let mut cfg = sc::StreamConfiguration::new();
        //         cfg.set_width(display.width() as usize * 2);
        //         cfg.set_height(display.height() as usize * 2);

        //         println!("cfg size: {0}, {1}", cfg.width(), cfg.height());

        //         let stream = sc::Stream::with_delegate(&filter, &cfg, &d2);
        //         stream.as_type_ref().show();
        //         let mut error = None;
        //         queue.as_type_ref().show();
        //         let res = stream.add_stream_output(&d, sc::OutputType::Screen, Some(&queue), &mut error);

        //         stream.start_sync();
        //     }
        //     if let Some(e) = error {
        //         e.show();
        //     }
        // });

        // sema.wait_forever();
    }
}
