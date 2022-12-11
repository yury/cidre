use cidre::{av, cf, cv};
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

    let num = cv::PixelFormatType::_420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE.to_cf_number();

    let options = cf::DictionaryOf::with_keys_values(
        &[cv::pixel_buffer_keys::pixel_format_type()],
        &[num.as_type_ref()],
    )
    .unwrap();

    let mut output = av::asset::ReaderTrackOutput::with_track(&tracks[0], Some(&options)).unwrap();
    output.set_always_copies_sample_data(false);
    reader.add_output(&output);
    let true = reader.start_reading() else {
        println!("error: {:?}", reader.error());
        println!("status: {:?}", reader.status());
        return;
    };

    let mut count = 0;
    while let Some(buf) = output.copy_next_sample_buffer() {
        let Some(_image) = buf.image_buffer() else {
            continue;
        };

        //println!("width {:?}", image.width());
        count += 1;
    }

    println!("count {:?}, {:?}", count, reader.status());
}
