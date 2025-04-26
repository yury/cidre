use cidre::{define_obj_type, ns, objc, sc};

#[repr(C)]
struct ObserverInner {}

define_obj_type!(
    Observer + sc::ContentSharingPickerObserverImpl,
    ObserverInner,
    PICKER_OBSERVER
);

impl sc::ContentSharingPickerObserver for Observer {}

#[objc::add_methods]
impl sc::ContentSharingPickerObserverImpl for Observer {
    extern "C" fn impl_picker_did_cancel_for_stream(
        &mut self,
        _cmd: Option<&objc::Sel>,
        _picker: &mut sc::ContentSharingPicker,
        _stream: Option<&sc::Stream>,
    ) {
        println!("Cancel")
    }

    extern "C" fn impl_picker_did_update_with_filter_for_stream(
        &mut self,
        _cmd: Option<&objc::Sel>,
        _picker: &mut sc::ContentSharingPicker,
        filter: &sc::ContentFilter,
        _stream: Option<&sc::Stream>,
    ) {
        println!("Picker did update with filter: {:?}", filter);
    }

    extern "C" fn impl_picker_start_did_fail_with_err(
        &mut self,
        _cmd: Option<&objc::Sel>,
        err: &ns::Error,
    ) {
        println!("Error {:?}", err);
    }
}

fn main() {
    let observer = Observer::with(ObserverInner {});
    let mut picker = sc::ContentSharingPicker::shared();
    picker.add_observer(observer.as_ref());
    picker.set_active(true);
    picker.present();
    ns::App::shared().run();
}
