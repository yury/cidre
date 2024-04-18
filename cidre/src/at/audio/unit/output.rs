use crate::{
    at::{
        au,
        au::{Scope, Unit, UnitRef},
        audio,
        audio::component::{InitializedState, State, UninitializedState},
    },
    os,
};

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
    pub fn last_render_err(&self) -> Result<os::Status, os::Status> {
        self.unit().last_render_err()
    }

    #[inline]
    pub fn set_input_cb<const N: usize, T>(
        &mut self,
        cb: au::RenderCb<N, T>,
        ref_con: *const T,
    ) -> Result<(), os::Status> {
        self.0.set_input_cb(0, cb, ref_con)
    }

    #[inline]
    pub fn remove_input_cb(&mut self) -> Result<(), os::Status> {
        self.0.remove_input_cb(0)
    }

    #[inline]
    pub fn output_stream_format(&self, bus: u32) -> Result<audio::StreamBasicDesc, os::Status> {
        self.unit().stream_format(Scope::OUTPUT, bus)
    }

    #[inline]
    pub fn input_stream_format(&self, bus: u32) -> Result<audio::StreamBasicDesc, os::Status> {
        self.unit().stream_format(Scope::INPUT, bus)
    }

    pub fn is_running(&self) -> Result<bool, os::Status> {
        let res: u32 = self.unit().prop(
            au::PropId::OUTPUT_IS_RUNNING,
            au::Scope::GLOBAL,
            au::Element(0),
        )?;
        Ok(res != 0)
    }

    pub fn is_io_enabled(&self, scope: au::Scope, bus: u32) -> Result<bool, os::Status> {
        let res: u32 = self
            .unit()
            .prop(au::PropId::OUTPUT_ENABLE_IO, scope, au::Element(bus))?;
        Ok(res != 0)
    }

    pub fn set_io_enabled(
        &mut self,
        scope: au::Scope,
        bus: u32,
        val: bool,
    ) -> Result<(), os::Status> {
        let val = val as u32;
        self.unit_mut()
            .set_prop(au::PropId::OUTPUT_ENABLE_IO, scope, au::Element(bus), &val)
    }
}

impl Output<UninitializedState> {
    pub fn new_apple() -> Result<Self, os::Status> {
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

    pub fn allocate_resources(self) -> Result<Output<InitializedState>, os::Status> {
        Ok(Output(self.0.initialize()?))
    }

    pub fn start_ts_at_zero(&self) -> Result<bool, os::Status> {
        let res: u32 = self.unit().prop(
            au::PropId::OUTPUT_START_TS_AT_ZERO,
            au::Scope::GLOBAL,
            au::Element(0),
        )?;
        Ok(res != 0)
    }

    pub fn set_start_ts_at_zero(&mut self, val: bool) -> Result<(), os::Status> {
        let val = val as u32;
        self.unit_mut().set_prop(
            au::PropId::OUTPUT_START_TS_AT_ZERO,
            au::Scope::GLOBAL,
            au::Element(0),
            &val,
        )
    }

    #[inline]
    pub fn set_output_stream_format(
        &mut self,
        val: &audio::StreamBasicDesc,
    ) -> Result<(), os::Status> {
        self.unit_mut().set_stream_format(Scope::OUTPUT, 0, val)
    }

    #[inline]
    pub fn set_input_stream_format(
        &mut self,
        val: &audio::StreamBasicDesc,
    ) -> Result<(), os::Status> {
        self.unit_mut().set_stream_format(Scope::INPUT, 0, val)
    }
}

impl Output<InitializedState> {
    #[inline]
    pub fn deallocate_resources(self) -> Result<Output<UninitializedState>, os::Status> {
        Ok(Output(self.0.unintialize()?))
    }

    #[inline]
    pub fn render<const N: usize>(
        &mut self,
        n_frames: u32,
        buf_list: &mut audio::BufList<N>,
    ) -> Result<(), os::Status> {
        let ts = audio::TimeStamp::invalid();
        self.0.render(&ts, 0, n_frames, buf_list)
    }

    #[doc(alias = "AudioOutputUnitStart")]
    #[inline]
    pub fn start(&mut self) -> Result<(), os::Status> {
        unsafe { AudioOutputUnitStart(self.unit_mut()).result() }
    }

    #[doc(alias = "AudioOutputUnitStop")]
    #[inline]
    pub fn stop(&mut self) -> Result<(), os::Status> {
        unsafe { AudioOutputUnitStop(self.unit_mut()).result() }
    }
}

#[link(name = "AudioToolbox", kind = "framework")]
extern "C" {
    fn AudioOutputUnitStart(unit: &mut audio::Unit) -> os::Status;
    fn AudioOutputUnitStop(unit: &mut audio::Unit) -> os::Status;
}

#[cfg(test)]
mod tests {

    use crate::{at::au, os};

    #[test]
    fn basics() {
        let output = au::Output::new_apple().unwrap();
        let count = output.unit().element_count(au::Scope::INPUT);
        println!("input count {count:?}");
        let count = output.unit().element_count(au::Scope::OUTPUT);
        println!("output count {count:?}");
        let format = output.input_stream_format(0).unwrap();
        eprintln!("{format:?}");
        let format = output.output_stream_format(1).unwrap();
        eprintln!("{format:?}");

        assert!(!output.is_running().unwrap());
        assert!(output.start_ts_at_zero().unwrap());
        let mut output = output.allocate_resources().unwrap();

        assert_eq!(output.is_io_enabled(au::Scope::INPUT, 1).unwrap(), false);
        // An I/O unit’s bus 1 connects to input hardware, such as for recording from a microphone.
        // Input is disabled by default. To enable input, the bus 1 input scope must be enabled,
        // as follows:
        output.set_io_enabled(au::Scope::INPUT, 1, true).unwrap();
        output.start().unwrap();
        assert!(output.is_running().unwrap());
        output.stop().unwrap();

        assert_eq!(output.last_render_err().unwrap(), os::Status::NO_ERR);
    }
}
