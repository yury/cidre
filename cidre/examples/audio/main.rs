use std::path::PathBuf;

use cidre::{at::audio, cf};
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

fn encode(args: &EncodeArgs) {
    let true = args.src.is_file() else {
        return eprintln!("src file doesn't exists `{}`", args.src.to_string_lossy());
    };
    let src = cf::URL::with_path(args.src.as_path(), false).unwrap();
    let mut src_file =
        audio::FileID::open(&src, audio::FilePermissions::Read, Default::default()).unwrap();

    let src_asbd = src_file.data_format().unwrap();

    if src_asbd.format_id != audio::FormatID::LINEAR_PCM {
        return eprintln!("The input file data format is not PCM");
    };

    println!("{:?}", src_asbd);

    let input_uses_packet_descriptions =
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

    let dst_file = audio::FileID::create(
        &dst,
        audio::FileTypeID::M4A,
        &dst_asbd,
        audio::FileFlags::ERASE_FILE,
    )
    .unwrap();

    let conv = audio::ConverterRef::with_formats(&src_asbd, &dst_asbd).unwrap();

    println!("{src:?} {dst:?}")
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
    let mut in_file =
        audio::FileID::open(&src, audio::FilePermissions::Read, Default::default()).unwrap();

    let dst = match args.dst {
        Some(ref dst) => dst.clone(),
        None => args.src.with_extension("aac"),
    };

    println!("{src:?} {dst:?}")
}
