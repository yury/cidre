use crate::{av::audio, define_obj_type};

mod effect;
pub use effect::Effect as UnitEffect;

mod eq;
pub use eq::Eq;
pub use eq::FilterParameters as EqFilterParameters;
pub use eq::FilterType as EqFilterType;
pub use eq::UnitEq;

mod time_effect;
pub use time_effect::TimeEffect as UnitTimeEffect;

define_obj_type!(Unit(audio::Node));
