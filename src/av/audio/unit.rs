use crate::{av::audio, define_obj_type};

mod effect;
pub use effect::Effect as UnitEffect;

mod eq;
pub use eq::FilterType as EqFilterType;
pub use eq::UnitEq;

define_obj_type!(Unit(audio::Node));
