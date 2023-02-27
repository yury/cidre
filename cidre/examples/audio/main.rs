use std::path::PathBuf;

use cidre::{at::audio, cf, os};
use clap::Parser;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
enum Commands {
    /// Encode wav file to aac file
    #[clap(alias = "e", alias = "enc")]
    Encode(EncodeArgs),

    /// Decode file to wav file
    #[clap(alias = "d", alias = "dec")]
    Decode(DecodeArgs),
}

impl Commands {
    fn run(&self) {
        match self {
            Self::Encode(args) => encode(args),
            Self::Decode(args) => decode(args),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    cli.command.run()
}

#[derive(clap::Args, Debug)]
struct EncodeArgs {
    #[arg(value_name = "WAV FILE")]
    src: PathBuf,

    #[arg(value_name = "AAC FILE")]
    dst: Option<PathBuf>,

    #[arg(
        short,
        long,
        value_name = "skip write to file",
        default_value_t = false
    )]
    skip_write: bool,
}

#[repr(C)]
struct Context {
    packet: isize,
    buffer: Vec<u8>,
    file: audio::FileID,
    asbd: audio::StreamBasicDescription,
    max_packet_size: u32,
    uses_packet_descriptions: bool,
    packet_descriptions: Vec<audio::StreamPacketDescription>,
}

extern "C" fn data_proc(
    _converter: &audio::Converter,
    io_number_data_packets: &mut u32,
    io_data: &mut audio::BufferList,
    out_data_packet_descriptions: *mut *mut audio::StreamPacketDescription,
    in_user_data: *mut Context,
) -> os::Status {
    let ctx = unsafe { &mut *in_user_data };

    let buf_len = *io_number_data_packets as usize * ctx.max_packet_size as usize;

    if ctx.buffer.len() != buf_len {
        ctx.buffer.resize(buf_len, 0u8);
    }

    let packet_descriptions_ptr = if ctx.uses_packet_descriptions {
        if ctx.packet_descriptions.len() != *io_number_data_packets as usize {
            ctx.packet_descriptions
                .resize(*io_number_data_packets as _, Default::default());
        }
        unsafe { *out_data_packet_descriptions = ctx.packet_descriptions.as_mut_ptr() };
        ctx.packet_descriptions.as_mut_ptr()
    } else {
        std::ptr::null_mut()
    };

    io_data.number_buffers = 1;
    io_data.buffers[0].number_channels = ctx.asbd.channels_per_frame;
    io_data.buffers[0].data_bytes_size = ctx.buffer.len() as _;
    io_data.buffers[0].data = ctx.buffer.as_mut_ptr();

    match ctx.file.read_packets(
        true,
        &mut io_data.buffers[0].data_bytes_size,
        packet_descriptions_ptr,
        ctx.packet,
        io_number_data_packets,
        io_data.buffers[0].data,
    ) {
        Ok(_) => {
            ctx.packet += *io_number_data_packets as isize;
            os::Status::NO_ERR
        }
        Err(e) => {
            eprintln!("error {e:?}");
            e
        }
    }
}

fn encode(args: &EncodeArgs) {
    let true = args.src.is_file() else {
        return eprintln!("src file doesn't exists `{}`", args.src.to_string_lossy());
    };
    let src = cf::URL::with_path(args.src.as_path(), false).unwrap();
    let src_file =
        audio::FileID::open(&src, audio::FilePermissions::Read, Default::default()).unwrap();

    let src_asbd = src_file.data_format().unwrap();

    if src_asbd.format_id != audio::FormatID::LINEAR_PCM {
        return eprintln!("The input file data format is not PCM");
    };

    let src_uses_packet_descriptions =
        src_asbd.bytes_per_packet == 0 || src_asbd.frames_per_packet == 0;

    // Create the dst file as AAC of the same sampling rate and number of channels as
    // the input.
    let dst_asbd = audio::StreamBasicDescription {
        sample_rate: src_asbd.sample_rate,
        channels_per_frame: src_asbd.channels_per_frame,
        format_id: audio::FormatID::MPEG4_AAC,
        format_flags: audio::FormatFlags::ALL_CLEAR,
        frames_per_packet: 1024,
        ..Default::default()
    };

    let dst = match args.dst {
        Some(ref dst) => dst.clone(),
        None => args.src.with_extension("m4a"),
    };
    let dst = cf::URL::with_path(dst.as_path(), false).unwrap();

    let mut dst_file = audio::FileID::create(
        &dst,
        audio::FileTypeID::M4A,
        &dst_asbd,
        audio::FileFlags::ERASE_FILE,
    )
    .unwrap();

    let conv = audio::ConverterRef::with_formats(&src_asbd, &dst_asbd).unwrap();
    let max_src_packet_size = src_asbd.bytes_per_packet;
    let max_dst_packet_size = conv.maximum_output_packet_size().unwrap();

    let packets_per_loop = 100u32;

    let mut packet_descriptions =
        vec![audio::StreamPacketDescription::default(); packets_per_loop as _];

    let mut packet_buffer = vec![0u8; packets_per_loop as usize * max_dst_packet_size as usize];
    let mut starting_packet = 0isize;

    let mut ctx = Context {
        packet: 0,
        buffer: Default::default(),
        file: src_file,
        asbd: src_asbd,
        max_packet_size: max_src_packet_size,
        uses_packet_descriptions: src_uses_packet_descriptions,
        packet_descriptions: Default::default(),
    };

    loop {
        let mut num_packets = packets_per_loop;

        let mut list = audio::BufferList {
            number_buffers: 1,
            buffers: [audio::Buffer {
                number_channels: dst_asbd.channels_per_frame,
                data_bytes_size: packet_buffer.len() as _,
                data: packet_buffer.as_mut_ptr(),
            }],
        };

        conv.fill_complex_buf_desc(
            data_proc,
            &mut ctx,
            &mut num_packets,
            &mut list,
            &mut packet_descriptions,
        )
        .unwrap();

        if num_packets > 0 {
            if !args.skip_write {
                dst_file
                    .write_packets(
                        true,
                        list.buffers[0].data_bytes_size,
                        packet_descriptions.as_ptr(),
                        starting_packet,
                        &mut num_packets,
                        list.buffers[0].data as _,
                    )
                    .unwrap();
            }

            starting_packet += num_packets as isize;
        }

        if num_packets < packets_per_loop {
            break;
        }
    }

    // Obtain the magic cookie from the encoder and write it to the file.
    // Note that the sample waits until the end of the encoding to do this, because the magic cookie
    // may update during the encoding process.
    let cookie = conv.compression_magic_cookie().unwrap();
    unsafe {
        if !args.skip_write {
            dst_file
                .set_property(
                    audio::FilePropertyID::MAGIC_COOKIE_DATA,
                    cookie.len() as _,
                    cookie.as_ptr() as _,
                )
                .result()
                .unwrap();
        }
    }
}

#[derive(clap::Args, Debug)]
struct DecodeArgs {
    #[arg(value_name = "AUDIO FILE")]
    src: PathBuf,

    #[arg(value_name = "WAV FILE")]
    dst: Option<PathBuf>,
}

fn decode(args: &DecodeArgs) {
    let true = args.src.is_file() else {
        return eprintln!("src file doesn't exists `{}`", args.src.to_string_lossy());
    };
    let src = cf::URL::with_path(args.src.as_path(), false).unwrap();
    let src_file =
        audio::FileID::open(&src, audio::FilePermissions::Read, Default::default()).unwrap();

    let src_asbd = src_file.data_format().unwrap();

    let src_uses_packet_descriptions =
        src_asbd.bytes_per_packet == 0 || src_asbd.frames_per_packet == 0;

    let dst_asbd = audio::StreamBasicDescription {
        sample_rate: src_asbd.sample_rate,
        channels_per_frame: src_asbd.channels_per_frame,
        format_id: audio::FormatID::LINEAR_PCM,
        format_flags: audio::FormatFlags::IS_FLOAT | audio::FormatFlags::IS_PACKED,
        bytes_per_packet: 4 * src_asbd.channels_per_frame,
        frames_per_packet: 1,
        bytes_per_frame: 4 * src_asbd.channels_per_frame,
        bits_per_channel: 32,
        ..Default::default()
    };
    let dst = match args.dst {
        Some(ref dst) => dst.clone(),
        None => args.src.with_extension("wav"),
    };
    let dst = cf::URL::with_path(dst.as_path(), false).unwrap();

    let mut dst_file = audio::FileID::create(
        &dst,
        audio::FileTypeID::WAVE,
        &dst_asbd,
        audio::FileFlags::ERASE_FILE,
    )
    .unwrap();

    let packets_per_loop = 10_000u32;

    let mut conv = audio::ConverterRef::with_formats(&src_asbd, &dst_asbd).unwrap();

    match src_file.magic_cookie_data() {
        Ok(cookie) => {
            conv.set_decompression_magic_cookie(cookie).unwrap();
        }
        Err(audio::file_errors::UNSUPPORTED_PROPERTY) => {}
        Err(e) => {
            return eprintln!("Error {e:?}");
        }
    }

    let max_src_packet_size = src_file.maximum_packet_size().unwrap();
    let max_dst_packet_size = dst_asbd.bytes_per_packet;

    let mut packet_buffer = vec![0u8; packets_per_loop as usize * max_dst_packet_size as usize];
    let mut starting_packet = 0isize;

    let mut ctx = Context {
        packet: 0,
        buffer: Default::default(),
        file: src_file,
        asbd: src_asbd,
        max_packet_size: max_src_packet_size,
        uses_packet_descriptions: src_uses_packet_descriptions,
        packet_descriptions: Default::default(),
    };

    loop {
        let mut num_packets = packets_per_loop;

        let mut list = audio::BufferList {
            number_buffers: 1,
            buffers: [audio::Buffer {
                number_channels: dst_asbd.channels_per_frame,
                data_bytes_size: packet_buffer.len() as _,
                data: packet_buffer.as_mut_ptr(),
            }],
        };

        conv.fill_complex_buf(data_proc, &mut ctx, &mut num_packets, &mut list)
            .unwrap();

        if num_packets > 0 {
            dst_file
                .write_packets(
                    true,
                    list.buffers[0].data_bytes_size,
                    std::ptr::null(),
                    starting_packet,
                    &mut num_packets,
                    list.buffers[0].data,
                )
                .unwrap();

            starting_packet += num_packets as isize;
        }

        if num_packets < packets_per_loop {
            break;
        }
    }
}
