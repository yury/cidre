use std::ffi::c_void;

use crate::{cf::{self}, cg, define_obj_type, objc::Id, sys};
use crate::objc::block::CompletionHandlerAB;

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
  use std::{thread::sleep, time::Duration};

use crate::sc;

  #[test]
  pub fn current_with_completion() {
    sc::ShareableContent::current_with_completion(|content, error| {
        assert!(error.is_none());
        assert!(content.is_some());

        if let Some(c) = content {
          println!("apps {:?}", c.applications().len());
          println!("windows {:?}", c.windows().len());
          println!("displays {:?}", c.displays().len());
        }
        if let Some(e) = error {
          e.show();
        }
    });
    sleep(Duration::from_secs(1));
  }
}