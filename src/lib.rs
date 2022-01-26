pub type FourCharCode = u32;
pub type ResType = FourCharCode; // ??

pub mod os {
    use super::FourCharCode;

    pub type Err = i16;
    pub type Status = i32;
    pub type Type = FourCharCode;

    pub const NO_ERR: Status = 0;
}

pub mod cf;
pub mod cm;
pub mod io;
pub mod vt;

#[macro_export]
macro_rules! define_ref {
    ( $Base:path, $Ref:ident, $Owned:ident ) => {
        
        #[derive(Copy, Clone)]
        #[repr(transparent)]
        pub struct $Ref($Base);

        impl $Ref {
            #[inline]
            pub fn retained(&self) -> $Owned {
                unsafe { $Owned(self.retain()) }
            }

            #[inline]
            pub unsafe fn retain(&self) -> $Ref {
                $Ref(self.0.retain())
            }
        }

        impl std::ops::Deref for $Ref {
            type Target = $Base;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $Ref {

            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
        
        #[repr(transparent)]
        pub struct $Owned($Ref);

        impl std::ops::Deref for $Owned {
            type Target = $Ref;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::ops::DerefMut for $Owned {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Drop for $Owned {

            #[inline]
            fn drop(&mut self) {
                unsafe { self.release() }
            }
        }
    };
}


#[cfg(test)]
mod tests {
    use crate::cf;
    #[test]
    fn it_works() {
        cf::Null::value().show();
        let str = cf::MutableString::create(None, 0).unwrap();
        str.show_str();
        assert_eq!(cf::array_get_type_id(), 19);
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
