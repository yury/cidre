use cidre::{cf, cg};

extern "C" fn tap_cb(
    _proxy: *mut cg::EventTapProxy,
    event_type: cg::EventType,
    event: &mut cg::Event,
    _user_info: *mut std::ffi::c_void,
) -> Option<&cg::Event> {
    if !event.is_autorepeat() {
        println!("{event_type:?} {}", event.key_code());
    }
    return Some(event);
}

fn main() {
    if !cg::event::access::listen_preflight() {
        if !cg::event::access::listen_request() {
            eprintln!("no access for event listen");
            return;
        }
    }

    let tap = cg::EventTap::new(
        cg::EventTapLocation::Hid,
        cg::EventTapPlacement::HeadInsert,
        cg::EventTapOpts::LISTEN_ONLY,
        cg::EventType::KB_EVENTS_MASK,
        tap_cb,
        std::ptr::null_mut(),
    )
    .unwrap();

    let rl_src = tap.run_loop_src(0).unwrap();
    cf::RunLoop::main().add_src(&rl_src, Default::default());
    cf::RunLoop::run();
}
