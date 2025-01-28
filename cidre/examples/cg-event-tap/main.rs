use std::io::Write;

use cidre::{cf, cg};

fn display(buf: *mut String, event_type: cg::EventType, event: &cg::Event) {
    let buf = unsafe { buf.as_mut().unwrap() };
    buf.clear();

    // Clean line
    buf.push_str("\r                                       ");

    match event_type {
        cg::EventType::KEY_DOWN => buf.push_str("\r▼"),
        cg::EventType::KEY_UP => buf.push_str("\r▲"),
        _ => buf.push_str("\r•"),
    }

    let flags = event.flags();

    // Add modifiers

    if flags.contains(cg::EventFlags::CMD) {
        buf.push_str("⌘");
    }
    if flags.contains(cg::EventFlags::SHIFT) {
        buf.push_str("⇧");
    }
    if flags.contains(cg::EventFlags::CTRL) {
        buf.push_str("⌃");
    }
    if flags.contains(cg::EventFlags::ALT) {
        buf.push_str("⌥");
    }

    let key_code = event.kb_key_code();
    let key_name = match key_code {
        0x00 => "a",
        0x01 => "s",
        0x02 => "d",
        0x03 => "f",
        0x04 => "h",
        0x05 => "g",
        0x06 => "z",
        0x07 => "x",
        0x08 => "c",
        0x09 => "v",
        0x0B => "b",
        0x0C => "q",
        0x0D => "w",
        0x0E => "e",
        0x0F => "r",
        0x10 => "y",
        0x11 => "t",
        0x12 => "1",
        0x13 => "2",
        0x14 => "3",
        0x15 => "4",
        0x16 => "6",
        0x17 => "5",
        0x18 => "=",
        0x19 => "9",
        0x1A => "7",
        0x1B => "-",
        0x1C => "8",
        0x1D => "0",
        0x1E => "]",
        0x1F => "o",
        0x20 => "u",
        0x21 => "[",
        0x22 => "i",
        0x23 => "p",
        0x24 => "↩",
        0x25 => "l",
        0x26 => "j",
        0x27 => "'",
        0x28 => "k",
        0x29 => ";",
        0x2A => "\\",
        0x2B => ",",
        0x2C => "/",
        0x2D => "n",
        0x2E => "m",
        0x2F => ".",
        0x30 => "⇥",
        0x31 => "space",
        0x32 => "`",
        0x33 => "⌫",
        0x35 => "esc",
        0x39 => "⇪",
        0x7B => "←",
        0x7C => "→",
        0x7D => "↓",
        0x7E => "↑",
        0x37 => "", //"⌘",
        0x38 => "", //"⇧",
        0x3A => "", //"⌥",
        0x3B => "", //"⌃",
        _ => "?",
    };

    buf.push_str(key_name);

    buf.push_str("\n");

    std::io::stderr().write(buf.as_bytes()).ok();
}

extern "C" fn tap_cb(
    _proxy: *mut cg::EventTapProxy,
    event_type: cg::EventType,
    event: &mut cg::Event,
    buf: *mut String,
) -> Option<&cg::Event> {
    if !event.is_kb_autorepeat() {
        display(buf, event_type, event);
    }
    Some(event)
}

fn main() {
    if !cg::event::access::listen_preflight() {
        if !cg::event::access::listen_request() {
            eprintln!("no access for event listen");
            return;
        }
    }

    let mut buf = String::new();

    let tap = cg::EventTap::new(
        cg::EventTapLocation::Hid,
        cg::EventTapPlacement::HeadInsert,
        cg::EventTapOpts::LISTEN_ONLY,
        cg::EventType::KB_EVENTS_MASK,
        tap_cb,
        &mut buf,
    )
    .unwrap();

    let rl_src = tap.run_loop_src(0).unwrap();
    cf::RunLoop::main().add_src(&rl_src, Default::default());
    cf::RunLoop::run();
}
