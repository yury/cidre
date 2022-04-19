use std::ffi::c_void;

use crate::objc::block::CompletionHandlerAB;
use crate::{
    cf::{self},
    cg, define_obj_type,
    objc::Id,
    sys,
};

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
    use crate::{sc::{self, Window}, cf, dispatch};

    #[test]
    pub fn current_with_completion() {
        let sema = dispatch::Semaphore::new(0);
        let signal_guard = sema.signal_guard();

        sc::ShareableContent::current_with_completion(move |content, error| {
            signal_guard.consume();

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
            }
            if let Some(e) = error {
                e.show();
            }
        });
        
        sema.wait_forever();
    }
}
