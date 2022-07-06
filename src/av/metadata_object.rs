use crate::{cf, cg, define_cf_type, define_obj_type, msg_send, ns::Id};

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

    /// objects generated from UPC-E codes return this constant as their type.
    pub fn upc_e_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeUPCECode }
    }

    /// objects generated from Code 39 codes return this constant as their type.
    pub fn code_39_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeCode39Code }
    }

    /// objects generated from Code 39 mod 43 codes return this constant as their type.
    pub fn code_39_mod_43_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeCode39Mod43Code }
    }

    /// objects generated from EAN-13 (including UPC-A) codes return this constant as their type.
    pub fn ean_13_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeEAN13Code }
    }

    /// objects generated from EAN-8 codes return this constant as their type.
    pub fn ean_8_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeEAN8Code }
    }

    /// objects generated from Code 93 codes return this constant as their type.
    pub fn code_93_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeCode93Code }
    }

    /// objects generated from Code 128 codes return this constant as their type.
    pub fn code_128_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeCode128Code }
    }

    /// objects generated from PDF417 codes return this constant as their type
    pub fn pdf_417_code() -> &'static Type {
        unsafe { AVMetadataObjectTypePDF417Code }
    }

    ///  objects generated from QR codes return this constant as their type.
    pub fn qr_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeQRCode }
    }

    /// objects generated from Aztec codes return this constant as their type.
    pub fn aztec_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeAztecCode }
    }

    /// objects generated from Interleaved 2 of 5 codes return this constant as their type.
    pub fn interleaved_2_to_5_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeInterleaved2of5Code }
    }

    /// objects generated from ITF14 codes return this constant as their type.
    pub fn itf_14_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeITF14Code }
    }

    /// objects generated from DataMatrix codes return this constant as their type.
    pub fn data_matrix_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeDataMatrixCode }
    }

    /// objects generated from Codabar codes return this constant as their type.
    pub fn codebar_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeCodabarCode }
    }

    /// objects generated from GS1DataBar codes return this constant as their type.
    pub fn gs_1_data_bar_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeGS1DataBarCode }
    }

    /// objects generated from GS1DataBarExpanded codes return this constant as their type.
    pub fn gs_1_data_bar_expanded_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeGS1DataBarExpandedCode }
    }

    /// objects generated from GS1DataBarLimited codes return this constant as their type.
    pub fn gs_1_data_bar_limited_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeGS1DataBarLimitedCode }
    }

    /// objects generated from MicroQR codes return this constant as their type.
    pub fn micro_qr_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeMicroQRCode }
    }

    /// objects generated from MicroPDF417 codes return this constant as their type.
    pub fn micro_pdf_417_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeMicroPDF417Code }
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
    static AVMetadataObjectTypeInterleaved2of5Code: &'static Type;
    static AVMetadataObjectTypeITF14Code: &'static Type;
    static AVMetadataObjectTypeDataMatrixCode: &'static Type;
    static AVMetadataObjectTypeCodabarCode: &'static Type;
    static AVMetadataObjectTypeGS1DataBarCode: &'static Type;
    static AVMetadataObjectTypeGS1DataBarExpandedCode: &'static Type;
    static AVMetadataObjectTypeGS1DataBarLimitedCode: &'static Type;
    static AVMetadataObjectTypeMicroQRCode: &'static Type;
    static AVMetadataObjectTypeMicroPDF417Code: &'static Type;

}

impl Object {
    pub fn bounds(&self) -> cg::Rect {
        msg_send!("common", self, sel_bounds)
    }
}
