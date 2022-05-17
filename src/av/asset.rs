use crate::{define_obj_type, ns};

pub mod cache;
pub use cache::Cache as AssetCache;

define_obj_type!(Asset(ns::Id));
define_obj_type!(URLAsset(Asset));
define_obj_type!(FragmentedAsset(URLAsset));

define_obj_type!(FragmentedAssetMinder(ns::Id));

