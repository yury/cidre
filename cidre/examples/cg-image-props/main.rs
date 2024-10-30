use std::path::PathBuf;

use cidre::{cf, cg};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Image file path to analyse
    path: PathBuf,
}

fn main() {
    let args = Args::parse();
    let url = cf::Url::with_path(&args.path, false).unwrap();
    let src = cg::ImageSrc::with_url(&url, None).unwrap();
    let count = src.count();
    println!("images count: {count}");
    let props = src.props(None).unwrap();
    props.show();

    for i in 0..count {
        let img = src.image_at(i, None).unwrap();
        img.show();
    }
}
