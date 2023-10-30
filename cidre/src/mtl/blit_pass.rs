use crate::{arc, define_cls_init, define_obj_type, ns, objc};

define_obj_type!(SampleBufAttachDesc(ns::Id));
define_obj_type!(SampleBufAttachDescArray(ns::Id));

define_obj_type!(Desc(ns::Id));

define_cls_init!(Desc, MTL_BLIT_PASS_DESCRIPTOR);

/// Represents a collection of attachments to be used to create a concrete blit command encoder
impl Desc {
    /// An array of sample buffers and associated sample indices.
    #[objc::msg_send(sampleBufferAttachments)]
    pub fn sample_buf_attaches(&self) -> &SampleBufAttachDescArray;

    /// An array of sample buffers and associated sample indices.
    #[objc::msg_send(sampleBufferAttachments)]
    pub fn sample_buf_attaches_mut(&mut self) -> &mut SampleBufAttachDescArray;
}

#[link(name = "mtl", kind = "static")]
extern "C" {
    static MTL_BLIT_PASS_DESCRIPTOR: &'static objc::Class<Desc>;
}

#[cfg(test)]
mod tests {
    use crate::mtl;

    #[test]
    fn basics() {
        let mut bpd = mtl::BlitPassDesc::new();
        let _attachments = bpd.sample_buf_attaches_mut();
    }
}
