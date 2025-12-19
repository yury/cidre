#[cfg(target_os = "macos")]
mod macos {
    use cidre::{
        arc,
        at::{
            self, au,
            audio::component::{InitializedState, UninitializedState},
        },
        av, core_audio, ns, os,
    };

    struct Ctx {
        file: arc::R<av::AudioFile>,
        format: arc::R<av::AudioFormat>,
        output: Option<au::Output<InitializedState>>,
        data: Vec<f32>,
    }

    impl Drop for Ctx {
        fn drop(&mut self) {
            if let Some(mut output) = self.output.take() {
                output.stop().unwrap();
            }
            self.file.close();
        }
    }

    impl Ctx {
        fn new(file: arc::R<av::AudioFile>, format: arc::R<av::AudioFormat>) -> Self {
            Self {
                file,
                format,
                output: Default::default(),
                data: Default::default(),
            }
        }

        fn start(&mut self, mut output: au::Output<UninitializedState>) -> os::Result<()> {
            output.set_io_enabled(au::Scope::INPUT, 1, true)?;
            output.set_io_enabled(au::Scope::OUTPUT, 0, false)?;
            output.set_should_allocate_input_buf(false)?;
            output.set_should_allocate_output_buf(false)?;
            output.set_input_cb(Ctx::input_cb, self as *mut Self)?;
            // output.vp_set_enable_agc(false)?;
            output.vp_set_other_audio_ducking_cfg(&au::VoiceIoOtherAudioDuckingCfg {
                enable_advanced_ducking: true,
                ducking_level: au::voice_io_other_audio_ducking_level::MIN,
            })?;

            let output = output.allocate_resources().unwrap();
            self.data = vec![0f32; output.unit().max_frames_per_slice()? as usize];
            self.output = Some(output);
            let output = unsafe { self.output.as_mut().unwrap_unchecked() };
            output.start()?;

            let output_device = core_audio::System::default_output_device()?;
            // unduck output device
            output_device.duck(1.0, None, 0.0)
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
                number_channels: 1,
                data_bytes_size: std::mem::size_of_val(&ctx.data[..]) as u32,
                data: ctx.data.as_mut_ptr() as *mut _,
            };

            if let Err(e) = output.render(in_number_frames, &mut buf_list, 1) {
                return e.status();
            }

            let buf = av::AudioPcmBuf::with_buf_list_no_copy(&ctx.format, &buf_list, None).unwrap();
            ctx.file.write(&buf).unwrap();

            os::Status::NO_ERR
        }
    }

    #[tokio::main]
    pub async fn main() {
        let output = au::Output::new_apple_vp().unwrap();
        let input_device = output.input_device().unwrap();
        let asbd = output
            .input_stream_format(1)
            .expect("Failed to get input stream format");

        let format = av::AudioFormat::with_asbd(&asbd).unwrap();
        let url = ns::Url::with_fs_path_string(ns::str!(c"/tmp/record.aiff"), false);
        let file = av::AudioFile::open_write_common_format(
            &url,
            &format.settings(),
            av::audio::CommonFormat::PcmF32,
            format.is_interleaved(),
        )
        .unwrap();

        println!("format {:#?}", asbd);

        let name = input_device.name().expect("Failed to device name property");

        println!("input device: {name}");

        let buf_list = input_device.input_stream_cfg().unwrap();
        println!("stream cfg: {:?}", buf_list);
        println!("writing audio to {url:?}");

        let mut ctx = Box::new(Ctx::new(file, format));

        ctx.start(output).unwrap();

        tokio::signal::ctrl_c().await.unwrap();
    }
}

#[cfg(target_os = "macos")]
pub use macos::main;

#[cfg(not(target_os = "macos"))]
fn main() {
    todo!()
}
