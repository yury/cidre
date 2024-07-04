use crate::{arc, cm, define_cls, define_obj_type, ns, objc};

#[doc(alias = "SNTimeDurationConstraintType")]
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(isize)]
pub enum Type {
    #[doc(alias = "SNTimeDurationConstraintTypeEnumerated")]
    Enumerated = 1,

    #[doc(alias = "SNTimeDurationConstraintTypeRange")]
    TimeRange = 2,
}

define_obj_type!(
    #[doc(alias = "SNTimeDurationConstraint")]
    pub Constraint(ns::Id)
);

impl arc::A<Constraint> {
    #[objc::msg_send(initWithDurationRange:)]
    pub fn init_with_duration_range(self, duration_range: cm::TimeRange) -> arc::R<Constraint>;

    #[objc::msg_send(initWithEnumeratedDurations:)]
    pub fn init_with_enumerated_durations(
        self,
        durations: &ns::Array<ns::Value>,
    ) -> arc::R<Constraint>;
}

impl Constraint {
    define_cls!(SN_TIME_DURATION_CONSTRAINT);

    pub fn with_duration_range(duration_range: cm::TimeRange) -> arc::R<Self> {
        Self::alloc().init_with_duration_range(duration_range)
    }

    pub fn with_enumerated_durations(durations: &ns::Array<ns::Value>) -> arc::R<Self> {
        Self::alloc().init_with_enumerated_durations(durations)
    }

    #[objc::msg_send(type)]
    pub fn type_(&self) -> Type;

    #[objc::msg_send(enumeratedDurations)]
    pub fn enumerated_durations(&self) -> arc::R<ns::Array<ns::Value>>;

    #[objc::msg_send(durationRange)]
    pub fn duration_range(&self) -> cm::TimeRange;
}

#[link(name = "sn", kind = "static")]
extern "C" {
    static SN_TIME_DURATION_CONSTRAINT: &'static objc::Class<Constraint>;
}
