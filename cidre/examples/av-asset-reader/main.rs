use cidre::{arc, av, cv, ns, objc::ar_pool, vn};
use tokio;

#[tokio::main]
async fn main() {
    let url = ns::Url::with_str("file:/Users/yury/Downloads/0.mov").unwrap();
    let asset = av::UrlAsset::with_url(&url, None).expect("asset");

    let tracks = asset
        .load_tracks_with_media_type(av::MediaType::video())
        .await
        .unwrap();

    let options = ns::Dictionary::with_keys_values(
        &[cv::pixel_buffer_keys::pixel_format().as_ns()],
        &[cv::PixelFormat::_420V.to_cf_number().as_ns().as_id_ref()],
        // for ML tasks reading in BGRA is faster 3:55 vs 5:00
        // if you analyze every frame. If you skip frames it is better to use 420v
        //&[cv::PixelFormat::_32_BGRA.to_cf_number().as_type_ref()],
    );

    let mut output =
        av::AssetReaderTrackOutput::with_track(&tracks.get(0).unwrap(), Some(&options)).unwrap();
    // let mut output = av::AssetReaderTrackOutput::with_track(&tracks[0], None).unwrap();
    output.set_always_copies_sample_data(false);

    let mut reader = av::AssetReader::with_asset(&asset).unwrap();
    reader.add_output(&output).unwrap();
    let true = reader.start_reading().expect("Failed to start reading") else {
        println!("error: {:?}", reader.error());
        println!("status: {:?}", reader.status());
        return;
    };

    // let classify = vn::ClassifyImageRequest::new();
    // let horizon = vn::DetectHorizonRequest::new();
    // let attention = vn::GenerateAttentionBasedSaliencyImageRequest::new();
    // let objectness = vn::GenerateObjectnessBasedSaliencyImageRequest::new();
    let features = vn::GenImageFeaturePrintRequest::new();
    //let face_quality = vn::DetectFaceCaptureQualityRequest::new();
    // let text = vn::RecognizeTextRequest::new();
    //let requests_slice: &[&vn::Request] = &[&classify, &horizon, &attention, &objectness, &text];
    let requests_slice: &[&vn::Request] = &[&features];
    let requests = ns::Array::from_slice(requests_slice);

    let handler = vn::SequenceRequestHandler::new();

    let mut feature_prints: Vec<Vec<f32>> = Vec::with_capacity(50_000);

    let _prev_frame_featurs: Option<arc::R<vn::FeaturePrintObservation>> = None;

    let mut count = 0;
    while let Some(buf) = unsafe { output.next_sample_buf_throws() } {
        let Some(image) = buf.image_buf() else {
            continue;
        };
        if count % 30 == 0 {
        } else {
            count += 1;
            continue;
        }
        let _pts = buf.pts();
        ar_pool(|| {
            handler.perform_on_cv_pixel_buf(&requests, &image).unwrap();
            // if let Some(results) = classify.results() {
            //     if !results.is_empty() {
            //         let ids = [
            //             results[0].identifier().to_string(),
            //             results[1].identifier().to_string(),
            //             results[2].identifier().to_string(),
            //             results[3].identifier().to_string(),
            //             results[5].identifier().to_string(),
            //             results[6].identifier().to_string(),
            //             results[7].identifier().to_string(),
            //             results[8].identifier().to_string(),
            //             results[9].identifier().to_string(),
            //             results[10].identifier().to_string(),
            //             results[11].identifier().to_string(),
            //             results[12].identifier().to_string(),
            //         ]
            //         .join(", ");

            //         println!("{}, {}", count, ids)
            //     }
            // }
            // if let Some(results) = horizon.results() {
            //     if !results.is_empty() {
            //         println!("{:?}", results[0].angle());
            //     }
            // }
            // if let Some(results) = objectness.results() {
            //     if !results.is_empty() {
            //         println!("{:?}", results[0]);
            //     }
            // }
            // if let Some(results) = attention.results() {
            //     if !results.is_empty() {
            //         println!("{:?}", results[0].salient_objects().unwrap());
            //     }
            // }
            // if let Some(results) = text.results() {
            //     if !results.is_empty() {
            //         let res = &results[0].top_candidates(1)[0];
            //         println!("res {:?}", res.string());
            //     }
            // }
            if let Some(results) = features.results() {
                if !results.is_empty() {
                    let res = results.get(0).unwrap();
                    feature_prints.push(res.vec_f32());

                    // if let Some(prev) = prev_frame_featurs.as_ref() {
                    //     let dist = res.compute_distance(&prev).unwrap();
                    //     println!("pts: {:.2} dist: {}", pts.seconds(), dist,);
                    // }
                    // prev_frame_featurs = Some(res.retained());
                }
            }

            // if let Some(results) = face_quality.results() {
            //     if !results.is_empty() {
            //         if let Some(res) = &results[0].face_capture_quality() {
            //             println!("face q: {:.2}", res.to_f64().unwrap());
            //         }
            //     }
            // }
        });

        count += 1;
    }

    println!(
        "count {:?}, {:?} {:?}",
        count,
        reader.status(),
        feature_prints.len()
    );

    // https://towardsdatascience.com/how-to-cluster-images-based-on-visual-similarity-cd6e7209fe34
}
