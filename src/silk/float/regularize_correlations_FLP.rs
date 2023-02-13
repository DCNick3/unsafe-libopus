#[c2rust::src_loc = "35:1"]
pub unsafe fn silk_regularize_correlations_FLP(XX: *mut f32, xx: *mut f32, noise: f32, D: i32) {
    let mut i: i32 = 0;
    i = 0 as i32;
    while i < D {
        *(&mut *XX.offset(0 as i32 as isize) as *mut f32).offset((i * D + i) as isize) += noise;
        i += 1;
    }
    *xx.offset(0 as i32 as isize) += noise;
}
