use cidre::{av, cf};
use tokio;

#[tokio::main]
async fn main() {
    let url = cf::URL::from_str("file:/Users/yury/Downloads/0.mov").unwrap();
    let asset = av::URLAsset::with_url(&url, None).unwrap();

    let tracks = asset
        .load_tracks_with_media_type(&av::MediaType::video())
        .await
        .unwrap();

    let mut reader = av::AssetReader::with_asset(&asset).unwrap();

    let mut output = av::asset::ReaderTrackOutput::with_track(&tracks[0], None).unwrap();
    output.set_always_copies_sample_data(false);
    reader.add_output(&output);
    let res = reader.start_reading();

    if !res {
        println!("error: {:?}", reader.error());
        println!("status: {:?}", reader.status());
        return;
    }
    let mut count = 0;
    while let Some(_buf) = output.copy_next_sample_buffer() {
        //  println!("hmm");
        count += 1;
    }

    println!("count {:?}", count);
}
