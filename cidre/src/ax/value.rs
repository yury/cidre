use crate::{arc, ax, cf, cg, define_cf_type};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum ValueType {
    #[doc(alias = "kAXValueTypeCGPoint")]
    Point = 1,
    #[doc(alias = "kAXValueTypeCGSize")]
    Size = 2,
    #[doc(alias = "kAXValueTypeCGRect")]
    Rect = 3,
    #[doc(alias = "kAXValueTypeCFRange")]
    Range = 4,
    #[doc(alias = "kAXValueTypeAXError")]
    Error = 5,
    #[doc(alias = "kAXValueTypeIllegal")]
    Illegal = 0,
}

define_cf_type!(
    #[doc(alias = "AXValueRef")]
    Value(cf::Type)
);

impl Value {
    #[doc(alias = "AXValueGetTypeID")]
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { AXValueGetTypeID() }
    }

    #[doc(alias = "AXValueCreate")]
    #[inline]
    pub fn create(the_type: ValueType, value_ptr: *const std::ffi::c_void) -> Option<arc::R<Self>> {
        unsafe { AXValueCreate(the_type, value_ptr) }
    }

    #[inline]
    pub fn with_cg_point(val: &cg::Point) -> arc::R<Self> {
        unsafe { Self::create(ValueType::Point, val as *const cg::Point as _).unwrap_unchecked() }
    }

    #[inline]
    pub fn with_cg_size(val: &cg::Size) -> arc::R<Self> {
        unsafe { Self::create(ValueType::Size, val as *const cg::Size as _).unwrap_unchecked() }
    }

    #[inline]
    pub fn with_cg_rect(val: &cg::Rect) -> arc::R<Self> {
        unsafe { Self::create(ValueType::Rect, val as *const cg::Rect as _).unwrap_unchecked() }
    }

    #[inline]
    pub fn with_cf_range(val: &cf::Range) -> arc::R<Self> {
        unsafe { Self::create(ValueType::Range, val as *const cf::Range as _).unwrap_unchecked() }
    }

    #[inline]
    pub fn with_ax_error(val: &ax::Error) -> arc::R<Self> {
        unsafe { Self::create(ValueType::Error, val as *const ax::Error as _).unwrap_unchecked() }
    }

    #[inline]
    pub fn value_type(&self) -> ValueType {
        unsafe { AXValueGetType(self) }
    }

    pub fn cg_point(&self) -> Option<cg::Point> {
        self.value(ValueType::Point)
    }

    #[inline]
    pub fn is_cg_point(&self) -> bool {
        self.value_type() == ValueType::Point
    }

    pub fn cg_size(&self) -> Option<cg::Size> {
        self.value(ValueType::Size)
    }

    #[inline]
    pub fn is_cg_size(&self) -> bool {
        self.value_type() == ValueType::Size
    }

    pub fn cg_rect(&self) -> Option<cg::Rect> {
        self.value(ValueType::Rect)
    }

    #[inline]
    pub fn is_cg_rect(&self) -> bool {
        self.value_type() == ValueType::Rect
    }

    pub fn cf_range(&self) -> Option<cf::Range> {
        self.value(ValueType::Range)
    }

    pub fn is_cf_range(&self) -> bool {
        self.value_type() == ValueType::Range
    }

    pub fn ax_error(&self) -> Option<ax::Error> {
        self.value(ValueType::Error)
    }

    pub fn is_ax_error(&self) -> bool {
        self.value_type() == ValueType::Error
    }

    fn value<T>(&self, value_type: ValueType) -> Option<T> {
        let mut res = std::mem::MaybeUninit::uninit();
        unsafe {
            if AXValueGetValue(self, value_type, res.as_mut_ptr() as _) {
                Some(res.assume_init())
            } else {
                None
            }
        }
    }

    #[inline]
    pub fn is_illigal_value(&self) -> bool {
        self.value_type() == ValueType::Illegal
    }
}

#[link(name = "ApplicationServices", kind = "framework")]
unsafe extern "C" {
    fn AXValueGetTypeID() -> cf::TypeId;
    fn AXValueCreate(
        the_type: ValueType,
        value_ptr: *const std::ffi::c_void,
    ) -> Option<arc::R<Value>>;
    fn AXValueGetType(value: &Value) -> ValueType;
    fn AXValueGetValue(
        value: &Value,
        the_type: ValueType,
        value_ptr: *mut std::ffi::c_void,
    ) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::{ax, cf, cg};

    #[test]
    fn basics() {
        let a = cg::Point { x: 10.0, y: 20.0 };
        let val = ax::Value::with_cg_point(&a);
        assert_eq!(val.value_type(), ax::ValueType::Point);
        assert!(val.is_cg_point());
        let b = val.cg_point().unwrap();
        assert_eq!(b, a);
        assert!(val.cg_size().is_none());

        let val = ax::Value::with_cg_rect(&cg::Rect::zero());
        assert_eq!(val.value_type(), ax::ValueType::Rect);

        let val = ax::Value::with_cg_size(&cg::Size::zero());
        assert_eq!(val.value_type(), ax::ValueType::Size);

        let val = ax::Value::with_cf_range(&cf::Range::zero());
        assert_eq!(val.value_type(), ax::ValueType::Range);
    }
}
