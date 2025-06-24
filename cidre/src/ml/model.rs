use crate::{arc, define_cls, define_obj_type, ml, ns, objc};

define_obj_type!(
    #[doc(alias = "MLModel")]
    pub Model(ns::Id)
);

impl Model {
    #[objc::available(macos = 10.14, ios = 12.0, watchos = 5.0, tvos = 12.0)]
    define_cls!(ML_MODEL);

    /// A model holds a description of its required inputs and expected outputs.
    #[objc::msg_send(modelDescription)]
    pub fn model_desc(&self) -> arc::R<ml::ModelDesc>;

    #[objc::msg_send(configuration)]
    pub fn cfg(&self) -> arc::R<ml::ModelCfg>;

    #[objc::msg_send(modelWithContentsOfURL:error:)]
    pub unsafe fn with_url_err<'ear>(
        url: &ns::Url,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<Self>>;

    pub fn with_url<'ear>(url: &ns::Url) -> ns::Result<'ear, arc::R<Self>> {
        ns::if_none(|err| unsafe { Self::with_url_err(url, err) })
    }

    #[objc::msg_send(modelWithContentsOfURL:configuration:error:)]
    pub unsafe fn with_cfg_err<'ear>(
        url: &ns::Url,
        cfg: &ml::ModelCfg,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<Self>>;

    pub fn with_cfg<'ear>(url: &ns::Url, cfg: &ml::ModelCfg) -> ns::Result<'ear, arc::R<Self>> {
        ns::if_none(|err| unsafe { Self::with_cfg_err(url, cfg, err) })
    }

    #[objc::msg_send(predictionFromFeatures:error:)]
    pub unsafe fn prediction_from_features_err<'ear, F: ml::FeatureProvider>(
        &self,
        input: &F,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ml::AnyFeatureProvider>>;

    pub fn prediction_from_features<'ear, F: ml::FeatureProvider>(
        &self,
        input: &F,
    ) -> ns::Result<'ear, arc::R<ml::AnyFeatureProvider>> {
        ns::if_none(|err| unsafe { self.prediction_from_features_err(input, err) })
    }

    #[objc::msg_send(predictionsFromBatch:error:)]
    pub unsafe fn predictions_from_batch_err<'ear, P: ml::BatchProvider>(
        &self,
        input_batch: &P,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<ml::AnyBatchProvider>>;

    pub unsafe fn predictions_from_batch<'ear, P: ml::BatchProvider>(
        &self,
        input_batch: &P,
    ) -> ns::Result<'ear, arc::R<ml::AnyBatchProvider>> {
        ns::if_none(|err| unsafe { self.predictions_from_batch_err(input_batch, err) })
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
        //     "/Users/yury/Projects/MLModels/nvidia_parakeet-v2/AudioEncoder.mlmodelc",
        //     false,
        // );
        // let model = ml::Model::with_url(&url).unwrap();
        // println!("{model:?}");
        // println!("{:?}", ml::Model::available_compute_devices());
        // println!("{:?}", model.cfg());
    }
}
