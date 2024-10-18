use cidre::{
    arc, av, cf, cg, cm, ns, ut,
    vn::{self, video_processor as vp},
};
use clap::Parser;
use std::{collections::HashMap, num, path::Path};
use tokio::{sync::mpsc, task};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Url for video to analyse
    video_url: String,

    /// Number of frames to analyse
    #[arg(short, default_value_t = unsafe { num::NonZeroU32::new_unchecked(200) } )]
    n: num::NonZeroU32,

    /// Number of top frames to peek
    #[arg(short, default_value_t = unsafe { num::NonZeroU8::new_unchecked(5) } )]
    t: num::NonZeroU8,
}

#[derive(Debug, Clone)]
struct Frame {
    /// The timestamp of the frame.
    ts: cm::Time,

    /// The score of the frame.
    score: f32,

    /// The feature-print observation of the frame.
    observation: arc::R<vn::FeaturePrintObservation>,
}

async fn process_video(
    url: &ns::Url,
    n: num::NonZeroU32,
    top_n: num::NonZeroU8,
) -> ns::Result<(), arc::R<ns::Error>> {
    // The instance of the `VideoProcessor` with the local path to the video file.
    let mut processor = vp::VideoProcessor::with_url(url);

    let asset = av::UrlAsset::with_url(url, None).expect("Failed to create av::UrlAsset");

    let total_duration = asset.duration().as_secs();

    if total_duration <= 0.0 {
        return Ok(());
    }

    let frames_to_eval: i32 = n.get() as i32;
    const TIME_SCALE: i32 = 600;

    // The time interval for the video-processing cadence.
    let interval = cm::Time::with_secs(total_duration / frames_to_eval as f64, TIME_SCALE);

    // The video-processing cadence to process only 100 frames.
    let cadence = vp::TimeIntervalCadence::new(interval.as_secs());

    // The request to calculate the aesthetics score for each frame.
    let (scores_tx, mut scores_rx) = mpsc::unbounded_channel();
    let request = vn::CalcImageAestheticsScoresRequest::with(move |request, _error| {
        if let Some(arr) = request.results() {
            for r in arr.iter() {
                scores_tx.send(r.retained()).unwrap();
            }
        }
    });
    processor
        .add_request_with_cadence(&request, &cadence)
        .expect("Failed to add aesthetics scores request");

    // The request to generate feature prints from an image.
    let (fprint_tx, mut fprint_rx) = mpsc::unbounded_channel();
    let request = vn::GenImageFeaturePrintRequest::with(move |request, _error| {
        if let Some(arr) = request.results() {
            for r in arr.iter() {
                fprint_tx.send(r.retained()).unwrap();
            }
        }
    });
    processor
        .add_request_with_cadence(&request, &cadence)
        .expect("Failed to add feature print request");

    let (complete_tx, mut complete_rx) = tokio::sync::oneshot::channel::<()>();
    let collect_task = tokio::spawn(async move {
        let mut aesthetics_map = HashMap::new();
        let mut feature_print_map = HashMap::new();
        loop {
            tokio::select! {
                Some(v) = fprint_rx.recv() => {
                    let start = v.time_range().start;
                    feature_print_map.insert(start, v);
                    eprint!("\r{:.5}", start.as_secs() / total_duration);
                }
                Some(v) = scores_rx.recv() => {
                    aesthetics_map.insert(v.time_range().start, v.overall_score());
                }
                _ = &mut complete_rx => break
            }
        }
        (aesthetics_map, feature_print_map)
    });

    task::spawn_blocking(move || {
        let r = processor.analyze();
        complete_tx.send(()).unwrap();
        r.map_err(|e| e.retained())
    })
    .await
    .unwrap()?;

    let (a, f) = collect_task.await.unwrap();
    print!("\r");
    let frames = calc_top_frames(&a, &f, top_n).map_err(|e| e.retained())?;
    gen_thumbs(frames, &asset).await?;
    Ok(())
}

fn calc_top_frames<'ear>(
    aesthetic_map: &HashMap<cm::Time, f32>,
    feature_print_map: &HashMap<cm::Time, arc::R<vn::FeaturePrintObservation>>,
    n: num::NonZeroU8,
) -> ns::Result<'ear, Vec<Frame>> {
    // The number of frames to store.
    let top_frames_n: usize = n.get() as _;

    // The threshold for counting the image distance as similar.
    const SIMILARITY_THRESHOLD: f32 = 0.3f32;

    let mut frames: Vec<Frame> = Vec::with_capacity(top_frames_n + 1);

    for (ts, score) in aesthetic_map.iter() {
        let Some(feature_print) = feature_print_map.get(ts) else {
            continue;
        };

        // The new frame at that timestamp.
        let new_frame = Frame {
            ts: *ts,
            score: *score,
            observation: feature_print.clone(),
        };

        // The variable that tracks whether to add the image based on image similarity.
        let mut is_similar = false;

        // The variable to track the index to insert the new frame.
        let mut insert_idx = frames.len();

        // Iterate through the current top-rated frames to check whether any of them
        // are similar to the new frame and find the insertion index.
        for (i, frame) in frames.iter().enumerate() {
            let distance = feature_print.distance_to(&frame.observation)?;
            if distance < SIMILARITY_THRESHOLD {
                // Replace the frame if the new frame has a higher score.
                if &new_frame.score > &frame.score {
                    frames[i] = new_frame.clone();
                }
                is_similar = true;
                break;
            }

            // Comparing the scores to find the insertion index.
            if new_frame.score > frame.score {
                insert_idx = i;
                break;
            }
        }

        // Insert the new frame if it's not similar and
        // has an insertion index within the number of frames to store.
        if !is_similar && insert_idx < top_frames_n {
            frames.insert(insert_idx, new_frame);
            if frames.len() > top_frames_n {
                frames.pop();
            }
        }
    }

    Ok(frames)
}

async fn gen_thumbs<'ear>(
    mut frames: Vec<Frame>,
    asset: &av::UrlAsset,
) -> ns::Result<'ear, (), arc::R<ns::Error>> {
    // The image generator that generates images from the video.
    let mut image_generator = av::AssetImageGenerator::with_asset(asset);

    // Apply the orientation of the source when it generates an image.
    image_generator.set_applies_preferred_track_transform(true);

    let png_type = ut::Type::png().id();
    for (i, frame) in frames.drain(..).enumerate() {
        let (image, actual_time) = image_generator.cg_image_for_time(frame.ts).await?;
        let path = format!("thumb-{i}.png");
        println!("writing {path} ts: {}", actual_time.as_secs());
        let path = Path::new(&path);
        let url = cf::Url::with_path(path, false).unwrap();
        let mut dst = cg::ImageDst::with_url(&url, png_type.as_cf(), 0).unwrap();
        dst.add_image(&image, None);
        dst.finalize();
    }

    Ok(())
}

#[tokio::main]
pub async fn main() {
    let args = Args::parse();
    let url = ns::Url::with_fs_path_str(&args.video_url, false);
    process_video(&url, args.n, args.t).await.unwrap();
}
