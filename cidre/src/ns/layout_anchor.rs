use crate::{arc, cg, define_obj_type, ns, objc};

/// Adds the constraint factories shared by every anchor kind, between anchors
/// of the same kind.
macro_rules! impl_layout_anchor {
    ($Anchor:ident) => {
        impl $Anchor {
            /// `self == other`
            #[objc::msg_send(constraintEqualToAnchor:)]
            pub fn constraint_eq(&self, other: &Self) -> arc::R<ns::LayoutConstraint>;

            /// `self >= other`
            #[objc::msg_send(constraintGreaterThanOrEqualToAnchor:)]
            pub fn constraint_ge(&self, other: &Self) -> arc::R<ns::LayoutConstraint>;

            /// `self <= other`
            #[objc::msg_send(constraintLessThanOrEqualToAnchor:)]
            pub fn constraint_le(&self, other: &Self) -> arc::R<ns::LayoutConstraint>;

            /// `self == other + constant`
            #[objc::msg_send(constraintEqualToAnchor:constant:)]
            pub fn constraint_eq_c(
                &self,
                other: &Self,
                constant: cg::Float,
            ) -> arc::R<ns::LayoutConstraint>;

            /// `self >= other + constant`
            #[objc::msg_send(constraintGreaterThanOrEqualToAnchor:constant:)]
            pub fn constraint_ge_c(
                &self,
                other: &Self,
                constant: cg::Float,
            ) -> arc::R<ns::LayoutConstraint>;

            /// `self <= other + constant`
            #[objc::msg_send(constraintLessThanOrEqualToAnchor:constant:)]
            pub fn constraint_le_c(
                &self,
                other: &Self,
                constant: cg::Float,
            ) -> arc::R<ns::LayoutConstraint>;
        }
    };
}

define_obj_type!(
    #[doc(alias = "NSLayoutXAxisAnchor")]
    pub LayoutXAxisAnchor(ns::Id)
);

impl_layout_anchor!(LayoutXAxisAnchor);

impl LayoutXAxisAnchor {
    /// `self == other + system spacing * multiplier`, the system spacing being
    /// the appropriate distance after `other`'s view.
    #[objc::msg_send(constraintEqualToSystemSpacingAfterAnchor:multiplier:)]
    #[objc::available(macos = 11.0, ios = 11.0, tvos = 11.0)]
    pub fn constraint_eq_system_spacing_after(
        &self,
        other: &Self,
        multiplier: cg::Float,
    ) -> arc::R<ns::LayoutConstraint>;

    #[objc::msg_send(constraintGreaterThanOrEqualToSystemSpacingAfterAnchor:multiplier:)]
    #[objc::available(macos = 11.0, ios = 11.0, tvos = 11.0)]
    pub fn constraint_ge_system_spacing_after(
        &self,
        other: &Self,
        multiplier: cg::Float,
    ) -> arc::R<ns::LayoutConstraint>;

    #[objc::msg_send(constraintLessThanOrEqualToSystemSpacingAfterAnchor:multiplier:)]
    #[objc::available(macos = 11.0, ios = 11.0, tvos = 11.0)]
    pub fn constraint_le_system_spacing_after(
        &self,
        other: &Self,
        multiplier: cg::Float,
    ) -> arc::R<ns::LayoutConstraint>;
}

define_obj_type!(
    #[doc(alias = "NSLayoutYAxisAnchor")]
    pub LayoutYAxisAnchor(ns::Id)
);

impl_layout_anchor!(LayoutYAxisAnchor);

impl LayoutYAxisAnchor {
    /// `self == other + system spacing * multiplier`, the system spacing being
    /// the appropriate distance below `other`'s view.
    #[objc::msg_send(constraintEqualToSystemSpacingBelowAnchor:multiplier:)]
    #[objc::available(macos = 11.0, ios = 11.0, tvos = 11.0)]
    pub fn constraint_eq_system_spacing_below(
        &self,
        other: &Self,
        multiplier: cg::Float,
    ) -> arc::R<ns::LayoutConstraint>;

    #[objc::msg_send(constraintGreaterThanOrEqualToSystemSpacingBelowAnchor:multiplier:)]
    #[objc::available(macos = 11.0, ios = 11.0, tvos = 11.0)]
    pub fn constraint_ge_system_spacing_below(
        &self,
        other: &Self,
        multiplier: cg::Float,
    ) -> arc::R<ns::LayoutConstraint>;

    #[objc::msg_send(constraintLessThanOrEqualToSystemSpacingBelowAnchor:multiplier:)]
    #[objc::available(macos = 11.0, ios = 11.0, tvos = 11.0)]
    pub fn constraint_le_system_spacing_below(
        &self,
        other: &Self,
        multiplier: cg::Float,
    ) -> arc::R<ns::LayoutConstraint>;
}

define_obj_type!(
    #[doc(alias = "NSLayoutDimension")]
    pub LayoutDimension(ns::Id)
);

impl_layout_anchor!(LayoutDimension);

impl LayoutDimension {
    /// `self == constant`
    #[objc::msg_send(constraintEqualToConstant:)]
    pub fn constraint_eq_const(&self, constant: cg::Float) -> arc::R<ns::LayoutConstraint>;

    /// `self >= constant`
    #[objc::msg_send(constraintGreaterThanOrEqualToConstant:)]
    pub fn constraint_ge_const(&self, constant: cg::Float) -> arc::R<ns::LayoutConstraint>;

    /// `self <= constant`
    #[objc::msg_send(constraintLessThanOrEqualToConstant:)]
    pub fn constraint_le_const(&self, constant: cg::Float) -> arc::R<ns::LayoutConstraint>;

    /// `self == other * multiplier`
    #[objc::msg_send(constraintEqualToAnchor:multiplier:)]
    pub fn constraint_eq_m(
        &self,
        other: &Self,
        multiplier: cg::Float,
    ) -> arc::R<ns::LayoutConstraint>;

    #[objc::msg_send(constraintGreaterThanOrEqualToAnchor:multiplier:)]
    pub fn constraint_ge_m(
        &self,
        other: &Self,
        multiplier: cg::Float,
    ) -> arc::R<ns::LayoutConstraint>;

    #[objc::msg_send(constraintLessThanOrEqualToAnchor:multiplier:)]
    pub fn constraint_le_m(
        &self,
        other: &Self,
        multiplier: cg::Float,
    ) -> arc::R<ns::LayoutConstraint>;

    /// `self == other * multiplier + constant`
    #[objc::msg_send(constraintEqualToAnchor:multiplier:constant:)]
    pub fn constraint_eq_mc(
        &self,
        other: &Self,
        multiplier: cg::Float,
        constant: cg::Float,
    ) -> arc::R<ns::LayoutConstraint>;

    #[objc::msg_send(constraintGreaterThanOrEqualToAnchor:multiplier:constant:)]
    pub fn constraint_ge_mc(
        &self,
        other: &Self,
        multiplier: cg::Float,
        constant: cg::Float,
    ) -> arc::R<ns::LayoutConstraint>;

    #[objc::msg_send(constraintLessThanOrEqualToAnchor:multiplier:constant:)]
    pub fn constraint_le_mc(
        &self,
        other: &Self,
        multiplier: cg::Float,
        constant: cg::Float,
    ) -> arc::R<ns::LayoutConstraint>;
}

/// Adds the anchor accessors every view and layout guide has.
macro_rules! impl_layout_anchors {
    ($Type:ty) => {
        impl $Type {
            #[$crate::objc::msg_send(leadingAnchor)]
            pub fn leading_anchor(&self) -> crate::arc::R<crate::ns::LayoutXAxisAnchor>;

            #[$crate::objc::msg_send(trailingAnchor)]
            pub fn trailing_anchor(&self) -> crate::arc::R<crate::ns::LayoutXAxisAnchor>;

            #[$crate::objc::msg_send(leftAnchor)]
            pub fn left_anchor(&self) -> crate::arc::R<crate::ns::LayoutXAxisAnchor>;

            #[$crate::objc::msg_send(rightAnchor)]
            pub fn right_anchor(&self) -> crate::arc::R<crate::ns::LayoutXAxisAnchor>;

            #[$crate::objc::msg_send(topAnchor)]
            pub fn top_anchor(&self) -> crate::arc::R<crate::ns::LayoutYAxisAnchor>;

            #[$crate::objc::msg_send(bottomAnchor)]
            pub fn bottom_anchor(&self) -> crate::arc::R<crate::ns::LayoutYAxisAnchor>;

            #[$crate::objc::msg_send(widthAnchor)]
            pub fn width_anchor(&self) -> crate::arc::R<crate::ns::LayoutDimension>;

            #[$crate::objc::msg_send(heightAnchor)]
            pub fn height_anchor(&self) -> crate::arc::R<crate::ns::LayoutDimension>;

            #[$crate::objc::msg_send(centerXAnchor)]
            pub fn center_x_anchor(&self) -> crate::arc::R<crate::ns::LayoutXAxisAnchor>;

            #[$crate::objc::msg_send(centerYAnchor)]
            pub fn center_y_anchor(&self) -> crate::arc::R<crate::ns::LayoutYAxisAnchor>;
        }
    };
}

/// Adds the baseline anchors views have.
macro_rules! impl_baseline_anchors {
    ($Type:ty) => {
        impl $Type {
            #[$crate::objc::msg_send(firstBaselineAnchor)]
            pub fn first_baseline_anchor(&self) -> crate::arc::R<crate::ns::LayoutYAxisAnchor>;

            #[$crate::objc::msg_send(lastBaselineAnchor)]
            pub fn last_baseline_anchor(&self) -> crate::arc::R<crate::ns::LayoutYAxisAnchor>;
        }
    };
}

pub(crate) use impl_baseline_anchors;
pub(crate) use impl_layout_anchors;

#[cfg(all(test, target_os = "macos", feature = "app"))]
mod tests {
    use crate::{ns, objc::Obj};

    #[test]
    fn anchors_and_constraints() {
        let mut container = ns::View::with_frame(ns::Rect::new(0.0, 0.0, 300.0, 200.0));
        let mut child = ns::View::with_frame(ns::Rect::new(0.0, 0.0, 10.0, 10.0));
        child.set_translates_autoresizing_mask_into_constraints(false);
        container.add_subview(&child);

        let leading = child
            .leading_anchor()
            .constraint_eq_c(&container.leading_anchor(), 20.0);
        let top = child.top_anchor().constraint_eq(&container.top_anchor());
        let width = child.width_anchor().constraint_eq_const(100.0);
        let height = child
            .height_anchor()
            .constraint_eq_mc(&container.height_anchor(), 0.5, -10.0);
        ns::LayoutConstraint::activate(&ns::Array::from_slice(&[
            &*leading, &*top, &*width, &*height,
        ]));
        assert!(leading.is_active());
        assert_eq!(leading.constant(), 20.0);
        assert_eq!(leading.relation(), ns::LayoutRelation::Equal);
        assert_eq!(height.multiplier(), 0.5);
        assert!(leading.first_item().unwrap().is_equal(child.as_id_ref()));

        // Solving happens on the main thread only, so the layout itself is not
        // exercised here.
        ns::LayoutConstraint::deactivate(&ns::Array::from_slice(&[&*width]));
        assert!(!width.is_active());
    }
}
