use crate::{arc, define_obj_type, ns, objc};

pub type Distance = f64;

define_obj_type!(
    #[doc(alias = "NLEmbedding")]
    pub Embedding(ns::Id),
    NL_EMBEDDING
);

#[link(name = "nl", kind = "static")]
extern "C" {
    static NL_EMBEDDING: &'static objc::Class<Embedding>;
}
