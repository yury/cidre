use crate::{cf, cg, define_obj_type, msg_send, ns::Id, define_cf_type};

define_obj_type!(Object(Id));

define_obj_type!(MachineReadableCodeObject(Object));
define_obj_type!(FaceObject(Object));
define_obj_type!(SalientObject(Object));
define_obj_type!(BodyObject(Object));
define_obj_type!(HumanBodyObject(BodyObject));
define_obj_type!(CatBodyObject(BodyObject));
define_obj_type!(DogBodyObject(BodyObject));

define_cf_type!(Type(cf::String));

impl Type {
    pub fn human_body() -> &'static Type {
        unsafe { AVMetadataObjectTypeHumanBody }
    }

    pub fn cat_body() -> &'static Type {
        unsafe { AVMetadataObjectTypeCatBody }
    }

    pub fn dog_body() -> &'static Type {
        unsafe { AVMetadataObjectTypeDogBody }
    }

    pub fn salient_object() -> &'static Type {
        unsafe { AVMetadataObjectTypeSalientObject }
    }

    pub fn face() -> &'static Type {
        unsafe { AVMetadataObjectTypeFace }
    }

    pub fn upce_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeUPCECode }
    }

    pub fn code_39_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeCode39Code }
    }

    pub fn code_39_mod_43_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeCode39Mod43Code }
    }

    pub fn ean_13_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeEAN13Code }
    }

    pub fn ean_8_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeEAN8Code }
    }

    pub fn code_93_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeCode93Code }
    }

    pub fn code_128_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeCode128Code }
    }

    pub fn pdf_417_code() -> &'static Type {
        unsafe { AVMetadataObjectTypePDF417Code }
    }

    pub fn qr_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeQRCode }
    }

    pub fn aztec_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeAztecCode }
    }
}

extern "C" {
    static AVMetadataObjectTypeHumanBody: &'static Type;
    static AVMetadataObjectTypeCatBody: &'static Type;
    static AVMetadataObjectTypeDogBody: &'static Type;
    static AVMetadataObjectTypeSalientObject: &'static Type;
    static AVMetadataObjectTypeFace: &'static Type;
    static AVMetadataObjectTypeUPCECode: &'static Type;
    static AVMetadataObjectTypeCode39Code: &'static Type;
    static AVMetadataObjectTypeCode39Mod43Code: &'static Type;
    static AVMetadataObjectTypeEAN13Code: &'static Type;
    static AVMetadataObjectTypeEAN8Code: &'static Type;
    static AVMetadataObjectTypeCode93Code: &'static Type;
    static AVMetadataObjectTypeCode128Code: &'static Type;
    static AVMetadataObjectTypePDF417Code: &'static Type;
    static AVMetadataObjectTypeQRCode: &'static Type;
    static AVMetadataObjectTypeAztecCode: &'static Type;

}

impl Object {
    pub fn bounds(&self) -> cg::Rect {
        msg_send!("common", self, sel_bounds)
    }
}
