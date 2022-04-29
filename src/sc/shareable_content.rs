use std::ffi::c_void;

use crate::objc::block::CompletionHandlerAB;
use crate::{cf, cg, define_obj_type, objc::Id, sc, sys};

define_obj_type!(RunningApplication(Id));
define_obj_type!(Display(Id));

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
    fn rsel_bundleIdentifier(id: &Id) -> cf::String;
    fn rsel_applicationName(id: &Id) -> cf::String;
    fn rsel_processID(id: &Id) -> sys::Pid;
}

define_obj_type!(Window(Id));

impl Window {
    pub fn id(&self) -> cg::WindowID {
        unsafe { rsel_windowID(self) }
    }
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn rsel_windowID(id: &Id) -> cg::WindowID;
}

define_obj_type!(ShareableContent(Id));

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
}

#[link(name = "sc", kind = "static")]
extern "C" {
    fn rsel_windows(id: &Id) -> &cf::ArrayOf<Window>;
    fn rsel_displays(id: &Id) -> &cf::ArrayOf<Display>;
    fn rsel_applications(id: &Id) -> &cf::ArrayOf<RunningApplication>;
    fn cs_shareable_content_with_completion_handler(rb: *const c_void);
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration, sync::Arc};

    use crate::{
        cf, dispatch,
        sc::{
            self,
            stream::{Delegate, StreamDelegate, StreamOutput},
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
            // println!("nice {0}", self.bla);
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

    #[test]
    pub fn current_with_completion() {
        let sema = dispatch::Semaphore::new(0);
        let queue = dispatch::Queue::serial_with_autoreleasepool();
        let signal_guard = sema.signal_guard();

        let bla = Foo { bla: 0 };
        let bla2 = Foo2 { bla: 5 };

        let d = Arc::new(bla.delegate());
        let d2 = Arc::new(bla2.delegate());

        let d3 = d.clone();
        let d4 = d2.clone() ;

        sc::ShareableContent::current_with_completion(move |content, error| {
            // signal_guard.consume();

            assert!(error.is_none());
            assert!(content.is_some());

            if let Some(c) = content {
                println!("apps {:?}", c.applications().len());
                println!("windows {:?}", c.windows().len());
                println!("displays {:?}", c.displays().len());

                let ref display = c.displays()[0];

                display.as_type_ref().show();
                let windows = cf::ArrayOf::<Window>::new().unwrap();

                let filter = sc::ContentFilter::with_display_excluding_windows(&display, &windows);
                filter.as_type_ref().show();

                let mut cfg = sc::StreamConfiguration::new();
                cfg.set_width(1440 * 2);
                cfg.set_height(900 * 2);

                println!("!");

                let stream = sc::Stream::new(&filter, &cfg, Some(&d2));
                println!("!!");
                stream.as_type_ref().show();
                let mut error = None;
                queue.as_type_ref().show();
                let res = stream.add_stream_output(&d, sc::OutputType::Screen, None, &mut error);
                println!("!!!res {:?} {:?}", res, error);

                stream.start();
            }
            if let Some(e) = error {
                e.show();
            }
        });

        // sleep(Duration::from_secs(100));
        sema.wait_forever();
        // d4.obj.as_type_ref().show();
    }
}
