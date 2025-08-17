pub mod cblas {

    #[link(name = "Accelerate", kind = "framework")]
    unsafe extern "C-unwind" {
        #[link_name = "cblas_scopy"]
        pub fn _scopy_f32(n: isize, x: *const f32, inc_x: isize, y: *mut f32, inc_y: isize);

    }
}
