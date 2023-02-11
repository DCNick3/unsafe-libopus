use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_rom.h:33"]
pub mod resampler_rom_h {
    #[c2rust::src_loc = "42:9"]
    pub const RESAMPLER_ORDER_FIR_12: libc::c_int = 8 as libc::c_int;
    extern "C" {
        #[c2rust::src_loc = "62:25"]
        pub static silk_resampler_frac_FIR_12: [[i16; 4]; 12];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_private.h:33"]
pub mod resampler_private_h {
    extern "C" {
        #[c2rust::src_loc = "69:1"]
        pub fn silk_resampler_private_up2_HQ(
            S: *mut i32,
            out: *mut i16,
            in_0: *const i16,
            len: i32,
        );
    }
}
use self::resampler_private_h::silk_resampler_private_up2_HQ;
pub use self::resampler_rom_h::{silk_resampler_frac_FIR_12, RESAMPLER_ORDER_FIR_12};
use crate::silk::resampler_structs::silk_resampler_state_struct;

pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};

use crate::externs::memcpy;
#[inline]
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn silk_resampler_private_IIR_FIR_INTERPOL(
    mut out: *mut i16,
    buf: *mut i16,
    max_index_Q16: i32,
    index_increment_Q16: i32,
) -> *mut i16 {
    let mut index_Q16: i32 = 0;
    let mut res_Q15: i32 = 0;
    let mut buf_ptr: *mut i16 = 0 as *mut i16;
    let mut table_index: i32 = 0;
    index_Q16 = 0 as libc::c_int;
    while index_Q16 < max_index_Q16 {
        table_index = ((index_Q16 & 0xffff as libc::c_int) as libc::c_long
            * 12 as libc::c_int as i16 as i64
            >> 16 as libc::c_int) as i32;
        buf_ptr = &mut *buf.offset((index_Q16 >> 16 as libc::c_int) as isize) as *mut i16;
        res_Q15 = *buf_ptr.offset(0 as libc::c_int as isize) as i32
            * silk_resampler_frac_FIR_12[table_index as usize][0 as libc::c_int as usize] as i32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(1 as libc::c_int as isize) as i32
                * silk_resampler_frac_FIR_12[table_index as usize][1 as libc::c_int as usize]
                    as i32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(2 as libc::c_int as isize) as i32
                * silk_resampler_frac_FIR_12[table_index as usize][2 as libc::c_int as usize]
                    as i32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(3 as libc::c_int as isize) as i32
                * silk_resampler_frac_FIR_12[table_index as usize][3 as libc::c_int as usize]
                    as i32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(4 as libc::c_int as isize) as i32
                * silk_resampler_frac_FIR_12[(11 as libc::c_int - table_index) as usize]
                    [3 as libc::c_int as usize] as i32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(5 as libc::c_int as isize) as i32
                * silk_resampler_frac_FIR_12[(11 as libc::c_int - table_index) as usize]
                    [2 as libc::c_int as usize] as i32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(6 as libc::c_int as isize) as i32
                * silk_resampler_frac_FIR_12[(11 as libc::c_int - table_index) as usize]
                    [1 as libc::c_int as usize] as i32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(7 as libc::c_int as isize) as i32
                * silk_resampler_frac_FIR_12[(11 as libc::c_int - table_index) as usize]
                    [0 as libc::c_int as usize] as i32;
        let fresh0 = out;
        out = out.offset(1);
        *fresh0 = (if (if 15 as libc::c_int == 1 as libc::c_int {
            (res_Q15 >> 1 as libc::c_int) + (res_Q15 & 1 as libc::c_int)
        } else {
            (res_Q15 >> 15 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 15 as libc::c_int == 1 as libc::c_int {
            (res_Q15 >> 1 as libc::c_int) + (res_Q15 & 1 as libc::c_int)
        } else {
            (res_Q15 >> 15 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 15 as libc::c_int == 1 as libc::c_int {
            (res_Q15 >> 1 as libc::c_int) + (res_Q15 & 1 as libc::c_int)
        } else {
            (res_Q15 >> 15 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as i16;
        index_Q16 += index_increment_Q16;
    }
    return out;
}
#[no_mangle]
#[c2rust::src_loc = "65:1"]
pub unsafe extern "C" fn silk_resampler_private_IIR_FIR(
    SS: *mut libc::c_void,
    mut out: *mut i16,
    mut in_0: *const i16,
    mut inLen: i32,
) {
    let S: *mut silk_resampler_state_struct = SS as *mut silk_resampler_state_struct;
    let mut nSamplesIn: i32 = 0;
    let mut max_index_Q16: i32 = 0;
    let mut index_increment_Q16: i32 = 0;
    let vla = (2 as libc::c_int * (*S).batchSize + 8 as libc::c_int) as usize;
    let mut buf: Vec<i16> = ::std::vec::from_elem(0, vla);
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        ((*S).sFIR.i16_0).as_mut_ptr() as *const libc::c_void,
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
    index_increment_Q16 = (*S).invRatio_Q16;
    loop {
        nSamplesIn = if inLen < (*S).batchSize {
            inLen
        } else {
            (*S).batchSize
        };
        silk_resampler_private_up2_HQ(
            ((*S).sIIR).as_mut_ptr(),
            &mut *buf.as_mut_ptr().offset(RESAMPLER_ORDER_FIR_12 as isize),
            in_0,
            nSamplesIn,
        );
        max_index_Q16 = ((nSamplesIn as u32) << 16 as libc::c_int + 1 as libc::c_int) as i32;
        out = silk_resampler_private_IIR_FIR_INTERPOL(
            out,
            buf.as_mut_ptr(),
            max_index_Q16,
            index_increment_Q16,
        );
        in_0 = in_0.offset(nSamplesIn as isize);
        inLen -= nSamplesIn;
        if !(inLen > 0 as libc::c_int) {
            break;
        }
        memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            &mut *buf
                .as_mut_ptr()
                .offset((nSamplesIn << 1 as libc::c_int) as isize) as *mut i16
                as *const libc::c_void,
            (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
        );
    }
    memcpy(
        ((*S).sFIR.i16_0).as_mut_ptr() as *mut libc::c_void,
        &mut *buf
            .as_mut_ptr()
            .offset((nSamplesIn << 1 as libc::c_int) as isize) as *mut i16
            as *const libc::c_void,
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i16>() as libc::c_ulong),
    );
}
