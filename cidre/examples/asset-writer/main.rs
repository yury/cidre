use std::{path::PathBuf, sync::Arc};

use cidre::{at::audio, av, blocks, cf, cm, dispatch, os};
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
    async fn run(&self) {
        match self {
            Self::Encode(args) => encode(args).await,
            Self::Decode(args) => decode(args).await,
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    cli.command.run().await;
}

#[derive(clap::Args, Debug)]
struct EncodeArgs {
    #[arg(value_name = "WAV FILE")]
    src: PathBuf,

    #[arg(value_name = "AAC FILE")]
    dst: Option<PathBuf>,
}

async fn encode(args: &EncodeArgs) {
    let true = args.src.is_file() else {
        return eprintln!("src file doesn't exists `{}`", args.src.to_string_lossy());
    };

    let src = cf::URL::with_path(args.src.as_path(), false).unwrap();

    let src_asset = av::URLAsset::with_url(src.as_ns_url(), None).unwrap();
    let mut asset_reader = av::AssetReader::with_asset(&src_asset).unwrap();

    println!("asset reader {asset_reader:?}");
    let tracks = src_asset
        .load_tracks_with_media_type(av::MediaType::audio())
        .await
        .unwrap();

    let mut track_output = av::AssetReaderTrackOutput::with_track(&tracks[0], None).unwrap();
    track_output.set_always_copies_sample_data(false);

    asset_reader.add_output(&track_output).unwrap();
    let dst = match args.dst {
        Some(ref dst) => dst.clone(),
        None => args.src.with_extension("m4a"),
    };

    if dst.exists() {
        std::fs::remove_file(&dst).unwrap();
    }

    let dst = cf::URL::with_path(dst.as_path(), false).unwrap();

    let mut asset_writer =
        av::AssetWriter::with_url_and_file_type(dst.as_ns_url(), av::FileType::m4a()).unwrap();

    assert!(asset_reader.start_reading());
    let mut buf = track_output.copy_next_sample_buffer_throws().unwrap();
    let fd = buf.format_description().unwrap();
    let src_asbd = fd.stream_basic_description().unwrap();

    let desc = cm::AudioFormatDescription::with_asbd(&src_asbd).unwrap();

    let input =
        av::AssetWriterInput::with_media_type_format_hint_throws(av::MediaType::audio(), &desc);
    asset_writer.add_input(&input).unwrap();
    asset_writer.start_writing();
    asset_writer.start_session_at_source_time(cm::Time::zero());

    let sema = Arc::new(dispatch::Semaphore::new(0));
    let queue = dispatch::Queue::serial_with_autoreleasepool();
    let mut inp = input.retained();
    let sem = sema.clone();

    let mut block = blocks::mut0(move || {
        while inp.is_ready_for_more_media_data() {
            inp.append_sample_buffer_throws(&buf);
            let Some(b) = track_output.copy_next_sample_buffer_throws() else {
                inp.mark_as_finished();
                sem.signal();
                break;
            };
            buf = b;
        }
    });

    input.request_media_data_when_ready_on_queue_throws(&queue, block.escape());

    sema.wait_forever();

    asset_reader.cancel_reading();
    asset_writer.finish_writing();
}

#[derive(clap::Args, Debug)]
struct DecodeArgs {
    #[arg(value_name = "AUDIO FILE")]
    src: PathBuf,

    #[arg(value_name = "WAV FILE")]
    dst: Option<PathBuf>,
}

async fn decode(args: &DecodeArgs) {
    let true = args.src.is_file() else {
        return eprintln!("src file doesn't exists `{}`", args.src.to_string_lossy());
    };
    let src = cf::URL::with_path(args.src.as_path(), false).unwrap();
}
