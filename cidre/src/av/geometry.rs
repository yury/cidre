use crate::{arc, cg, cm, ns, objc};

impl cg::Rect {
    #[doc(alias = "AVMakeRectWithAspectRatioInsideRect")]
    pub fn with_aspect_ratio_inside_rect(aspect_ratio: cg::Size, bounding_rect: cg::Rect) -> Self {
        unsafe { AVMakeRectWithAspectRatioInsideRect(aspect_ratio, bounding_rect) }
    }
}

/// NSValueCMVideoDimensionsExtensions
impl ns::Value {
    #[objc::msg_send(valueWithCMVideoDimensions:)]
    #[objc::available(macos = 13.0, ios = 16.0, tvos = 16.0, watchos = 9.0, visionos = 1.0)]
    pub fn with_cm_video_dimensions(dimensions: cm::VideoDimensions) -> arc::R<Self>;

    #[objc::msg_send(CMVideoDimensionsValue)]
    #[objc::available(macos = 13.0, ios = 16.0, tvos = 16.0, watchos = 9.0, visionos = 1.0)]
    pub fn cm_video_dimensions_value(&self) -> cm::VideoDimensions;
}

extern "C" {
    fn AVMakeRectWithAspectRatioInsideRect(
        aspect_ratio: cg::Size,
        bounding_rect: cg::Rect,
    ) -> cg::Rect;
}

#[cfg(test)]
mod tests {
    use crate::{cg, cm, ns};

    #[test]
    fn basics() {
        let rect = cg::Rect::with_aspect_ratio_inside_rect(
            cg::Size {
                width: 16.0,
                height: 9.0,
            },
            cg::Rect {
                origin: cg::Point { x: 0.0, y: 0.0 },
                size: cg::Size {
                    width: 1000.0,
                    height: 1000.0,
                },
            },
        );

        assert_eq!(
            rect,
            cg::Rect {
                origin: cg::Point { x: 0.0, y: 218.75 },
                size: cg::Size {
                    width: 1000.0,
                    height: 562.5
                }
            }
        );

        let val = ns::Value::with_cm_video_dimensions(cm::VideoDimensions {
            width: 1920,
            height: 1080,
        });

        let val2 = val.cm_video_dimensions_value();

        assert_eq!(val2.width, 1920);
        assert_eq!(val2.height, 1080);
    }
}
