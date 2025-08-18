pub mod cblas {

    #[link(name = "Accelerate", kind = "framework")]
    unsafe extern "C-unwind" {
        #[link_name = "cblas_scopy"]
        pub fn _copy_f32(n: isize, x: *const f32, inc_x: isize, y: *mut f32, inc_y: isize);

    }
}

pub mod catlas {

    #[link(name = "Accelerate", kind = "framework")]
    unsafe extern "C-unwind" {
        #[link_name = "catlas_saxpby"]
        pub fn _axpby_f32(
            n: isize,
            alpha: f32,
            x: *const f32,
            inc_x: isize,
            beta: f32,
            y: *mut f32,
            inc_y: isize,
        );

    }
}
