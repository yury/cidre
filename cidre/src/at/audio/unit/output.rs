use crate::{
    at::{
        au::{self, Scope, Unit, UnitRef},
        audio::{
            self,
            component::{InitializedState, State, UninitializedState},
        },
    },
    os,
};

#[cfg(all(target_os = "macos", feature = "core_audio"))]
use crate::core_audio::Device;

#[derive(Debug)]
pub struct Output<S>(UnitRef<S>)
where
    S: State<Unit>;

impl<S> Output<S>
where
    S: State<Unit>,
{
    #[inline]
    pub fn unit(&self) -> &Unit {
        self.0.unit()
    }

    #[inline]
    pub fn unit_mut(&mut self) -> &mut Unit {
        self.0.unit_mut()
    }

    #[inline]
    pub fn last_render_err(&self) -> os::Result<os::Status> {
        self.unit().last_render_err()
    }

    #[inline]
    pub fn set_input_cb<const N: usize, T>(
        &mut self,
        cb: au::RenderCb<N, T>,
        ref_con: *const T,
    ) -> os::Result {
        self.0
            .unit_mut()
            .output_set_input_cb(au::Scope::GLOBAL, 1, cb, ref_con)
    }

    #[inline]
    pub fn set_output_cb<const N: usize, T>(
        &mut self,
        cb: au::RenderCb<N, T>,
        ref_con: *const T,
    ) -> os::Result {
        self.0
            .unit_mut()
            .set_input_cb(au::Scope::GLOBAL, 0, cb, ref_con)
    }

    #[inline]
    pub fn output_stream_format(&self, bus: u32) -> os::Result<audio::StreamBasicDesc> {
        self.unit().stream_format(Scope::INPUT, bus)
    }

    #[inline]
    pub fn input_stream_format(&self, bus: u32) -> os::Result<audio::StreamBasicDesc> {
        self.unit().stream_format(Scope::OUTPUT, bus)
    }

    pub fn is_running(&self) -> os::Result<bool> {
        let res: u32 = self.unit().prop(
            au::PropId::OUTPUT_IS_RUNNING,
            au::Scope::GLOBAL,
            au::Element(0),
        )?;
        Ok(res != 0)
    }

    pub fn is_io_enabled(&self, scope: au::Scope, bus: u32) -> os::Result<bool> {
        let res: u32 = self
            .unit()
            .prop(au::PropId::OUTPUT_ENABLE_IO, scope, au::Element(bus))?;
        Ok(res != 0)
    }

    pub fn set_io_enabled(&mut self, scope: au::Scope, bus: u32, val: bool) -> os::Result {
        let val = val as u32;
        self.unit_mut()
            .set_prop(au::PropId::OUTPUT_ENABLE_IO, scope, au::Element(bus), &val)
    }

    pub fn has_output_io(&self) -> os::Result<bool> {
        let res: u32 =
            self.unit()
                .prop(au::PropId::OUTPUT_HAS_IO, au::Scope::OUTPUT, au::Element(0))?;
        Ok(res != 0)
    }

    pub fn has_input_io(&self) -> os::Result<bool> {
        let res: u32 =
            self.unit()
                .prop(au::PropId::OUTPUT_HAS_IO, au::Scope::INPUT, au::Element(1))?;
        Ok(res != 0)
    }

    pub fn set_should_allocate_input_buf(&mut self, arg: bool) -> os::Result {
        let arg: u32 = arg as u32;
        self.unit_mut().set_prop(
            au::PropId::SHOULD_ALLOCATE_BUF,
            au::Scope::OUTPUT,
            au::Element(1),
            &arg,
        )
    }

    pub fn set_should_allocate_output_buf(&mut self, arg: bool) -> os::Result {
        let arg: u32 = arg as u32;
        self.unit_mut().set_prop(
            au::PropId::SHOULD_ALLOCATE_BUF,
            au::Scope::INPUT,
            au::Element(0),
            &arg,
        )
    }

    pub fn vp_mute_output(&mut self) -> os::Result<bool> {
        let val: u32 = self.unit().prop(
            au::PropId::VOICE_IO_MUTE_OUTPUT,
            au::Scope::GLOBAL,
            au::Element(1),
        )?;
        Ok(val == 1)
    }

    pub fn vp_set_mute_output(&mut self, arg: bool) -> os::Result {
        let arg: u32 = arg as u32;
        self.unit_mut().set_prop(
            au::PropId::VOICE_IO_MUTE_OUTPUT,
            au::Scope::GLOBAL,
            au::Element(1),
            &arg,
        )
    }

    pub fn vp_enable_agc(&mut self) -> os::Result<bool> {
        let val: u32 = self.unit().prop(
            au::PropId::VOICE_IO_ENABLE_AGC,
            au::Scope::GLOBAL,
            au::Element(1),
        )?;
        Ok(val == 1)
    }

    pub fn vp_set_enable_agc(&mut self, arg: bool) -> os::Result {
        let arg: u32 = arg as u32;
        self.unit_mut().set_prop(
            au::PropId::VOICE_IO_ENABLE_AGC,
            au::Scope::GLOBAL,
            au::Element(1),
            &arg,
        )
    }
    pub fn vp_bypass_voice_processing(&mut self) -> os::Result<bool> {
        let val: u32 = self.unit().prop(
            au::PropId::VOICE_IO_BYPASS_VOICE_PROCESSING,
            au::Scope::GLOBAL,
            au::Element(1),
        )?;
        Ok(val == 1)
    }

    pub fn vp_set_bypass_voice_processing(&mut self, arg: bool) -> os::Result {
        let arg: u32 = arg as u32;
        self.unit_mut().set_prop(
            au::PropId::VOICE_IO_BYPASS_VOICE_PROCESSING,
            au::Scope::GLOBAL,
            au::Element(1),
            &arg,
        )
    }
}

impl Output<UninitializedState> {
    pub fn new_apple() -> os::Result<Self> {
        let desc = audio::ComponentDesc {
            type_: au::Type::OUTPUT.0,
            #[cfg(target_os = "macos")]
            sub_type: au::SubType::HAL_OUTPUT.0,
            #[cfg(not(target_os = "macos"))]
            sub_type: au::SubType::REMOTE_IO.0,
            manufacturer: au::Manufacturer::APPLE.0,
            flags: 0,
            flags_mask: 0,
        };

        let comp = desc
            .into_iter()
            .next()
            .ok_or(au::component_err::UNSUPPORTED_TYPE)?;
        let unit = comp.open_unit()?;
        Ok(Self(unit))
    }

    /// Apple voice processing unit
    ///
    /// Note: It is reported that the echo source need to be specified as the output device.
    /// If no output device is specified, MacOS would take the default output device as echo source.
    /// See [mumble-voip](https://github.com/mumble-voip/mumble/blob/70dea6077854d79d4cb60b3ed1e8f1f8f78963a5/src/mumble/CoreAudio.mm#L614)
    pub fn new_apple_vp() -> os::Result<Self> {
        let desc = audio::ComponentDesc {
            type_: au::Type::OUTPUT.0,
            sub_type: au::SubType::VOICE_PROCESSING_IO.0,
            manufacturer: au::Manufacturer::APPLE.0,
            flags: 0,
            flags_mask: 0,
        };

        let comp = desc
            .into_iter()
            .next()
            .ok_or(au::component_err::UNSUPPORTED_TYPE)?;
        let unit = comp.open_unit()?;
        Ok(Self(unit))
    }

    #[doc(alias = "AudioUnitInitialize")]
    pub fn allocate_resources(self) -> os::Result<Output<InitializedState>> {
        Ok(Output(self.0.initialize()?))
    }

    pub fn start_ts_at_zero(&self) -> os::Result<bool> {
        let res: u32 = self.unit().prop(
            au::PropId::OUTPUT_START_TS_AT_ZERO,
            au::Scope::GLOBAL,
            au::Element(0),
        )?;
        Ok(res != 0)
    }

    pub fn set_start_ts_at_zero(&mut self, val: bool) -> os::Result {
        let val = val as u32;
        self.unit_mut().set_prop(
            au::PropId::OUTPUT_START_TS_AT_ZERO,
            au::Scope::GLOBAL,
            au::Element(0),
            &val,
        )
    }

    #[inline]
    pub fn set_output_stream_format(&mut self, val: &audio::StreamBasicDesc) -> os::Result {
        self.unit_mut().set_stream_format(Scope::INPUT, 0, val)
    }

    #[inline]
    pub fn set_input_stream_format(&mut self, val: &audio::StreamBasicDesc) -> os::Result {
        self.unit_mut().set_stream_format(Scope::OUTPUT, 1, val)
    }

    #[cfg(all(target_os = "macos", feature = "core_audio"))]
    #[inline]
    pub fn output_device(&self) -> os::Result<Device> {
        self.unit().prop(
            au::PropId::OUTPUT_CURRENT_DEVICE,
            Scope::GLOBAL,
            au::Element(0),
        )
    }

    /// Set device after enable IO
    #[cfg(all(target_os = "macos", feature = "core_audio"))]
    #[inline]
    pub fn set_output_device(&mut self, val: &Device) -> os::Result {
        self.unit_mut().set_prop(
            au::PropId::OUTPUT_CURRENT_DEVICE,
            Scope::GLOBAL,
            au::Element(0),
            val,
        )
    }

    #[cfg(all(target_os = "macos", feature = "core_audio"))]
    #[inline]
    pub fn input_device(&self) -> os::Result<Device> {
        self.unit().prop(
            au::PropId::OUTPUT_CURRENT_DEVICE,
            Scope::GLOBAL,
            au::Element(1),
        )
    }

    /// Set device after enable IO
    #[cfg(all(target_os = "macos", feature = "core_audio"))]
    #[inline]
    pub fn set_input_device(&mut self, val: &Device) -> os::Result {
        self.unit_mut().set_prop(
            au::PropId::OUTPUT_CURRENT_DEVICE,
            Scope::GLOBAL,
            au::Element(1),
            val,
        )
    }
}

impl Output<InitializedState> {
    #[inline]
    pub fn deallocate_resources(self) -> os::Result<Output<UninitializedState>> {
        Ok(Output(self.0.unintialize()?))
    }

    #[inline]
    pub fn render<const N: usize>(
        &mut self,
        n_frames: u32,
        buf_list: &mut audio::BufList<N>,
        bus: u32,
    ) -> os::Result {
        let ts = audio::TimeStamp::invalid();
        self.0.render(&ts, bus, n_frames, buf_list)
    }

    #[doc(alias = "AudioOutputUnitStart")]
    #[inline]
    pub fn start(&mut self) -> os::Result {
        unsafe { AudioOutputUnitStart(self.unit_mut()).result() }
    }

    #[doc(alias = "AudioOutputUnitStop")]
    #[inline]
    pub fn stop(&mut self) -> os::Result {
        unsafe { AudioOutputUnitStop(self.unit_mut()).result() }
    }
}

#[link(name = "AudioToolbox", kind = "framework")]
unsafe extern "C-unwind" {
    fn AudioOutputUnitStart(unit: &mut audio::Unit) -> os::Status;
    fn AudioOutputUnitStop(unit: &mut audio::Unit) -> os::Status;
}

#[cfg(all(test, target_os = "macos"))]
mod tests {

    use std::ffi::c_void;

    use crate::{
        at::{self, au},
        core_audio::{Obj, PropAddr, PropElement, PropScope, PropSelector, System},
        os,
    };
    const BUS_IN: u32 = 1;
    const BUS_OUT: u32 = 0;

    #[test]
    fn basics() {
        let mut output = au::Output::new_apple().unwrap();
        let count = output.unit().element_count(au::Scope::INPUT);
        println!("input count {count:?}");
        let count = output.unit().element_count(au::Scope::OUTPUT);
        println!("output count {count:?}");
        let format = output.input_stream_format(BUS_IN).unwrap();
        eprintln!("{format:?}");
        let format = output.output_stream_format(BUS_OUT).unwrap();
        eprintln!("{format:?}");

        assert!(!output.is_running().unwrap());
        assert!(output.start_ts_at_zero().unwrap());

        assert_eq!(
            output.is_io_enabled(au::Scope::INPUT, BUS_IN).unwrap(),
            false
        );
        // An I/O unit’s bus 1 connects to input hardware, such as for recording from a microphone.
        // Input is disabled by default. To enable input, the bus 1 input scope must be enabled,
        // as follows:
        output
            .set_io_enabled(au::Scope::INPUT, BUS_IN, true)
            .unwrap();
        output
            .set_io_enabled(au::Scope::OUTPUT, BUS_OUT, false)
            .unwrap();

        extern "C-unwind" fn input_cb(
            _in_ref_con: *mut c_void,
            _io_action_flags: &mut au::RenderActionFlags,
            _in_timestamp: &at::AudioTimeStamp,
            _in_bus_num: u32,
            _in_number_frames: u32,
            _io_data: *mut at::AudioBufList<1>,
        ) -> os::Status {
            os::Status::NO_ERR
        }

        output.set_input_cb(input_cb, std::ptr::null_mut()).unwrap();

        let device = System::default_input_device().unwrap();
        device.show();

        output.set_input_device(&device).unwrap();
        let mut output = output.allocate_resources().unwrap();

        output.start().unwrap();
        assert!(output.is_running().unwrap());
        output.stop().unwrap();

        assert_eq!(output.last_render_err().unwrap(), os::Status::NO_ERR);
    }

    #[test]
    fn voice_processing() {
        let output = au::Output::new_apple_vp().unwrap();

        let count = output.unit().element_count(au::Scope::INPUT);
        println!("input count {count:?}");
        let count = output.unit().element_count(au::Scope::OUTPUT);
        println!("output count {count:?}");
        let format = output.input_stream_format(BUS_IN).unwrap();
        eprintln!("{format:?}");

        let output_device = output.input_device().unwrap();
        let mut mute_count = 0usize;

        extern "C-unwind" fn callback(
            _obj_id: Obj,
            _number_addresses: u32,
            _addresses: *const PropAddr,
            client_data: *mut usize,
        ) -> os::Status {
            unsafe { client_data.write(1) };
            os::Status::NO_ERR
        }

        let mute_addr = PropAddr {
            selector: PropSelector::DEVICE_MUTE,
            scope: PropScope::INPUT,
            element: PropElement::MAIN,
        };

        output_device
            .add_prop_listener(&mute_addr, callback, &mut mute_count)
            .unwrap();

        output_device.set_prop(&mute_addr, &1u32).unwrap();

        let mut output = output.allocate_resources().unwrap();

        assert_eq!(true, output.vp_mute_output().unwrap());
        output.vp_set_mute_output(false).unwrap();
        assert_eq!(false, output.vp_mute_output().unwrap());

        assert_eq!(true, output.vp_enable_agc().unwrap());
        output.vp_set_enable_agc(false).unwrap();
        assert_eq!(false, output.vp_enable_agc().unwrap());

        assert_eq!(false, output.vp_bypass_voice_processing().unwrap());
        output.vp_set_bypass_voice_processing(true).unwrap();
        assert_eq!(true, output.vp_bypass_voice_processing().unwrap());

        assert_eq!(1, mute_count);

        output.start().unwrap();
        assert!(output.is_running().unwrap());
        output.stop().unwrap();
    }
}
