use ::libc;

use crate::silk::macros::silk_CLZ32;
use crate::silk::SigProc_FIX::silk_max_32;

#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn silk_sum_sqr_shift(
    energy: *mut i32,
    shift: *mut libc::c_int,
    x: *const i16,
    len: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut shft: libc::c_int = 0;
    let mut nrg_tmp: u32 = 0;
    let mut nrg: i32 = 0;
    shft = 31 as libc::c_int - silk_CLZ32(len);
    nrg = len;
    i = 0 as libc::c_int;
    while i < len - 1 as libc::c_int {
        nrg_tmp = (*x.offset(i as isize) as i32 * *x.offset(i as isize) as i32) as u32;
        nrg_tmp = nrg_tmp.wrapping_add(
            (*x.offset((i + 1 as libc::c_int) as isize) as i32
                * *x.offset((i + 1 as libc::c_int) as isize) as i32) as u32,
        ) as i32 as u32;
        nrg = (nrg as libc::c_uint).wrapping_add(nrg_tmp >> shft) as i32;
        i += 2 as libc::c_int;
    }
    if i < len {
        nrg_tmp = (*x.offset(i as isize) as i32 * *x.offset(i as isize) as i32) as u32;
        nrg = (nrg as libc::c_uint).wrapping_add(nrg_tmp >> shft) as i32;
    }
    shft = silk_max_32(0 as libc::c_int, shft + 3 as libc::c_int - silk_CLZ32(nrg));
    nrg = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len - 1 as libc::c_int {
        nrg_tmp = (*x.offset(i as isize) as i32 * *x.offset(i as isize) as i32) as u32;
        nrg_tmp = nrg_tmp.wrapping_add(
            (*x.offset((i + 1 as libc::c_int) as isize) as i32
                * *x.offset((i + 1 as libc::c_int) as isize) as i32) as u32,
        ) as i32 as u32;
        nrg = (nrg as libc::c_uint).wrapping_add(nrg_tmp >> shft) as i32;
        i += 2 as libc::c_int;
    }
    if i < len {
        nrg_tmp = (*x.offset(i as isize) as i32 * *x.offset(i as isize) as i32) as u32;
        nrg = (nrg as libc::c_uint).wrapping_add(nrg_tmp >> shft) as i32;
    }
    *shift = shft;
    *energy = nrg;
}
