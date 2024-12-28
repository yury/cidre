#[cfg(target_os = "macos")]
mod macos {
    use std::ffi::CString;

    use cidre::{dispatch, ns, nw};

    pub fn main() {
        let url = CString::new("https://example.com").unwrap();
        let endpoint = nw::Endpoint::with_url(&url).unwrap();

        let mut conn =
            nw::Connection::with_endpoint(&endpoint, &nw::Params::default_tcp()).unwrap();

        let mut block_conn = conn.retained();
        conn.set_state_changed_handler(move |state, err| match state {
            nw::ConnectionState::Invalid => todo!(),
            nw::ConnectionState::Waiting => {
                eprintln!("waiting");
            }
            nw::ConnectionState::Preparing => {
                eprintln!("preparing");
            }
            nw::ConnectionState::Ready => {
                eprintln!("ready");
                recv_loop(&mut block_conn);
                send_request(&mut block_conn);
            }
            nw::ConnectionState::Failed => eprintln!("error: {err:?}"),
            nw::ConnectionState::Cancelled => todo!(),
        });

        let queue = dispatch::Queue::new();
        conn.start(&queue);

        ns::App::shared().run();
    }

    fn recv_loop(conn: &mut nw::Connection) {
        let mut block_conn = conn.retained();
        conn.recv(1, u32::MAX, move |content, _ctx, _is_complete, err| {
            if let Some(err) = err {
                // breaking the "loop" on error
                eprintln!("{err:?}");
                return;
            }

            // map content data regions if any
            if let Some(map) = content.map(dispatch::Data::map) {
                let res = std::str::from_utf8(&map).unwrap();
                eprintln!("{res}");
            }
            // "looping"
            recv_loop(&mut block_conn)
        });
    }

    fn send_request(conn: &mut nw::Connection) {
        let body = b"GET / HTTP/1.1
Host: example.com


";
        let data = dispatch::Data::from_static(body);
        let ctx = nw::ContentCtx::default_msg();
        let is_complete = false;

        conn.send(Some(&data), ctx, is_complete, |err| {
            eprintln!("request accepted {err:?}");
        });
    }
}

#[cfg(target_os = "macos")]
pub use macos::main;

#[cfg(not(target_os = "macos"))]
fn main() {
    println!("todo");
}
