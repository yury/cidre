use cidre::{
    at::{
        self, au,
        audio::component::{InitializedState, UninitializedState},
    },
    cf,
    core_audio::{AudioObjPropAddr, AudioObjPropElement, AudioObjPropScope, AudioObjPropSelector},
    os, vdsp,
};

#[derive(Default)]
struct Ctx {
    output: Option<au::Output<InitializedState>>,
    data: Vec<f32>,
}

impl Ctx {
    fn start(&mut self, mut output: au::Output<UninitializedState>) -> os::Result<()> {
        output.set_io_enabled(au::Scope::INPUT, 1, true)?;
        output.set_io_enabled(au::Scope::OUTPUT, 0, false)?;
        output.set_should_allocate_input_buf(false)?;
        output.set_should_allocate_output_buf(false)?;
        output.set_input_cb(Ctx::input_cb, self as *mut Self)?;
        let output = output.allocate_resources().unwrap();
        let frames_per_slice = output.unit().max_frames_per_slice()? as usize;
        self.data = vec![0f32; frames_per_slice * 2];
        self.output = Some(output);
        let output = unsafe { self.output.as_mut().unwrap_unchecked() };
        output.start()
    }

    extern "C-unwind" fn input_cb(
        ctx: *mut Ctx,
        _io_action_flags: &mut au::RenderActionFlags,
        _in_timestamp: &at::AudioTimeStamp,
        _in_bus_num: u32,
        in_number_frames: u32,
        _io_data: *mut at::AudioBufList<1>,
    ) -> os::Status {
        if ctx.is_null() {
            return au::err::NO_CONNECTION.into();
        }
        let ctx = unsafe { &mut *ctx };

        if ctx.output.is_none() {
            return au::err::NO_CONNECTION.into();
        }

        let output = unsafe { ctx.output.as_mut().unwrap_unchecked() };

        let mut buf_list = at::AudioBufList::<1>::new();
        buf_list.buffers[0] = at::AudioBuf {
            number_channels: 2,
            data_bytes_size: std::mem::size_of_val(&ctx.data[..]) as u32,
            data: ctx.data.as_mut_ptr() as *mut _,
        };

        if let Err(e) = output.render(in_number_frames, &mut buf_list, 1) {
            return e.status();
        }

        let min_left = vdsp::min_stride_f32(&ctx.data[..in_number_frames as usize * 2], 2);
        let min_right = vdsp::min_stride_f32(&ctx.data[1..in_number_frames as usize * 2], 2);
        let max_left = vdsp::max_stride_f32(&ctx.data[..in_number_frames as usize * 2], 2);
        let max_right = vdsp::max_stride_f32(&ctx.data[1..in_number_frames as usize * 2], 2);
        println!("{min_left} {min_right} {max_left} {max_right}");
        os::Status::NO_ERR
    }
}

#[tokio::main]
async fn main() {
    let output = au::Output::new_apple_vp().unwrap();
    let device = output.current_device().unwrap();
    let asbd = output
        .input_stream_format(0)
        .expect("Failed to get output stream format");

    println!("format {:#?}", asbd);

    let name = device
        .cf_prop::<cf::String>(&AudioObjPropAddr {
            selector: AudioObjPropSelector::NAME,
            scope: AudioObjPropScope::INPUT,
            element: AudioObjPropElement::MAIN,
        })
        .expect("Failed to device name property");

    println!("name {name}");
    println!("device {device:?}");

    let mut ctx = Box::new(Ctx::default());

    let _s = "_⎽⎼—⎻⎺‾";
    let s = "▁▂▃▄▅▆▇█";

    ctx.start(output).unwrap();

    println!("{s}");

    tokio::signal::ctrl_c().await.unwrap();
}
