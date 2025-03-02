#[cfg(target_os = "macos")]
mod macos {
    use cidre::{core_audio as ca, io, ns, os};

    extern "C-unwind" fn listener(
        _obj_id: ca::Obj,
        number_addresses: u32,
        addresses: *const ca::PropAddr,
        _client_data: *mut (),
    ) -> os::Status {
        let addresses = unsafe { std::slice::from_raw_parts(addresses, number_addresses as usize) };

        for addr in addresses {
            match addr.selector {
                ca::PropSelector::HW_DEFAULT_INPUT_DEVICE => {
                    println!("default input device changed")
                }
                ca::PropSelector::HW_DEFAULT_OUTPUT_DEVICE => {
                    println!("default output device changed");
                    let device = ca::System::default_output_device().unwrap();
                    let streams = device.streams().unwrap();
                    let headphones = streams
                        .iter()
                        .find(|s| {
                            let term_type = s.terminal_type().unwrap();
                            term_type.0 == io::kit::audio::types::output_term::HEADPHONES
                                || term_type == ca::StreamTerminalType::HEADPHONES
                        })
                        .is_some();
                    println!("headphones connected {headphones}");
                }
                _ => panic!("unregistered selector"),
            }
        }
        os::Status::NO_ERR
    }

    pub(crate) fn main() {
        let selectors = [
            ca::PropSelector::HW_DEFAULT_INPUT_DEVICE,
            ca::PropSelector::HW_DEFAULT_OUTPUT_DEVICE,
        ];

        for s in selectors {
            ca::System::OBJ
                .add_prop_listener(&s.global_addr(), listener, std::ptr::null_mut())
                .unwrap();
        }

        ns::RunLoop::main().run();
    }
}

#[cfg(target_os = "ios")]
mod ios {
    use cidre::{av, ns, objc::Obj};

    fn has_headphones(route_desc: &av::AudioSessionRouteDesc) -> bool {
        let outputs = route_desc.outputs();
        let port = outputs
            .iter()
            .find(|port| &port.port_type() == av::AudioSessionPort::headphones());
        port.is_some()
    }

    pub(crate) fn main() {
        let mut nc = ns::NotificationCenter::default();
        let _guard = nc.add_observer_guard(
            av::AudioSession::route_change_notification(),
            None,
            None,
            |n| {
                let Some(user_info) = n.user_info() else {
                    return;
                };
                let Some(reason) = user_info.get(av::audio_session_keys::route_change_reason())
                else {
                    return;
                };

                // reason value stored as ns::Number
                let Some(reason) = reason.try_cast(ns::Number::cls()) else {
                    return;
                };

                let reason: av::AudioSessionRouteChangeReason = reason.as_u32().into();

                match reason {
                    av::AudioSessionRouteChangeReason::NewDeviceAvailable => {
                        println!("new device available");
                        let route_desc = av::AudioSession::shared().current_route();
                        let headphones = has_headphones(&route_desc);
                        println!("headphones connected {headphones}");
                    }
                    av::AudioSessionRouteChangeReason::OldDeviceUnavailable => {
                        println!("old device unavailable");
                        let Some(route_desc) =
                            user_info.get(av::audio_session_keys::route_change_previuous_route())
                        else {
                            return;
                        };

                        let Some(route_desc) =
                            route_desc.try_cast(av::AudioSessionRouteDesc::cls())
                        else {
                            return;
                        };
                        let headphones = has_headphones(&route_desc);
                        println!("headphones removed {headphones}");
                    }
                    x => println!("other reason {x:?}"),
                }
            },
        );
        println!("activating session to get notifications");
        av::AudioSession::shared().set_active(true).unwrap();
        println!("application is started");
        ns::RunLoop::main().run();
    }
}

#[cfg(not(target_os = "macos"))]
use ios::main;
#[cfg(target_os = "macos")]
use macos::main;
