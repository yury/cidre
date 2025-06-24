use crate::{arc, define_cls, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "MLModel")]
    pub Model(ns::Id)
);

impl Model {
    #[objc::available(macos = 10.14, ios = 12.0, watchos = 5.0, tvos = 12.0)]
    define_cls!(ML_MODEL);

    #[objc::msg_send(modelWithContentsOfURL:error:)]
    pub unsafe fn with_url_err<'ear>(
        url: &ns::Url,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<Self>>;

    pub fn with_url<'ear>(url: &ns::Url) -> ns::Result<'ear, arc::R<Self>> {
        ns::if_none(|err| unsafe { Self::with_url_err(url, err) })
    }
}

#[link(name = "ml", kind = "static")]
unsafe extern "C" {
    static ML_MODEL: &'static objc::Class<Model>;
}

#[cfg(test)]
mod tests {
    use crate::{ml, ns};

    #[test]
    fn basics() {
        let url = ns::Url::with_str("foo").unwrap();
        assert!(ml::Model::with_url(&url).is_err());

        // let url = ns::Url::with_fs_path_str(
        //     "/Users/yury/Projects/MLModels/nvidia_parakeet-v2/MelSpectrogram.mlmodelc",
        //     false,
        // );
        // let model = ml::Model::with_url(&url).unwrap();
        // println!("{model:?}");
    }
}
