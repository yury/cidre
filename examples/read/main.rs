use cidre::{arc, av, cf, cv, objc::autoreleasepool, vn};
use ndarray::{Array2, Axis};
// Import the linfa prelude and KMeans algorithm
use linfa::prelude::*;
use linfa_clustering::Optics;
use linfa_reduction::Pca;
use tokio;

#[tokio::main]
async fn main() {
    let url = cf::URL::from_str("file:/Users/yury/Downloads/0.mov").unwrap();
    let asset = av::URLAsset::with_url(&url, None);

    let tracks = asset
        .load_tracks_with_media_type(av::MediaType::video())
        .await
        .unwrap();

    let options = cf::DictionaryOf::with_keys_values(
        &[cv::pixel_buffer_keys::pixel_format_type()],
        &[cv::PixelFormatType::_420V.to_cf_number().as_type_ref()],
        // for ML tasks reading in BGRA is faster 3:55 vs 5:00
        // if you analyze every frame. If you skip frames it is better to use 420v
        //&[cv::PixelFormatType::_32_BGRA.to_cf_number().as_type_ref()],
    );

    let mut output = av::AssetReaderTrackOutput::with_track(&tracks[0], Some(&options)).unwrap();
    output.set_always_copies_sample_data(false);

    let mut reader = av::AssetReader::with_asset(&asset).unwrap();
    reader.add_output(&output);
    let true = reader.start_reading() else {
        println!("error: {:?}", reader.error());
        println!("status: {:?}", reader.status());
        return;
    };

    // let classify = vn::ClassifyImageRequest::new();
    // let horizon = vn::DetectHorizonRequest::new();
    // let attention = vn::GenerateAttentionBasedSaliencyImageRequest::new();
    // let objectness = vn::GenerateObjectnessBasedSaliencyImageRequest::new();
    let features = vn::GenerateImageFeaturePrintRequest::new();
    //let face_quality = vn::DetectFaceCaptureQualityRequest::new();
    // let text = vn::RecognizeTextRequest::new();
    //let requests_slice: &[&vn::Request] = &[&classify, &horizon, &attention, &objectness, &text];
    let requests_slice: &[&vn::Request] = &[&features];
    let requests = cf::ArrayOf::from_slice(requests_slice);

    let handler = vn::SequenceRequestHandler::new();

    let mut feature_prints: Vec<Vec<f32>> = Vec::with_capacity(50_000);

    let mut prev_frame_featurs: Option<arc::R<vn::FeaturePrintObservation>> = None;

    let mut count = 0;
    while let Some(buf) = output.copy_next_sample_buffer() {
        let Some(image) = buf.image_buffer() else {
            continue;
        };
        if count % 30 == 0 {
        } else {
            count += 1;
            continue;
        }
        let pts = buf.presentation_time_stamp();
        autoreleasepool(|| {
            handler
                .perform_on_cv_pixel_buffer(&requests, &image)
                .unwrap();
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
                    let res = &results[0];
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

    let pca = Pca::params(10);

    let mut arr = Array2::zeros((1446, 2048));
    for (i, mut row) in arr.axis_iter_mut(Axis(0)).enumerate() {
        let d = &feature_prints[i];
        for z in 0..2048 {
            row[z] = d[z];
        }
    }
    // let dataset = Dataset::from(arr);
    // let res = pca.fit(&dataset).unwrap();
    // let arr = res.predict(&dataset);
    //arr.row_mut(0)
    //    let arr = Array2::from(feature_prints);
    println!("arr {:?}", arr);

    let min_points = 5;
    //let clusters = Dbscan::params(min_points).tolerance(5.0).transform(&arr);
    //let clusters = Dbscan::params(min_points).tolerance(5.0).transform(&arr);
    let clusters = Optics::params(min_points)
        // .tolerance(5.0)
        .transform((&arr).into())
        .unwrap();
    println!("{:#?}", clusters);

    for r in clusters.iter() {
        println!(
            "{} {}",
            r.index(),
            r.reachability_distance().unwrap_or_default()
        );
    }

    // https://towardsdatascience.com/how-to-cluster-images-based-on-visual-similarity-cd6e7209fe34
}
