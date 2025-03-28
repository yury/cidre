use crate::{arc, cg, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "NSColor")]
    pub Color(ns::Id), NS_COLOR
);

impl Color {
    #[objc::msg_send(colorWithWhite:alpha:)]
    pub fn with_white_alpha(white: cg::Float, alpha: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(colorWithRed:green:blue:alpha:)]
    pub fn with_rgba(r: cg::Float, g: cg::Float, b: cg::Float, a: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(colorWithHue:saturation:brightness:alpha:)]
    pub fn with_hsba(h: cg::Float, s: cg::Float, b: cg::Float, a: cg::Float) -> arc::R<Self>;

    #[objc::msg_send(colorNamed:)]
    pub fn color_named(name: &ns::String) -> Option<arc::R<Self>>;

    pub fn named(name: impl AsRef<ns::String>) -> Option<arc::R<Self>> {
        Self::color_named(name.as_ref())
    }

    #[objc::msg_send(whiteComponent)]
    pub unsafe fn white_throws(&self) -> cg::Float;

    pub fn white<'ar>(&self) -> Result<cg::Float, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.white_throws() })
    }

    #[objc::msg_send(alphaComponent)]
    pub fn alpha(&self) -> cg::Float;

    #[objc::msg_send(redComponent)]
    pub unsafe fn red_throws(&self) -> cg::Float;

    #[objc::msg_send(greenComponent)]
    pub unsafe fn green_throws(&self) -> cg::Float;

    #[objc::msg_send(blueComponent)]
    pub unsafe fn blue_throws(&self) -> cg::Float;

    pub fn red<'ar>(&self) -> Result<cg::Float, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.red_throws() })
    }

    pub fn green<'ar>(&self) -> Result<cg::Float, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.green_throws() })
    }

    pub fn blue<'ar>(&self) -> Result<cg::Float, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.blue_throws() })
    }

    #[objc::msg_send(hueComponent)]
    pub unsafe fn hue_throws(&self) -> cg::Float;

    #[objc::msg_send(saturationComponent)]
    pub unsafe fn saturation_throws(&self) -> cg::Float;

    #[objc::msg_send(brightnessComponent)]
    pub unsafe fn brightness_throws(&self) -> cg::Float;

    pub fn hue<'ar>(&self) -> Result<cg::Float, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.hue_throws() })
    }

    pub fn saturation<'ar>(&self) -> Result<cg::Float, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.saturation_throws() })
    }

    pub fn brightness<'ar>(&self) -> Result<cg::Float, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.brightness_throws() })
    }
}

unsafe impl Send for Color {}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_COLOR: &'static objc::Class<Color>;
}

#[cfg(test)]
mod tests {
    use crate::ns;

    #[test]
    fn basics() {
        let black = ns::Color::with_white_alpha(0.0, 1.0);
        assert_eq!(black.white().unwrap(), 0.0);

        assert_eq!(black.alpha(), 1.0);

        black.red().expect_err("should be err");

        assert!(ns::Color::named(ns::str!(c"foo")).is_none());
    }

    #[test]
    fn hsb_components() {
        let color = ns::Color::with_hsba(0.5, 0.75, 0.25, 1.0);

        // Test hue component
        assert!((color.hue().unwrap() - 0.5f64).abs() < f64::EPSILON);

        // Test saturation component
        assert!((color.saturation().unwrap() - 0.75).abs() < f64::EPSILON);

        // Test brightness component
        assert!((color.brightness().unwrap() - 0.25).abs() < f64::EPSILON);

        // Test error case (color doesn't support HSB)
        let gray = ns::Color::with_white_alpha(0.5, 1.0);
        gray.hue()
            .expect_err("gray color shouldn't have hue component");
    }
}
