use cidre::{cf, cg};

extern "C" fn tap_cb(
    _proxy: *mut cg::EventTapProxy,
    event_type: cg::EventType,
    event: &mut cg::Event,
    _ctx: *mut (),
) -> Option<&cg::Event> {
    println!("{:?} {:?}", event_type, event.location());
    Some(event)
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
        cg::EventType::MOUSE_EVENTS_MASK,
        tap_cb,
        std::ptr::null_mut(),
    )
    .unwrap();

    let rl_src = tap.run_loop_src(0).unwrap();
    cf::RunLoop::main().add_src(&rl_src, Default::default());
    cf::RunLoop::run();
}
