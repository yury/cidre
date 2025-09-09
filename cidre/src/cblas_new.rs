pub mod cblas {

    #[link(name = "Accelerate", kind = "framework")]
    unsafe extern "C-unwind" {
        #[link_name = "cblas_scopy"]
        #[doc(alias = "cblas_scopy")]
        pub fn _copy_f32(n: isize, x: *const f32, inc_x: isize, y: *mut f32, inc_y: isize);

        #[link_name = "cblas_saxpy"]
        #[doc(alias = "cblas_saxpy")]
        pub fn _axpy_f32(
            n: isize,
            alpha: f32,
            x: *const f32,
            inc_x: isize,
            y: *mut f32,
            inc_y: isize,
        );

        #[link_name = "cblas_sdot"]
        #[doc(alias = "cblas_sdot")]
        pub fn _dot_f32(n: isize, x: *const f32, inc_x: isize, y: *const f32, inc_y: isize) -> f32;

        #[link_name = "cblas_isamax"]
        #[doc(alias = "cblas_isamax")]
        pub fn _i_abs_max_f32(n: isize, x: *const f32, inc_x: isize) -> isize;
    }
}

pub mod catlas {

    #[link(name = "Accelerate", kind = "framework")]
    unsafe extern "C-unwind" {
        #[link_name = "catlas_saxpby"]
        #[doc(alias = "catlas_saxpby")]
        pub fn _axpby_f32(
            n: isize,
            alpha: f32,
            x: *const f32,
            inc_x: isize,
            beta: f32,
            y: *mut f32,
            inc_y: isize,
        );

        #[link_name = "catlas_sset"]
        #[doc(alias = "catlas_sset")]
        pub fn _set_f32(n: isize, alpha: f32, x: *mut f32, inc_x: isize);

    }
}
