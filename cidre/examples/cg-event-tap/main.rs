use cidre::{cf, cg, ns};

extern "C" fn tap(
    _proxy: *mut cg::EventTapProxy,
    event_type: cg::EventType,
    event: &mut cg::Event,
    _user_info: *mut std::ffi::c_void,
) -> Option<&cg::Event> {
    println!("{event_type:?} {}", event.key_code());
    // event.show();
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
        cg::EventTapLocation::Session,
        cg::EventTapPlacement::TailAppend,
        cg::EventTapOpts::LISTEN_ONLY,
        cg::EventType::KB_EVENTS_MASK,
        tap,
        std::ptr::null_mut(),
    )
    .unwrap();
    let run_loop_src = tap.run_loop_src(0).unwrap();
    cf::RunLoop::main().add_src(&run_loop_src, cf::RunLoopMode::default());

    ns::RunLoop::main().run();
}
