use crate::{arc, av, cm, define_obj_type, ns, objc};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[doc(alias = "AVAssetSegmentType")]
#[repr(isize)]
pub enum SegmentType {
    /// Indicates that the segment is a initialization segment.
    #[doc(alias = "AVAssetSegmentTypeInitialization")]
    Initialization = 1,
    /// Indicates that the segment is a separable segment.
    #[doc(alias = "AVAssetSegmentTypeSeparable")]
    Separable = 2,
}

define_obj_type!(
    #[doc(alias = "AVAssetSegmentReport")]
    pub SegmentReport(ns::Id)
);

unsafe impl Send for SegmentReport {}

impl SegmentReport {
    /// A segment type of the segment data.
    #[objc::msg_send(segmentType)]
    pub fn segment_type(&self) -> SegmentType;

    #[objc::msg_send(trackReports)]
    pub fn track_reports(&self) -> arc::R<ns::Array<TrackReport>>;
}

define_obj_type!(
    #[doc(alias = "AVAssetSegmentTrackReport")]
    pub TrackReport(ns::Id)
);

unsafe impl Send for TrackReport {}

impl TrackReport {
    #[objc::msg_send(trackID)]
    pub fn track_id(&self) -> cm::PersistentTrackId;

    #[objc::msg_send(mediaType)]
    pub fn media_type(&self) -> av::MediaType;

    /// Indicates the earliest presentation timestamp (PTS) for this track.
    /// The value is kCMTimeInvalid if there is no information available.
    #[objc::msg_send(earliestPresentationTimeStamp)]
    pub fn earliest_pts(&self) -> cm::Time;

    /// Indicates the duration for this track. The value is kCMTimeInvalid
    /// if there is no information available.
    #[objc::msg_send(duration)]
    pub fn duration(&self) -> cm::Time;

    #[objc::msg_send(firstVideoSampleInformation)]
    pub fn first_sample_info(&self) -> &SampleInfo;
}

define_obj_type!(
    #[doc(alias = "AVAssetSegmentReportSampleInformation")]
    pub SampleInfo(ns::Id)
);

unsafe impl Send for SampleInfo {}

impl SampleInfo {
    /// The presentation timestamp (PTS) of the sample.
    /// This timestamp may be different from the earliest_pts if the video is encoded using frame reordering.
    #[objc::msg_send(presentationTimeStamp)]
    pub fn pts(&self) -> cm::Time;

    #[objc::msg_send(offset)]
    pub fn offset(&self) -> isize;

    /// The length of the sample.
    #[objc::msg_send(length)]
    pub fn length(&self) -> isize;

    #[inline]
    pub fn len(&self) -> usize {
        self.length() as _
    }

    /// Indicates whether the sample is a sync sample.
    #[objc::msg_send(isSyncSample)]
    pub fn is_sync(&self) -> bool;
}
