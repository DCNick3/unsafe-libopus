use crate::silk::Inlines::silk_CLZ_FRAC;

#[c2rust::src_loc = "35:1"]
pub unsafe fn silk_lin2log(inLin: i32) -> i32 {
    let mut lz: i32 = 0;
    let mut frac_Q7: i32 = 0;
    silk_CLZ_FRAC(inLin, &mut lz, &mut frac_Q7);
    return (frac_Q7 as libc::c_long
        + ((frac_Q7 * (128 as libc::c_int - frac_Q7)) as libc::c_long
            * 179 as libc::c_int as i16 as i64
            >> 16 as libc::c_int)) as i32
        + (((31 as libc::c_int - lz) as u32) << 7 as libc::c_int) as i32;
}
