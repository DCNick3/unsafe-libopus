pub mod typedef_h {
    pub const silk_int16_MAX: i32 = 0x7fff as i32;
    pub const silk_int16_MIN: i32 = 0x8000 as i32;
}
use crate::silk::resampler_structs::silk_resampler_state_struct;

pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};

use crate::externs::memcpy;
use crate::silk::resampler_private_up2_HQ::silk_resampler_private_up2_HQ;
use crate::silk::resampler_rom::{silk_resampler_frac_FIR_12, RESAMPLER_ORDER_FIR_12};

#[inline]
unsafe fn silk_resampler_private_IIR_FIR_INTERPOL(
    mut out: *mut i16,
    buf: *mut i16,
    max_index_Q16: i32,
    index_increment_Q16: i32,
) -> *mut i16 {
    let mut index_Q16: i32 = 0;
    let mut res_Q15: i32 = 0;
    let mut buf_ptr: *mut i16 = 0 as *mut i16;
    let mut table_index: i32 = 0;
    index_Q16 = 0 as i32;
    while index_Q16 < max_index_Q16 {
        table_index =
            ((index_Q16 & 0xffff as i32) as i64 * 12 as i32 as i16 as i64 >> 16 as i32) as i32;
        buf_ptr = &mut *buf.offset((index_Q16 >> 16 as i32) as isize) as *mut i16;
        res_Q15 = *buf_ptr.offset(0 as i32 as isize) as i32
            * silk_resampler_frac_FIR_12[table_index as usize][0 as i32 as usize] as i32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(1 as i32 as isize) as i32
                * silk_resampler_frac_FIR_12[table_index as usize][1 as i32 as usize] as i32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(2 as i32 as isize) as i32
                * silk_resampler_frac_FIR_12[table_index as usize][2 as i32 as usize] as i32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(3 as i32 as isize) as i32
                * silk_resampler_frac_FIR_12[table_index as usize][3 as i32 as usize] as i32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(4 as i32 as isize) as i32
                * silk_resampler_frac_FIR_12[(11 as i32 - table_index) as usize][3 as i32 as usize]
                    as i32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(5 as i32 as isize) as i32
                * silk_resampler_frac_FIR_12[(11 as i32 - table_index) as usize][2 as i32 as usize]
                    as i32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(6 as i32 as isize) as i32
                * silk_resampler_frac_FIR_12[(11 as i32 - table_index) as usize][1 as i32 as usize]
                    as i32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(7 as i32 as isize) as i32
                * silk_resampler_frac_FIR_12[(11 as i32 - table_index) as usize][0 as i32 as usize]
                    as i32;
        let fresh0 = out;
        out = out.offset(1);
        *fresh0 = (if (if 15 as i32 == 1 as i32 {
            (res_Q15 >> 1 as i32) + (res_Q15 & 1 as i32)
        } else {
            (res_Q15 >> 15 as i32 - 1 as i32) + 1 as i32 >> 1 as i32
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 15 as i32 == 1 as i32 {
            (res_Q15 >> 1 as i32) + (res_Q15 & 1 as i32)
        } else {
            (res_Q15 >> 15 as i32 - 1 as i32) + 1 as i32 >> 1 as i32
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 15 as i32 == 1 as i32 {
            (res_Q15 >> 1 as i32) + (res_Q15 & 1 as i32)
        } else {
            (res_Q15 >> 15 as i32 - 1 as i32) + 1 as i32 >> 1 as i32
        }) as i16;
        index_Q16 += index_increment_Q16;
    }
    return out;
}
pub unsafe fn silk_resampler_private_IIR_FIR(
    SS: *mut core::ffi::c_void,
    mut out: *mut i16,
    mut in_0: *const i16,
    mut inLen: i32,
) {
    let S: *mut silk_resampler_state_struct = SS as *mut silk_resampler_state_struct;
    let mut nSamplesIn: i32 = 0;
    let mut max_index_Q16: i32 = 0;
    let mut index_increment_Q16: i32 = 0;
    let vla = (2 as i32 * (*S).batchSize + 8 as i32) as usize;
    let mut buf: Vec<i16> = ::std::vec::from_elem(0, vla);
    memcpy(
        buf.as_mut_ptr() as *mut core::ffi::c_void,
        ((*S).sFIR.i16_0).as_mut_ptr() as *const core::ffi::c_void,
        (8 as i32 as u64).wrapping_mul(::core::mem::size_of::<i16>() as u64),
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
        max_index_Q16 = ((nSamplesIn as u32) << 16 as i32 + 1 as i32) as i32;
        out = silk_resampler_private_IIR_FIR_INTERPOL(
            out,
            buf.as_mut_ptr(),
            max_index_Q16,
            index_increment_Q16,
        );
        in_0 = in_0.offset(nSamplesIn as isize);
        inLen -= nSamplesIn;
        if !(inLen > 0 as i32) {
            break;
        }
        memcpy(
            buf.as_mut_ptr() as *mut core::ffi::c_void,
            &mut *buf.as_mut_ptr().offset((nSamplesIn << 1 as i32) as isize) as *mut i16
                as *const core::ffi::c_void,
            (8 as i32 as u64).wrapping_mul(::core::mem::size_of::<i16>() as u64),
        );
    }
    memcpy(
        ((*S).sFIR.i16_0).as_mut_ptr() as *mut core::ffi::c_void,
        &mut *buf.as_mut_ptr().offset((nSamplesIn << 1 as i32) as isize) as *mut i16
            as *const core::ffi::c_void,
        (8 as i32 as u64).wrapping_mul(::core::mem::size_of::<i16>() as u64),
    );
}
