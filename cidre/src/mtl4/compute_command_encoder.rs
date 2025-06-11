use crate::{define_obj_type, mtl4};

define_obj_type!(
    #[doc(alias = "MTL4ComputeCommandEncoder")]
    pub ComputeCmdEncoder(mtl4::CmdEncoder)
);
