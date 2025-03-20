#[doc(alias = "VTRegisterProfessionalVideoWorkflowVideoDecoders")]
pub fn register_decoders() {
    unsafe { VTRegisterProfessionalVideoWorkflowVideoDecoders() }
}

#[doc(alias = "VTRegisterProfessionalVideoWorkflowVideoEncoders")]
pub fn register_encoders() {
    unsafe { VTRegisterProfessionalVideoWorkflowVideoEncoders() }
}

#[link(name = "VideoToolbox", kind = "framework")]
unsafe extern "C" {
    fn VTRegisterProfessionalVideoWorkflowVideoDecoders();
    fn VTRegisterProfessionalVideoWorkflowVideoEncoders();
}

#[cfg(test)]
mod tests {
    use crate::vt;
    #[test]
    fn basics() {
        let list = vt::video_encoder_list::copy().unwrap();
        let a = list.len();
        // println!("{list:?}");
        vt::professional_video_workflow::register_encoders();
        let list = vt::video_encoder_list::copy().unwrap();
        // println!("{list:?}");
        let b = list.len();

        assert!(a < b);
    }
}
