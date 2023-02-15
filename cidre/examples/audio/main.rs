use std::path::PathBuf;

use cidre::cf;
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

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Encode(ref args) => encode(args),
        Commands::Decode(ref args) => decode(args),
    }
}

#[derive(clap::Args, Debug)]
struct EncodeArgs {
    #[arg(value_name = "WAV FILE")]
    src: PathBuf,

    #[arg(value_name = "AAC FILE")]
    dst: Option<PathBuf>,
}

fn encode(args: &EncodeArgs) {
    let src = cf::URL::with_path(args.src.as_path(), false).unwrap();
    println!("{src:?}")
}

#[derive(clap::Args, Debug)]
struct DecodeArgs {
    #[arg(value_name = "AUDIO FILE")]
    src: PathBuf,

    #[arg(value_name = "WAV FILE")]
    dst: Option<PathBuf>,
}

fn decode(args: &DecodeArgs) {}
