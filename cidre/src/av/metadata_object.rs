use crate::{arc, cg, cm, define_obj_type, ns, objc};

define_obj_type!(
    #[doc(alias = "AVMetadataObjectType")]
    pub Type(ns::String)
);

impl Type {
    #[doc(alias = "AVMetadataObjectTypeHumanBody")]
    #[inline]
    pub fn human_body() -> &'static Type {
        unsafe { AVMetadataObjectTypeHumanBody }
    }

    #[doc(alias = "AVMetadataObjectTypeHumanFullBody")]
    #[inline]
    pub fn human_full_body() -> &'static Type {
        unsafe { AVMetadataObjectTypeHumanFullBody }
    }

    #[doc(alias = "AVMetadataObjectTypeCatBody")]
    #[inline]
    pub fn cat_body() -> &'static Type {
        unsafe { AVMetadataObjectTypeCatBody }
    }

    #[doc(alias = "AVMetadataObjectTypeDogBody")]
    #[inline]
    pub fn dog_body() -> &'static Type {
        unsafe { AVMetadataObjectTypeDogBody }
    }

    #[doc(alias = "AVMetadataObjectTypeSalientObject")]
    #[inline]
    pub fn salient_object() -> &'static Type {
        unsafe { AVMetadataObjectTypeSalientObject }
    }

    #[doc(alias = "AVMetadataObjectTypeFace")]
    #[inline]
    pub fn face() -> &'static Type {
        unsafe { AVMetadataObjectTypeFace }
    }

    /// objects generated from UPC-E codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeUPCECode")]
    #[inline]
    pub fn upc_e_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeUPCECode }
    }

    /// objects generated from Code 39 codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeCode39Code")]
    #[inline]
    pub fn code_39_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeCode39Code }
    }

    /// objects generated from Code 39 mod 43 codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeCode39Mod43Code")]
    #[inline]
    pub fn code_39_mod_43_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeCode39Mod43Code }
    }

    /// objects generated from EAN-13 (including UPC-A) codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeEAN13Code")]
    #[inline]
    pub fn ean_13_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeEAN13Code }
    }

    /// objects generated from EAN-8 codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeEAN8Code")]
    #[inline]
    pub fn ean_8_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeEAN8Code }
    }

    /// objects generated from Code 93 codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeCode93Code")]
    #[inline]
    pub fn code_93_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeCode93Code }
    }

    /// objects generated from Code 128 codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeCode128Code")]
    #[inline]
    pub fn code_128_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeCode128Code }
    }

    /// objects generated from PDF417 codes return this constant as their type
    #[doc(alias = "AVMetadataObjectTypePDF417Code")]
    #[inline]
    pub fn pdf_417_code() -> &'static Type {
        unsafe { AVMetadataObjectTypePDF417Code }
    }

    /// objects generated from QR codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeQRCode")]
    #[inline]
    pub fn qr_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeQRCode }
    }

    /// objects generated from Aztec codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeAztecCode")]
    #[inline]
    pub fn aztec_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeAztecCode }
    }

    /// objects generated from Interleaved 2 of 5 codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeInterleaved2of5Code")]
    #[inline]
    pub fn interleaved_2_to_5_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeInterleaved2of5Code }
    }

    /// objects generated from ITF14 codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeITF14Code")]
    #[inline]
    pub fn itf_14_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeITF14Code }
    }

    /// objects generated from DataMatrix codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeDataMatrixCode")]
    #[inline]
    pub fn data_matrix_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeDataMatrixCode }
    }

    /// objects generated from Codabar codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeCodabarCode")]
    #[inline]
    pub fn codebar_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeCodabarCode }
    }

    /// objects generated from GS1DataBar codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeGS1DataBarCode")]
    #[inline]
    pub fn gs_1_data_bar_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeGS1DataBarCode }
    }

    /// objects generated from GS1DataBarExpanded codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeGS1DataBarExpandedCode")]
    #[inline]
    pub fn gs_1_data_bar_expanded_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeGS1DataBarExpandedCode }
    }

    /// objects generated from GS1DataBarLimited codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeGS1DataBarLimitedCode")]
    #[inline]
    pub fn gs_1_data_bar_limited_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeGS1DataBarLimitedCode }
    }

    /// objects generated from MicroQR codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeMicroQRCode")]
    #[inline]
    pub fn micro_qr_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeMicroQRCode }
    }

    /// objects generated from MicroPDF417 codes return this constant as their type.
    #[doc(alias = "AVMetadataObjectTypeMicroPDF417Code")]
    #[inline]
    pub fn micro_pdf_417_code() -> &'static Type {
        unsafe { AVMetadataObjectTypeMicroPDF417Code }
    }
}

unsafe extern "C" {
    static AVMetadataObjectTypeHumanBody: &'static Type;
    static AVMetadataObjectTypeHumanFullBody: &'static Type;
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

define_obj_type!(
    #[doc(alias = "AVMetadataObject")]
    pub Obj(ns::Id)
);

impl Obj {
    /// The value of this property is a cg::Rect representing the bounding rectangle
    /// of the object with respect to the picture in which it resides. The rectangle's
    /// origin is top left. If the metadata originates from video, bounds may be
    /// expressed as scalar values from 0. - 1. If the original video has been scaled
    /// down, the bounds of the metadata object still are meaningful. This property may
    /// return cg::Rect::zero if the metadata has no bounds.
    #[objc::msg_send(bounds)]
    pub fn bounds(&self) -> cg::Rect;

    /// The media time associated with this metadata object.
    ///
    /// The value of this property is a cm::Time associated with the metadata object.
    /// For capture, it is the time at which this object was captured. If this metadata
    /// object originates from a cm::SampleBuffer, its time matches the sample buffer's
    /// presentation time. This property may return cm::Time::invalid.
    #[objc::msg_send(time)]
    pub fn time(&self) -> cm::Time;

    /// The media duration associated with this metadata object.
    ///
    /// The value of this property is a cm::Time representing the duration
    /// of the metadata object. If this metadata object originates from a cm::SampleBuffer,
    /// its duration matches the sample buffer's duration.
    /// This property may return cm::Time::invalid.
    #[objc::msg_send(duration)]
    pub fn duration(&self) -> cm::Time;

    /// An identifier for the metadata object.
    ///
    /// Clients inspecting a collection of metadata objects can use this property to filter objects
    /// with a matching type.
    #[objc::msg_send(type)]
    pub fn obj_type(&self) -> &Type;
}

define_obj_type!(
    #[doc(alias = "AVMetadataBodyObject")]
    pub BodyObj(Obj));

impl BodyObj {
    #[objc::msg_send(bodyID)]
    pub fn body_id(&self) -> isize;
}

define_obj_type!(
    #[doc(alias = "AVMetadataCatBodyObject")]
    pub CatBodyObj(BodyObj)
);

define_obj_type!(
    #[doc(alias = "AVMetadataDogBodyObject")]
    pub DogBodyObj(BodyObj)
);

define_obj_type!(
    #[doc(alias = "AVMetadataHumanBodyObject")]
    pub HumanBodyObj(BodyObj)
);

define_obj_type!(
    #[doc(alias = "AVMetadataHumanFullBodyObject")]
    pub HumanFullBodyObj(BodyObj)
);

define_obj_type!(
    #[doc(alias = "AVMetadataFaceObject")]
    pub FaceObj(Obj)
);

define_obj_type!(
    #[doc(alias = "AVMetadataMachineReadableCodeObject")]
    pub MachineReadableCodeObj(Obj)
);

define_obj_type!(
    #[doc(alias = "AVMetadataSalientObject")]
    pub SalientObj(Obj)
);

impl SalientObj {
    #[objc::msg_send(objectID)]
    pub fn obj_id(&self) -> isize;
}

impl FaceObj {
    /// A unique number associated with the receiver.
    ///
    /// The value of this property is an isize indicating the unique identifier of this face in the picture.
    /// When a new face enters the picture, it is assigned a new unique identifier. face_ids are not re-used as faces
    /// leave the picture and new ones enter. Faces that leave the picture then re-enter are assigned a new face_id.
    #[objc::msg_send(faceID)]
    pub fn face_id(&self) -> isize;

    /// The roll angle of the face in degrees.
    ///
    /// The value of this property is a cg::Float indicating the face's angle of roll (or tilt) in degrees.
    /// A value of 0.0 indicates that the face is level in the picture. If -has_roll_angle returns false, then reading this property throws
    /// an ns::GenericException.
    #[objc::msg_send(hasRollAngle)]
    pub fn has_roll_angle(&self) -> bool;

    #[objc::msg_send(rollAngle)]
    pub unsafe fn roll_angle_throws(&self) -> cg::Float;

    /// The roll angle of the face in degrees.
    #[inline]
    pub fn roll_angle(&self) -> Option<cg::Float> {
        if self.has_roll_angle() {
            Some(unsafe { self.roll_angle_throws() })
        } else {
            None
        }
    }
    #[objc::msg_send(hasYawAngle)]
    pub fn has_yaw_angle(&self) -> bool;

    /// The yaw angle of the face in degrees.
    ///
    /// The value of this property is a cg::Float indicating the face's angle of yaw (or turn) in degrees.
    /// A value of 0.0 indicates that the face is straight on in the picture.
    /// If has_yaw_angle() returns false, then reading this property throws an ns::GenericException.
    #[objc::msg_send(yawAngle)]
    pub unsafe fn yaw_angle_throws(&self) -> cg::Float;

    /// The yaw angle of the face in degrees.
    #[inline]
    pub fn yaw_angle(&self) -> Option<cg::Float> {
        if self.has_yaw_angle() {
            Some(unsafe { self.yaw_angle_throws() })
        } else {
            None
        }
    }
}

impl MachineReadableCodeObj {
    /// The points defining the (X,Y) locations of the corners of the machine-readable code.
    #[objc::msg_send(corners)]
    pub fn corners(&self) -> arc::R<ns::Dictionary<ns::String, ns::Id>>;

    /// Returns the receiver's errorCorrectedData decoded into a human-readable string.
    #[objc::msg_send(stringValue)]
    pub fn string_value(&self) -> Option<arc::R<ns::String>>;
}

#[cfg(feature = "ci")]
use crate::ci;

/// AVMetadataMachineReadableCodeDescriptor
#[cfg(feature = "ci")]
impl MachineReadableCodeObj {
    /// An abstract representation of a machine readable code's symbol attributes.
    ///
    /// The value may be None if an abstract representation of a machine readable
    /// code object is not defined for the code type or could not be detected.
    #[objc::msg_send(descriptor)]
    pub fn descriptor(&self) -> Option<arc::R<ci::BarcodeDesc>>;
}
