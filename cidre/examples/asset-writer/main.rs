use std::path::PathBuf;

use cidre::{at::audio, av, cf, os};
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

    assert!(asset_reader.start_reading());

    while let Some(buf) = track_output.copy_next_sample_buffer_throws() {
        println!("buf {}", buf.num_samples());
    }

    asset_reader.cancel_reading();
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
