use crate::{arc, cm, define_cls, define_obj_type, ns, objc, vn};

define_obj_type!(
    #[doc(alias = "VNStatefulRequest")]
    pub StatefulRequest(vn::ImageBasedRequest)
);

impl arc::A<StatefulRequest> {
    #[objc::msg_send(initWithFrameAnalysisSpacing:completionHandler:)]
    pub fn init_with_frame_analysis_spacing_ch(
        self,
        frame_analysis_spacing: cm::Time,
        ch: Option<&mut vn::RequestCh>,
    ) -> arc::R<StatefulRequest>;
}

impl StatefulRequest {
    define_cls!(VN_STATEFUL_REQUEST);

    pub fn with_frame_analysis_spacing_ch(
        frame_analysis_spacing: cm::Time,
        ch: Option<&mut vn::RequestCh>,
    ) -> arc::R<Self> {
        Self::alloc().init_with_frame_analysis_spacing_ch(frame_analysis_spacing, ch)
    }

    pub fn with_frame_analysis_spacing(
        frame_analysis_spacing: cm::Time,
        completion: impl FnMut(&mut vn::Request, Option<&ns::Error>) + 'static,
    ) -> arc::R<Self> {
        let mut block = vn::RequestCh::new2(completion);
        Self::with_frame_analysis_spacing_ch(frame_analysis_spacing, Some(&mut block))
    }

    /// The minimum number of frames that the request has to process on before reporting back any observation.
    /// This information is provided by the request once initialized with its required paramters.
    ///
    /// Video based request often need a minimum number of frames before they can report back any observation.
    /// An example would be that a movement detection requires at least 5 frames to be detected. The minimum_latency_frame_count for that
    /// request would report 5 and only after 5 frames have been processed an observation would be returned in the results.
    /// This latency is indicative of how responsive a request is in respect to the incoming data.
    #[objc::msg_send(minimumLatencyFrameCount)]
    pub fn minimum_latency_frame_count(&self) -> isize;

    /// The reciprocal of maximum rate at which buffers will be processed.
    ///
    /// The request will not process buffers that fall within the `frame_analysis_spacing` after it has performed the analysis.
    /// The analysis is not done by wall time but by analysis of of the time stamps of the samplebuffers being processed.
    #[objc::msg_send(frameAnalysisSpacing)]
    pub fn frame_analysis_spacing(&self) -> cm::Time;
}

#[link(name = "vn", kind = "static")]
extern "C" {
    static VN_STATEFUL_REQUEST: &'static objc::Class<StatefulRequest>;
}

#[cfg(test)]
mod tests {
    use crate::{cm, vn};
    #[test]
    pub fn basics() {
        let request = vn::StatefulRequest::with_frame_analysis_spacing_ch(cm::Time::zero(), None);
        assert_eq!(request.frame_analysis_spacing(), cm::Time::zero());

        let request =
            vn::StatefulRequest::with_frame_analysis_spacing(cm::Time::zero(), |request, error| {
                if let Some(err) = error {
                    println!("err {err:?}");
                    return;
                }
                let _r = request.results();
            });
        assert_eq!(request.frame_analysis_spacing(), cm::Time::zero());
    }
}
