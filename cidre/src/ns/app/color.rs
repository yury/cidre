use crate::{arc, cg, define_obj_type, ns, objc};

#[doc(alias = "NSColorSystemEffect")]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(isize)]
pub enum ColorSysEffect {
    None,
    Pressed,
    DeepPressed,
    Disabled,
    Rollover,
}

define_obj_type!(
    #[doc(alias = "NSColor")]
    pub Color(ns::Id),
    NS_COLOR
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
    pub unsafe fn white_component_throws(&self) -> cg::Float;

    pub fn white_component<'ar>(&self) -> Result<cg::Float, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.white_component_throws() })
    }

    #[objc::msg_send(alphaComponent)]
    pub fn alpha_component(&self) -> cg::Float;

    #[objc::msg_send(redComponent)]
    pub unsafe fn red_component_throws(&self) -> cg::Float;

    #[objc::msg_send(greenComponent)]
    pub unsafe fn green_component_throws(&self) -> cg::Float;

    #[objc::msg_send(blueComponent)]
    pub unsafe fn blue_component_throws(&self) -> cg::Float;

    pub fn red_component<'ar>(&self) -> Result<cg::Float, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.red_component_throws() })
    }

    pub fn green_component<'ar>(&self) -> Result<cg::Float, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.green_component_throws() })
    }

    pub fn blue_component<'ar>(&self) -> Result<cg::Float, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.blue_component_throws() })
    }

    #[objc::msg_send(hueComponent)]
    pub unsafe fn hue_component_throws(&self) -> cg::Float;

    #[objc::msg_send(saturationComponent)]
    pub unsafe fn saturation_component_throws(&self) -> cg::Float;

    #[objc::msg_send(brightnessComponent)]
    pub unsafe fn brightness_component_throws(&self) -> cg::Float;

    pub fn hue_component<'ar>(&self) -> Result<cg::Float, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.hue_component_throws() })
    }

    pub fn saturation_component<'ar>(&self) -> Result<cg::Float, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.saturation_component_throws() })
    }

    pub fn brightness_component<'ar>(&self) -> Result<cg::Float, &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.brightness_component_throws() })
    }

    #[objc::msg_send(colorSpace)]
    pub unsafe fn color_space_throws(&self) -> arc::R<ns::ColorSpace>;

    pub fn color_space<'ear>(&self) -> ns::ExResult<'ear, arc::R<ns::ColorSpace>> {
        ns::try_catch(|| unsafe { self.color_space_throws() })
    }

    #[objc::msg_send(colorWithRed:green:blue:alpha:exposure:)]
    #[objc::available(macos = 26.0)]
    pub fn with_exposure(
        r: cg::Float,
        g: cg::Float,
        b: cg::Float,
        a: cg::Float,
        exposure: cg::Float,
    ) -> arc::R<Self>;

    #[objc::msg_send(colorWithRed:green:blue:alpha:linearExposure:)]
    #[objc::available(macos = 26.0)]
    pub fn with_linear_exposure(
        r: cg::Float,
        g: cg::Float,
        b: cg::Float,
        a: cg::Float,
        exposure: cg::Float,
    ) -> arc::R<Self>;

    /// Reinterpret the color by applying a new `content_headroom` without changing the color components.
    /// Changing the `content_headroom` redefines the color relative to a different peak white, changing its behavior
    /// under tone mapping and the result of calling `sdr`. The new color will have a `content_headroom` >= 1.0.
    #[objc::msg_send(colorByApplyingContentHeadroom:)]
    #[objc::available(macos = 26.0)]
    pub fn applying_content_headroom(&self, content_headroom: cg::Float) -> arc::R<Self>;

    /// The linear brightness multiplier that was applied when generating this color.
    /// Colors created with an exposure by ui::Color create cg::Colors that are tagged with a contentHeadroom value.
    /// While cg::Colors created without a contentHeadroom tag will return 0 from cg::Color::content_headroom, ui::Colors generated in a similar
    /// fashion return a linear_exposure of 1.0.
    #[objc::msg_send(linearExposure)]
    #[objc::available(macos = 26.0)]
    pub fn linear_exposure(&self) -> cg::Float;

    #[objc::msg_send(standardDynamicRangeColor)]
    #[objc::available(macos = 26.0)]
    pub fn sdr(&self) -> arc::R<Self>;

    #[cfg(feature = "cg")]
    #[objc::msg_send(CGColor)]
    pub fn cg_color(&self) -> Option<&crate::cg::Color>;
}

impl Color {
    #[objc::msg_send(blackColor)]
    pub fn black() -> arc::R<Self>;

    #[objc::msg_send(darkGrayColor)]
    pub fn dark_gray() -> arc::R<Self>;

    #[objc::msg_send(lightGrayColor)]
    pub fn light_gray() -> arc::R<Self>;

    #[objc::msg_send(whiteColor)]
    pub fn white() -> arc::R<Self>;

    #[objc::msg_send(grayColor)]
    pub fn gray() -> arc::R<Self>;

    #[objc::msg_send(redColor)]
    pub fn red() -> arc::R<Self>;

    #[objc::msg_send(greenColor)]
    pub fn green() -> arc::R<Self>;

    #[objc::msg_send(blueColor)]
    pub fn blue() -> arc::R<Self>;

    #[objc::msg_send(cyanColor)]
    pub fn cyan() -> arc::R<Self>;

    #[objc::msg_send(yellowColor)]
    pub fn yellow() -> arc::R<Self>;

    #[objc::msg_send(magentaColor)]
    pub fn magenta() -> arc::R<Self>;

    #[objc::msg_send(orangeColor)]
    pub fn orange() -> arc::R<Self>;

    #[objc::msg_send(purpleColor)]
    pub fn purple() -> arc::R<Self>;

    #[objc::msg_send(brownColor)]
    pub fn brown() -> arc::R<Self>;

    #[objc::msg_send(clearColor)]
    pub fn clear() -> arc::R<Self>;
}

/// Foreground Colors
impl Color {
    /// Foreground color for static text and related elements
    #[objc::msg_send(labelColor)]
    pub fn label() -> arc::R<Self>;

    /// Foreground color for secondary static text and related elements
    #[objc::msg_send(secondaryLabelColor)]
    pub fn secondary_label() -> arc::R<Self>;

    /// Foreground color for disabled static text and related elements
    #[objc::msg_send(tertiaryLabelColor)]
    pub fn tertiary_label() -> arc::R<Self>;

    /// Foreground color for large secondary or disabled static text, separators, large glyphs/icons, etc
    #[objc::msg_send(quaternaryLabelColor)]
    pub fn quaternary_label() -> arc::R<Self>;

    /// Used for large scale images or subtle decorative elements; not for general foreground content.
    #[objc::msg_send(quinaryLabelColor)]
    pub fn quinary_label() -> arc::R<Self>;

    /// Foreground color for standard system links
    #[objc::msg_send(linkColor)]
    pub fn link() -> arc::R<Self>;

    /// Foreground color for placeholder text in controls or text views
    #[objc::msg_send(placeholderTextColor)]
    pub fn placeholder_text() -> arc::R<Self>;

    /// Foreground color on window chrome
    #[objc::msg_send(windowFrameTextColor)]
    pub fn window_frame_text() -> arc::R<Self>;

    /// Foreground color inside selected menu items. Equivalent to +labelColor in a NSBackgroundStyleEmphasized context.
    #[objc::msg_send(selectedMenuItemTextColor)]
    pub fn selected_menu_item_text() -> arc::R<Self>;

    /// Foreground color inside emphasized and selected content: table views rows, collection views, etc.
    /// Equivalent to +labelColor in a ns::BackgroundStyleEmphasized context.
    #[objc::msg_send(alternateSelectedControlTextColor)]
    pub fn alternate_selected_control_text() -> arc::R<Self>;

    /// Foreground color for header cells in Table/OutlineView
    #[objc::msg_send(headerTextColor)]
    pub fn header_text() -> arc::R<Self>;

    /// Foreground used for separators between sections of content. Not appropriate for use as split view or window chrome dividers.
    #[objc::msg_send(separatorColor)]
    pub fn separator() -> arc::R<Self>;

    /// Grids in controls
    #[objc::msg_send(gridColor)]
    pub fn grid() -> arc::R<Self>;
}

/*
/* Background Colors */


@property (class, strong, readonly) NSColor *findHighlightColor API_AVAILABLE(macos(10.13));   // Background color of find indicators: the bubbles that show inline search result hits

 */

/// Background Colors
impl Color {
    /// Background color for windows. This should not be used for drawing, and NSVisualEffectMaterialWindowBackground should be used instead.
    #[objc::msg_send(windowBackgroundColor)]
    pub fn window_bg() -> arc::R<Self>;

    /// Background areas revealed behind documents. This should not be used for drawing, and NSVisualEffectMaterialUnderPageBackground should be used instead.
    #[objc::msg_send(underPageBackgroundColor)]
    pub fn under_page_bg() -> arc::R<Self>;

    /// Background for content areas: scroll views, table views, collection views. This should not be used for drawing, and NSVisualEffectMaterialContentBackground should be used instead.
    #[objc::msg_send(controlBackgroundColor)]
    pub fn control_bg() -> arc::R<Self>;

    /// The background color of selected and emphasized (focused) content: table views rows, collection views, etc. Alias for +alternateSelectedControlColor
    #[objc::msg_send(selectedContentBackgroundColor)]
    pub fn selected_content_bg() -> arc::R<Self>;

    /// The background color of selected and unemphasized content: table views rows, collection views, etc. Alias for +secondarySelectedControlColor
    #[objc::msg_send(unemphasizedSelectedContentBackgroundColor)]
    pub fn unemphasized_selected_content_bg() -> arc::R<Self>;

    /// The background colors for alternating content items: such as table view rows, collection view items. Alias for +controlAlternatingRowBackgroundColors
    #[objc::msg_send(alternatingContentBackgroundColors)]
    pub fn alternating_content_bg_colors() -> arc::R<ns::Array<Self>>;
}

/// Text Colors
impl Color {
    /// Document text
    #[objc::msg_send(textColor)]
    pub fn text() -> arc::R<Self>;

    /// Document text background
    #[objc::msg_send(textBackgroundColor)]
    pub fn text_bg() -> arc::R<Self>;

    #[objc::msg_send(textInsertionPointColor)]
    pub fn text_insertion_point() -> arc::R<Self>;

    /// Selected document text
    #[objc::msg_send(selectedTextColor)]
    pub fn selected_text() -> arc::R<Self>;

    /// Selected document text background
    #[objc::msg_send(selectedTextBackgroundColor)]
    pub fn selected_text_bg() -> arc::R<Self>;
    /// The background for unemphasized text selection (e.g. when the associated control/view/window is not key)
    #[objc::msg_send(unemphasizedSelectedTextBackgroundColor)]
    pub fn unemphasized_selected_text_bg() -> arc::R<Self>;
    /// The foreground for unemphasized text selection (e.g. when the associated control/view/window is not key)
    #[objc::msg_send(unemphasizedSelectedTextColor)]
    pub fn unemphasized_selected_text() -> arc::R<Self>;
}

/// Control Colors
impl Color {
    /// Approximate background for the color of control faces (such as buttons).
    #[objc::msg_send(controlColor)]
    pub fn control() -> arc::R<Self>;

    /// Text on controls
    #[objc::msg_send(controlTextColor)]
    pub fn control_text() -> arc::R<Self>;

    /// Control face for selected controls
    #[objc::msg_send(selectedControlColor)]
    pub fn selected_control() -> arc::R<Self>;

    /// Text on selected controls
    #[objc::msg_send(selectedControlTextColor)]
    pub fn selected_control_text() -> arc::R<Self>;

    /// Text on disabled controls
    #[objc::msg_send(disabledControlTextColor)]
    pub fn disabled_control_text() -> arc::R<Self>;

    /// Keyboard focus ring around controls
    #[objc::msg_send(keyboardFocusIndicatorColor)]
    pub fn keyboard_focus_indicator() -> arc::R<Self>;

    /// Patterned background color for use in NSScrubber
    #[objc::msg_send(scrubberTexturedBackgroundColor)]
    pub fn scrubber_textured_bg() -> arc::R<Self>;
}

impl Color {
    #[objc::msg_send(systemRedColor)]
    pub fn sys_red() -> arc::R<Self>;

    #[objc::msg_send(systemGreenColor)]
    pub fn sys_green() -> arc::R<Self>;

    #[objc::msg_send(systemBlueColor)]
    pub fn sys_blue() -> arc::R<Self>;

    #[objc::msg_send(systemOrangeColor)]
    pub fn sys_orange() -> arc::R<Self>;

    #[objc::msg_send(systemYellowColor)]
    pub fn sys_yellow() -> arc::R<Self>;

    #[objc::msg_send(systemBrownColor)]
    pub fn sys_brown() -> arc::R<Self>;

    #[objc::msg_send(systemPinkColor)]
    pub fn sys_pink() -> arc::R<Self>;

    #[objc::msg_send(systemPurpleColor)]
    pub fn sys_purple() -> arc::R<Self>;

    #[objc::msg_send(systemGrayColor)]
    pub fn sys_gray() -> arc::R<Self>;

    #[objc::msg_send(systemTealColor)]
    pub fn sys_teal() -> arc::R<Self>;

    #[objc::msg_send(systemIndigoColor)]
    pub fn sys_indigo() -> arc::R<Self>;

    #[objc::msg_send(systemMintColor)]
    pub fn sys_mint() -> arc::R<Self>;

    #[objc::msg_send(systemCyanColor)]
    pub fn sys_cyan() -> arc::R<Self>;
}

/// Fill colors for UI elements.
///
/// These are meant to be used over the background colors, since their alpha component is less than 1.
impl Color {
    /// systemFillColor is appropriate for filling thin shapes, such as the track of a slider.
    #[objc::msg_send(systemFillColor)]
    pub fn sys_fill() -> arc::R<Self>;
    /// secondarySystemFillColor is appropriate for filling small-size shapes, such as the backing of a progress indicator.
    #[objc::msg_send(secondarySystemFillColor)]
    pub fn sys_secondary_fill() -> arc::R<Self>;
    /// tertiarySystemFillColor is appropriate for filling medium-size shapes,  such as the backing of a switch.
    #[objc::msg_send(tertiarySystemFillColor)]
    pub fn sys_tertiary_fill() -> arc::R<Self>;
    /// quaternarySystemFillColor is appropriate for filling large areas, such as a group box or tab pane.
    #[objc::msg_send(quaternarySystemFillColor)]
    pub fn sys_quaternary_fill() -> arc::R<Self>;
    /// quinarySystemFillColor is appropriate for filling large areas that require subtle emphasis, such as content of a form..
    #[objc::msg_send(quinarySystemFillColor)]
    pub fn sys_quinary_fill() -> arc::R<Self>;
    /// A dynamic color that reflects the user's current preferred accent color. This color automatically updates when the accent color preference changes. Do not make assumptions about the color space of this color, which may change across releases.
    #[objc::msg_send(controlAccentColor)]
    pub fn control_accent() -> arc::R<Self>;
    /// The current system control tint.
    #[objc::msg_send(currentControlTint)]
    pub fn current_control_tint() -> ns::ControlTint;

    /// Highlight color for UI elements (this is abstract and defines the color all highlights tend toward)
    #[objc::msg_send(highlightColor)]
    pub fn highlight() -> arc::R<Self>;
    /// Shadow color for UI elements (this is abstract and defines the color all shadows tend toward)
    #[objc::msg_send(shadowColor)]
    pub fn shadow() -> arc::R<Self>;
    /// val = 0 => receiver, val = 1 => highlightColor
    #[objc::msg_send(highlightWithLevel:)]
    pub fn highlight_with_level(&self, val: cg::Float) -> Option<arc::R<Self>>;
    /// val = 0 => receiver, val = 1 => shadowColor
    #[objc::msg_send(shadowWithLevel:)]
    pub fn shadow_with_level(&self, val: cg::Float) -> Option<arc::R<Self>>;

    //     /// Returns a color representing the base color with a system defined effect applied to it. This color is safe to create before draw time, as the resolution of the final color only happens when being `-set`, retrieving its `CGColor`, resolving with `-colorWithType:`, etc. The return color type is `.named`.
    // - (NSColor *)colorWithSystemEffect:(NSColorSystemEffect)systemEffect API_AVAILABLE(macos(10.14));

    /// Returns a color representing the base color with a system defined effect applied to it. This color is safe to create before draw time, as the resolution of the final color only happens when being `-set`, retrieving its `CGColor`, resolving with `-colorWithType:`, etc. The return color type is `.named`.
    #[objc::msg_send(colorWithSystemEffect:)]
    pub fn color_with_sys_effect(&self, sys_effect: ns::ColorSysEffect) -> arc::R<Self>;
}

impl Color {
    /// Set the color: Sets the fill and stroke colors in the current drawing context. If the color doesn't know about alpha, it's set to 1.0.
    /// Should be implemented by subclassers.
    #[objc::msg_send(set)]
    pub fn set(&self);

    /// Set the fill or stroke colors individually. These should be implemented by subclassers.
    #[objc::msg_send(setFill)]
    pub fn set_fill(&self);

    /// Set the fill or stroke colors individually. These should be implemented by subclassers.
    #[objc::msg_send(setStroke)]
    pub fn set_stroke(&self);
}

impl Color {
    /// Blend using the NSCalibratedRGB color space. Both colors are converted into the calibrated RGB color space, and they are blended by taking fraction of color and 1 - fraction of the receiver. The result is in the calibrated RGB color space. If the colors cannot be converted into the calibrated RGB color space the blending fails and nil is returned.
    #[objc::msg_send(blendedColorWithFraction:ofColor:)]
    pub fn blended_color_with_fraction(
        &self,
        fraction: cg::Float,
        of_color: &Self,
    ) -> Option<arc::R<Self>>;
}

unsafe impl Send for Color {}

#[link(name = "app", kind = "static")]
unsafe extern "C" {
    static NS_COLOR: &'static objc::Class<Color>;
}

#[cfg(test)]
mod tests {
    use crate::{api, ns};

    #[test]
    fn basics() {
        let black = ns::Color::with_white_alpha(0.0, 1.0);
        assert_eq!(black.white_component().unwrap(), 0.0);

        assert_eq!(black.alpha_component(), 1.0);

        black.red_component().expect_err("should be err");

        assert!(ns::Color::named(ns::str!(c"foo")).is_none());
    }

    #[test]
    fn hsb_components() {
        let color = ns::Color::with_hsba(0.5, 0.75, 0.25, 1.0);

        // Test hue component
        assert!((color.hue_component().unwrap() - 0.5f64).abs() < f64::EPSILON);

        // Test saturation component
        assert!((color.saturation_component().unwrap() - 0.75).abs() < f64::EPSILON);

        // Test brightness component
        assert!((color.brightness_component().unwrap() - 0.25).abs() < f64::EPSILON);

        // Test error case (color doesn't support HSB)
        let gray = ns::Color::with_white_alpha(0.5, 1.0);
        gray.hue_component()
            .expect_err("gray color shouldn't have hue component");
    }

    #[test]
    #[allow(unused_unsafe)]
    fn hdr() {
        let red = ns::Color::sys_red();
        if api::macos_available("26.0") {
            unsafe {
                // we can't create hdr colors from system colors
                let bright_red = red.applying_content_headroom(2.0);
                assert_eq!(bright_red.linear_exposure(), 1.0);
                assert!(bright_red.color_space().is_err());

                let bright_red = ns::Color::with_linear_exposure(1.0, 0.0, 0.0, 1.0, 2.0);
                assert_eq!(bright_red.linear_exposure(), 2.0);
                assert!(bright_red.color_space().is_ok());

                let red = bright_red.sdr();
                assert_eq!(red.linear_exposure(), 1.0);
            }
        }
    }
}
