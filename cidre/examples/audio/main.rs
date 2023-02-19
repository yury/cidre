use std::path::PathBuf;

use cidre::{at::audio, cf, os};
use clap::Parser;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

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

extern "C" fn input_data_proc(
    _converter: &audio::Converter,
    io_number_data_packets: &mut u32,
    io_data: &mut audio::BufferList,
    #[allow(unused)] mut out_data_packet_descriptions: *mut audio::StreamPacketDescription,
    in_user_data: *mut Context,
) -> os::Status {
    let ctx = unsafe { &mut *in_user_data };

    let buf_len = *io_number_data_packets as usize * ctx.max_packet_size as usize;

    unsafe {
        if ctx.buffer.len() == buf_len {
        } else if ctx.buffer.capacity() >= buf_len {
            ctx.buffer.set_len(buf_len);
        } else {
            ctx.buffer.resize(buf_len, 0u8);
        }
    }

    // let mut descritpions = std::ptr::null_mut();
    out_data_packet_descriptions = if ctx.uses_packet_descriptions {
        println!("use packet descriptions?");
        ctx.packet_descriptions
            .resize(*io_number_data_packets as _, Default::default());
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
        out_data_packet_descriptions,
        ctx.packet,
        io_number_data_packets,
        io_data.buffers[0].data,
    ) {
        Ok(_) => {
            ctx.packet += *io_number_data_packets as isize;
            os::Status::NO_ERR
        }
        Err(e) => e,
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

    // Create the output file as PCM or AAC of the same sampling rate and number of channels as
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

    // let packet_buffer  = std::vector<uint8_t> packetBuffer((size_t)(packetsPerLoop * maxOutputPacketSize));
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
            input_data_proc,
            &mut ctx,
            &mut num_packets,
            &mut list,
            &mut packet_descriptions,
        )
        .unwrap();

        if num_packets > 0 {
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

            starting_packet += num_packets as isize;
        }

        if num_packets < packets_per_loop {
            break;
        }
    }

    let cookie = conv.compression_magic_cookie().unwrap();
    unsafe {
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
    let _in_file =
        audio::FileID::open(&src, audio::FilePermissions::Read, Default::default()).unwrap();

    let dst = match args.dst {
        Some(ref dst) => dst.clone(),
        None => args.src.with_extension("aac"),
    };

    println!("{src:?} {dst:?}")
}
