pub mod typedef_h {
    pub const silk_int16_MAX: i32 = i16::MAX as i32;
    pub const silk_int16_MIN: i32 = i16::MIN as i32;
}
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
use crate::celt::celt::celt_fatal;
use crate::externs::memcpy;
use crate::silk::resampler_private_AR2::silk_resampler_private_AR2;
use crate::silk::resampler_rom::{
    RESAMPLER_DOWN_ORDER_FIR0, RESAMPLER_DOWN_ORDER_FIR1, RESAMPLER_DOWN_ORDER_FIR2,
};
use crate::silk::resampler_structs::silk_resampler_state_struct;

#[inline]
unsafe fn silk_resampler_private_down_FIR_INTERPOL(
    mut out: *mut i16,
    buf: *mut i32,
    FIR_Coefs: *const i16,
    FIR_Order: i32,
    FIR_Fracs: i32,
    max_index_Q16: i32,
    index_increment_Q16: i32,
) -> *mut i16 {
    let mut index_Q16: i32 = 0;
    let mut res_Q6: i32 = 0;
    let mut buf_ptr: *mut i32 = 0 as *mut i32;
    let mut interpol_ind: i32 = 0;
    let mut interpol_ptr: *const i16 = 0 as *const i16;
    match FIR_Order {
        RESAMPLER_DOWN_ORDER_FIR0 => {
            index_Q16 = 0 as i32;
            while index_Q16 < max_index_Q16 {
                buf_ptr = buf.offset((index_Q16 >> 16 as i32) as isize);
                interpol_ind = ((index_Q16 & 0xffff as i32) as i64 * FIR_Fracs as i16 as i64
                    >> 16 as i32) as i32;
                interpol_ptr = &*FIR_Coefs
                    .offset((RESAMPLER_DOWN_ORDER_FIR0 / 2 as i32 * interpol_ind) as isize)
                    as *const i16;
                res_Q6 = (*buf_ptr.offset(0 as i32 as isize) as i64
                    * *interpol_ptr.offset(0 as i32 as isize) as i64
                    >> 16 as i32) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(1 as i32 as isize) as i64
                        * *interpol_ptr.offset(1 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(2 as i32 as isize) as i64
                        * *interpol_ptr.offset(2 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(3 as i32 as isize) as i64
                        * *interpol_ptr.offset(3 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(4 as i32 as isize) as i64
                        * *interpol_ptr.offset(4 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(5 as i32 as isize) as i64
                        * *interpol_ptr.offset(5 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(6 as i32 as isize) as i64
                        * *interpol_ptr.offset(6 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(7 as i32 as isize) as i64
                        * *interpol_ptr.offset(7 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(8 as i32 as isize) as i64
                        * *interpol_ptr.offset(8 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                interpol_ptr = &*FIR_Coefs.offset(
                    (RESAMPLER_DOWN_ORDER_FIR0 / 2 as i32 * (FIR_Fracs - 1 as i32 - interpol_ind))
                        as isize,
                ) as *const i16;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(17 as i32 as isize) as i64
                        * *interpol_ptr.offset(0 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(16 as i32 as isize) as i64
                        * *interpol_ptr.offset(1 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(15 as i32 as isize) as i64
                        * *interpol_ptr.offset(2 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(14 as i32 as isize) as i64
                        * *interpol_ptr.offset(3 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(13 as i32 as isize) as i64
                        * *interpol_ptr.offset(4 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(12 as i32 as isize) as i64
                        * *interpol_ptr.offset(5 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(11 as i32 as isize) as i64
                        * *interpol_ptr.offset(6 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(10 as i32 as isize) as i64
                        * *interpol_ptr.offset(7 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + (*buf_ptr.offset(9 as i32 as isize) as i64
                        * *interpol_ptr.offset(8 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                let fresh0 = out;
                out = out.offset(1);
                *fresh0 = (if (if 6 as i32 == 1 as i32 {
                    (res_Q6 >> 1 as i32) + (res_Q6 & 1 as i32)
                } else {
                    (res_Q6 >> 6 as i32 - 1 as i32) + 1 as i32 >> 1 as i32
                }) > silk_int16_MAX
                {
                    silk_int16_MAX
                } else if (if 6 as i32 == 1 as i32 {
                    (res_Q6 >> 1 as i32) + (res_Q6 & 1 as i32)
                } else {
                    (res_Q6 >> 6 as i32 - 1 as i32) + 1 as i32 >> 1 as i32
                }) < silk_int16_MIN
                {
                    silk_int16_MIN
                } else if 6 as i32 == 1 as i32 {
                    (res_Q6 >> 1 as i32) + (res_Q6 & 1 as i32)
                } else {
                    (res_Q6 >> 6 as i32 - 1 as i32) + 1 as i32 >> 1 as i32
                }) as i16;
                index_Q16 += index_increment_Q16;
            }
        }
        RESAMPLER_DOWN_ORDER_FIR1 => {
            index_Q16 = 0 as i32;
            while index_Q16 < max_index_Q16 {
                buf_ptr = buf.offset((index_Q16 >> 16 as i32) as isize);
                res_Q6 = ((*buf_ptr.offset(0 as i32 as isize) + *buf_ptr.offset(23 as i32 as isize))
                    as i64
                    * *FIR_Coefs.offset(0 as i32 as isize) as i64
                    >> 16 as i32) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(1 as i32 as isize) + *buf_ptr.offset(22 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(1 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(2 as i32 as isize) + *buf_ptr.offset(21 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(2 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(3 as i32 as isize) + *buf_ptr.offset(20 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(3 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(4 as i32 as isize) + *buf_ptr.offset(19 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(4 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(5 as i32 as isize) + *buf_ptr.offset(18 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(5 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(6 as i32 as isize) + *buf_ptr.offset(17 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(6 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(7 as i32 as isize) + *buf_ptr.offset(16 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(7 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(8 as i32 as isize) + *buf_ptr.offset(15 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(8 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(9 as i32 as isize) + *buf_ptr.offset(14 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(9 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(10 as i32 as isize) + *buf_ptr.offset(13 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(10 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(11 as i32 as isize) + *buf_ptr.offset(12 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(11 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                let fresh1 = out;
                out = out.offset(1);
                *fresh1 = (if (if 6 as i32 == 1 as i32 {
                    (res_Q6 >> 1 as i32) + (res_Q6 & 1 as i32)
                } else {
                    (res_Q6 >> 6 as i32 - 1 as i32) + 1 as i32 >> 1 as i32
                }) > silk_int16_MAX
                {
                    silk_int16_MAX
                } else if (if 6 as i32 == 1 as i32 {
                    (res_Q6 >> 1 as i32) + (res_Q6 & 1 as i32)
                } else {
                    (res_Q6 >> 6 as i32 - 1 as i32) + 1 as i32 >> 1 as i32
                }) < silk_int16_MIN
                {
                    silk_int16_MIN
                } else if 6 as i32 == 1 as i32 {
                    (res_Q6 >> 1 as i32) + (res_Q6 & 1 as i32)
                } else {
                    (res_Q6 >> 6 as i32 - 1 as i32) + 1 as i32 >> 1 as i32
                }) as i16;
                index_Q16 += index_increment_Q16;
            }
        }
        RESAMPLER_DOWN_ORDER_FIR2 => {
            index_Q16 = 0 as i32;
            while index_Q16 < max_index_Q16 {
                buf_ptr = buf.offset((index_Q16 >> 16 as i32) as isize);
                res_Q6 = ((*buf_ptr.offset(0 as i32 as isize) + *buf_ptr.offset(35 as i32 as isize))
                    as i64
                    * *FIR_Coefs.offset(0 as i32 as isize) as i64
                    >> 16 as i32) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(1 as i32 as isize) + *buf_ptr.offset(34 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(1 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(2 as i32 as isize) + *buf_ptr.offset(33 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(2 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(3 as i32 as isize) + *buf_ptr.offset(32 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(3 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(4 as i32 as isize) + *buf_ptr.offset(31 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(4 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(5 as i32 as isize) + *buf_ptr.offset(30 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(5 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(6 as i32 as isize) + *buf_ptr.offset(29 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(6 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(7 as i32 as isize) + *buf_ptr.offset(28 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(7 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(8 as i32 as isize) + *buf_ptr.offset(27 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(8 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(9 as i32 as isize) + *buf_ptr.offset(26 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(9 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(10 as i32 as isize) + *buf_ptr.offset(25 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(10 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(11 as i32 as isize) + *buf_ptr.offset(24 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(11 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(12 as i32 as isize) + *buf_ptr.offset(23 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(12 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(13 as i32 as isize) + *buf_ptr.offset(22 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(13 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(14 as i32 as isize) + *buf_ptr.offset(21 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(14 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(15 as i32 as isize) + *buf_ptr.offset(20 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(15 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(16 as i32 as isize) + *buf_ptr.offset(19 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(16 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                res_Q6 = (res_Q6 as i64
                    + ((*buf_ptr.offset(17 as i32 as isize) + *buf_ptr.offset(18 as i32 as isize))
                        as i64
                        * *FIR_Coefs.offset(17 as i32 as isize) as i64
                        >> 16 as i32)) as i32;
                let fresh2 = out;
                out = out.offset(1);
                *fresh2 = (if (if 6 as i32 == 1 as i32 {
                    (res_Q6 >> 1 as i32) + (res_Q6 & 1 as i32)
                } else {
                    (res_Q6 >> 6 as i32 - 1 as i32) + 1 as i32 >> 1 as i32
                }) > silk_int16_MAX
                {
                    silk_int16_MAX
                } else if (if 6 as i32 == 1 as i32 {
                    (res_Q6 >> 1 as i32) + (res_Q6 & 1 as i32)
                } else {
                    (res_Q6 >> 6 as i32 - 1 as i32) + 1 as i32 >> 1 as i32
                }) < silk_int16_MIN
                {
                    silk_int16_MIN
                } else if 6 as i32 == 1 as i32 {
                    (res_Q6 >> 1 as i32) + (res_Q6 & 1 as i32)
                } else {
                    (res_Q6 >> 6 as i32 - 1 as i32) + 1 as i32 >> 1 as i32
                }) as i16;
                index_Q16 += index_increment_Q16;
            }
        }
        _ => {
            if 0 as i32 == 0 {
                celt_fatal(
                    b"assertion failed: 0\0" as *const u8 as *const i8,
                    b"silk/resampler_private_down_FIR.c\0" as *const u8 as *const i8,
                    139 as i32,
                );
            }
        }
    }
    return out;
}
pub unsafe fn silk_resampler_private_down_FIR(
    SS: *mut core::ffi::c_void,
    mut out: *mut i16,
    mut in_0: *const i16,
    mut inLen: i32,
) {
    let S: *mut silk_resampler_state_struct = SS as *mut silk_resampler_state_struct;
    let mut nSamplesIn: i32 = 0;
    let mut max_index_Q16: i32 = 0;
    let mut index_increment_Q16: i32 = 0;
    let mut FIR_Coefs: *const i16 = 0 as *const i16;
    let vla = ((*S).batchSize + (*S).FIR_Order) as usize;
    let mut buf: Vec<i32> = ::std::vec::from_elem(0, vla);
    memcpy(
        buf.as_mut_ptr() as *mut core::ffi::c_void,
        ((*S).sFIR.i32_0).as_mut_ptr() as *const core::ffi::c_void,
        ((*S).FIR_Order as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    FIR_Coefs = &*((*S).Coefs).offset(2 as i32 as isize) as *const i16;
    index_increment_Q16 = (*S).invRatio_Q16;
    loop {
        nSamplesIn = if inLen < (*S).batchSize {
            inLen
        } else {
            (*S).batchSize
        };
        silk_resampler_private_AR2(
            ((*S).sIIR).as_mut_ptr(),
            &mut *buf.as_mut_ptr().offset((*S).FIR_Order as isize),
            in_0,
            (*S).Coefs,
            nSamplesIn,
        );
        max_index_Q16 = ((nSamplesIn as u32) << 16 as i32) as i32;
        out = silk_resampler_private_down_FIR_INTERPOL(
            out,
            buf.as_mut_ptr(),
            FIR_Coefs,
            (*S).FIR_Order,
            (*S).FIR_Fracs,
            max_index_Q16,
            index_increment_Q16,
        );
        in_0 = in_0.offset(nSamplesIn as isize);
        inLen -= nSamplesIn;
        if !(inLen > 1 as i32) {
            break;
        }
        memcpy(
            buf.as_mut_ptr() as *mut core::ffi::c_void,
            &mut *buf.as_mut_ptr().offset(nSamplesIn as isize) as *mut i32
                as *const core::ffi::c_void,
            ((*S).FIR_Order as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
        );
    }
    memcpy(
        ((*S).sFIR.i32_0).as_mut_ptr() as *mut core::ffi::c_void,
        &mut *buf.as_mut_ptr().offset(nSamplesIn as isize) as *mut i32 as *const core::ffi::c_void,
        ((*S).FIR_Order as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
}
