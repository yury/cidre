use crate::{api, arc, blocks, cv, define_obj_type, ns, objc};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[doc(alias = "MLMultiArrayDataType")]
#[repr(isize)]
pub enum MultiArrayDType {
    #[doc(alias = "MLMultiArrayDataTypeDouble")]
    #[doc(alias = "MLMultiArrayDataTypeFloat64")]
    F64 = 0x10000 | 64,

    #[doc(alias = "MLMultiArrayDataTypeFloat32")]
    #[doc(alias = "MLMultiArrayDataTypeFloat")]
    F32 = 0x10000 | 32,

    #[doc(alias = "MLMultiArrayDataTypeFloat16")]
    F16 = 0x10000 | 16,

    #[doc(alias = "MLMultiArrayDataTypeInt32")]
    I32 = 0x20000 | 32,

    #[doc(alias = "MLMultiArrayDataTypeInt8")]
    I8 = 0x20000 | 8,
}

define_obj_type!(
    #[doc(alias = "MLMultiArray")]
    pub MultiArray(ns::Id),
    ML_MULTI_ARRAY,
    #[api::available(macos = 10.13, ios = 11.0, watchos = 4.0, tvos = 11.0)]
);

impl MultiArray {
    #[objc::msg_send(dataType)]
    pub fn d_type(&self) -> MultiArrayDType;

    #[objc::msg_send(shape)]
    pub fn shape(&self) -> arc::R<ns::Array<ns::Number>>;

    #[objc::msg_send(strides)]
    pub fn strides(&self) -> arc::R<ns::Array<ns::Number>>;

    #[objc::msg_send(count)]
    pub fn count(&self) -> isize;

    #[objc::msg_send(pixelBuffer)]
    pub fn pixel_buf(&self) -> Option<&cv::PixelBuf>;
}

impl arc::A<MultiArray> {
    #[objc::msg_send(initWithShape:dataType:error:)]
    pub unsafe fn init_with_shape_err<'ear>(
        self,
        shape: &ns::Array<ns::Number>,
        d_type: MultiArrayDType,
        err: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<MultiArray>>;

    #[inline(never)]
    #[objc::msg_send(initWithShape:dataType:strides:)]
    pub fn init_with_shape_strides(
        self,
        shape: &ns::Array<ns::Number>,
        d_type: MultiArrayDType,
        strides: &ns::Array<ns::Number>,
    ) -> arc::R<MultiArray>;

    #[objc::msg_send(initWithDataPointer:shape:dataType:strides:deallocator:error:)]
    pub unsafe fn init_with_ptr<'ear>(
        self,
        data_ptr: *mut std::ffi::c_void,
        shape: &ns::Array<ns::Number>,
        d_type: MultiArrayDType,
        strides: &ns::Array<ns::Number>,
        deallocator: Option<&mut blocks::EscBlock<fn(ptr: *mut std::ffi::c_void)>>,
        error: *mut Option<&'ear ns::Error>,
    ) -> Option<arc::R<MultiArray>>;

    #[objc::msg_send(initWithPixelBuffer:shape:)]
    pub unsafe fn init_with_pixel_buf_throws(
        self,
        pixel_buf: &cv::PixelBuf,
        shape: &ns::Array<ns::Number>,
    ) -> arc::R<MultiArray>;
}

/// Creation
impl MultiArray {
    pub fn with_shape<'ear, S: AsRef<ns::Array<ns::Number>>>(
        shape: S,
        d_type: MultiArrayDType,
    ) -> ns::Result<'ear, arc::R<Self>> {
        ns::if_none(|err| unsafe { Self::alloc().init_with_shape_err(shape.as_ref(), d_type, err) })
    }

    pub fn with_strides<'ear, S: AsRef<ns::Array<ns::Number>>>(
        shape: S,
        d_type: MultiArrayDType,
        strides: S,
    ) -> arc::R<Self> {
        Self::alloc().init_with_shape_strides(shape.as_ref(), d_type, strides.as_ref())
    }
    pub fn with_pixel_buf<'ear, S: AsRef<ns::Array<ns::Number>>>(
        pixel_buf: &cv::PixelBuf,
        shape: S,
    ) -> ns::ExResult<'ear, arc::R<Self>> {
        ns::try_catch(|| unsafe {
            Self::alloc().init_with_pixel_buf_throws(pixel_buf, shape.as_ref())
        })
    }

    pub unsafe fn with_ptr<'ear, S: AsRef<ns::Array<ns::Number>>>(
        data_ptr: *mut std::ffi::c_void,
        shape: S,
        d_type: MultiArrayDType,
        strides: S,
        deallocator: Option<&mut blocks::EscBlock<fn(ptr: *mut std::ffi::c_void)>>,
    ) -> ns::Result<'ear, arc::R<Self>> {
        ns::if_none(|err| unsafe {
            Self::alloc().init_with_ptr(
                data_ptr,
                shape.as_ref(),
                d_type,
                strides.as_ref(),
                deallocator,
                err,
            )
        })
    }
}

/// ScopedBufferAccess
impl MultiArray {
    #[objc::msg_send(dataPointer)]
    pub unsafe fn data_ptr(&mut self) -> *mut std::ffi::c_void;

    #[objc::msg_send(getBytesWithHandler:)]
    pub fn bytes_handler(
        &self,
        handler: &mut blocks::NoEscBlock<fn(ptr: *const std::ffi::c_void, size: isize)>,
    );

    #[inline]
    pub fn bytes(&self, mut handler: impl FnMut(*const std::ffi::c_void, isize)) {
        let mut block = unsafe { blocks::NoEscBlock::stack2(&mut handler) };
        self.bytes_handler(&mut block);
    }

    #[objc::msg_send(getMutableBytesWithHandler:)]
    pub fn bytes_mut_handler(
        &mut self,
        handler: &mut blocks::NoEscBlock<
            fn(ptr: *mut std::ffi::c_void, size: isize, stides: &ns::Array<ns::Number>),
        >,
    );

    #[inline]
    pub fn bytes_mut(
        &mut self,
        mut handler: impl FnMut(*mut std::ffi::c_void, isize, &ns::Array<ns::Number>),
    ) {
        let mut block = unsafe { blocks::NoEscBlock::stack3(&mut handler) };
        self.bytes_mut_handler(&mut block);
    }
}

/// Concatenating
impl MultiArray {
    #[objc::msg_send(multiArrayByConcatenatingMultiArrays:alongAxis:dataType:)]
    pub unsafe fn concat_throws(
        multi_arrays: &ns::Array<Self>,
        axis: isize,
        d_type: MultiArrayDType,
    ) -> arc::R<Self>;

    pub fn concat<'ear, S: AsRef<ns::Array<Self>>>(
        multi_arrays: S,
        axis: isize,
        d_type: MultiArrayDType,
    ) -> ns::ExResult<'ear, arc::R<Self>> {
        ns::try_catch(|| unsafe { Self::concat_throws(multi_arrays.as_ref(), axis, d_type) })
    }
}

/// NSNumberDataAccess
impl MultiArray {
    #[objc::msg_send(objectAtIndexedSubscript:)]
    pub fn get(&self, idx: isize) -> arc::R<ns::Number>;

    #[objc::msg_send(objectForKeyedSubscript:)]
    pub fn get_at(&self, key: &ns::Array<ns::Number>) -> arc::R<ns::Number>;

    #[objc::msg_send(setObject:atIndexedSubscript:)]
    pub fn set(&mut self, val: &ns::Number, idx: isize);

    #[objc::msg_send(setObject:forKeyedSubscript:)]
    pub fn set_at(&mut self, val: &ns::Number, key: &ns::Array<ns::Number>);
}

/// Transferring
impl MultiArray {
    #[objc::msg_send(transferToMultiArray:)]
    pub fn transfer_to(&self, dst: &mut Self);
}

#[link(name = "ml", kind = "static")]
unsafe extern "C" {
    static ML_MULTI_ARRAY: &'static objc::Class<MultiArray>;
}

#[cfg(test)]
mod tests {
    use crate::{cf, cv, ml, ns};

    #[test]
    fn basics() {
        let shape = ns::arr![10, 5, 3, 10];
        let arr = ml::MultiArray::with_shape(shape, ml::MultiArrayDType::F16).unwrap();
        println!("{arr:?} {}", arr.count());
        let arr = ml::MultiArray::with_shape(ns::arr![3], ml::MultiArrayDType::F32).unwrap();
        println!("{arr:?}");

        let arr =
            ml::MultiArray::with_strides(ns::arr![2, 3], ml::MultiArrayDType::F32, ns::arr![4, 0]);
        println!("{:?}", arr.count());

        let attrs = cf::Dictionary::with_keys_values(
            &[cv::pixel_buffer_keys::io_surf_props()],
            &[&cf::Dictionary::new()],
        )
        .unwrap();

        let pixel_buf =
            cv::PixelBuf::new(4, 6, cv::PixelFormat::ONE_COMPONENT_H16, Some(&attrs)).unwrap();
        // cv::PixelBuf::new(4, 6, cv::PixelFormat::ONE_COMPONENT_H16, None).unwrap();
        let mut arr = ml::MultiArray::with_pixel_buf(&pixel_buf, ns::arr![2, 3, 4]).unwrap();
        let mut len = 0;
        arr.bytes(|_ptr, size| {
            len = size as usize;
        });
        assert_eq!(384, len);

        arr.bytes_mut(|_ptr, size, _strides| {
            len = size as usize;
        });
        assert_eq!(384, len);

        let _v = arr.get(-10);
    }
}
