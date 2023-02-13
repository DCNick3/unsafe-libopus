pub mod typedef_h {
    pub const silk_int16_MIN: i32 = 0x8000 as i32;
    pub const silk_int16_MAX: i32 = 0x7fff as i32;
}
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};

use crate::externs::memcpy;
use crate::silk::resampler_private_AR2::silk_resampler_private_AR2;
use crate::silk::resampler_rom::silk_Resampler_2_3_COEFS_LQ;

pub const ORDER_FIR: i32 = 4 as i32;
pub unsafe fn silk_resampler_down2_3(
    S: *mut i32,
    mut out: *mut i16,
    mut in_0: *const i16,
    mut inLen: i32,
) {
    let mut nSamplesIn: i32 = 0;
    let mut counter: i32 = 0;
    let mut res_Q6: i32 = 0;
    let mut buf_ptr: *mut i32 = 0 as *mut i32;
    let mut buf: [i32; 484] = [0; 484];
    memcpy(
        buf.as_mut_ptr() as *mut core::ffi::c_void,
        S as *const core::ffi::c_void,
        (4 as i32 as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
    loop {
        nSamplesIn = if inLen < 10 as i32 * 48 as i32 {
            inLen
        } else {
            10 as i32 * 48 as i32
        };
        silk_resampler_private_AR2(
            &mut *S.offset(ORDER_FIR as isize),
            &mut *buf.as_mut_ptr().offset(ORDER_FIR as isize),
            in_0,
            silk_Resampler_2_3_COEFS_LQ.as_ptr(),
            nSamplesIn,
        );
        buf_ptr = buf.as_mut_ptr();
        counter = nSamplesIn;
        while counter > 2 as i32 {
            res_Q6 = (*buf_ptr.offset(0 as i32 as isize) as i64
                * silk_Resampler_2_3_COEFS_LQ[2 as i32 as usize] as i64
                >> 16 as i32) as i32;
            res_Q6 = (res_Q6 as i64
                + (*buf_ptr.offset(1 as i32 as isize) as i64
                    * silk_Resampler_2_3_COEFS_LQ[3 as i32 as usize] as i64
                    >> 16 as i32)) as i32;
            res_Q6 = (res_Q6 as i64
                + (*buf_ptr.offset(2 as i32 as isize) as i64
                    * silk_Resampler_2_3_COEFS_LQ[5 as i32 as usize] as i64
                    >> 16 as i32)) as i32;
            res_Q6 = (res_Q6 as i64
                + (*buf_ptr.offset(3 as i32 as isize) as i64
                    * silk_Resampler_2_3_COEFS_LQ[4 as i32 as usize] as i64
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
            res_Q6 = (*buf_ptr.offset(1 as i32 as isize) as i64
                * silk_Resampler_2_3_COEFS_LQ[4 as i32 as usize] as i64
                >> 16 as i32) as i32;
            res_Q6 = (res_Q6 as i64
                + (*buf_ptr.offset(2 as i32 as isize) as i64
                    * silk_Resampler_2_3_COEFS_LQ[5 as i32 as usize] as i64
                    >> 16 as i32)) as i32;
            res_Q6 = (res_Q6 as i64
                + (*buf_ptr.offset(3 as i32 as isize) as i64
                    * silk_Resampler_2_3_COEFS_LQ[3 as i32 as usize] as i64
                    >> 16 as i32)) as i32;
            res_Q6 = (res_Q6 as i64
                + (*buf_ptr.offset(4 as i32 as isize) as i64
                    * silk_Resampler_2_3_COEFS_LQ[2 as i32 as usize] as i64
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
            buf_ptr = buf_ptr.offset(3 as i32 as isize);
            counter -= 3 as i32;
        }
        in_0 = in_0.offset(nSamplesIn as isize);
        inLen -= nSamplesIn;
        if !(inLen > 0 as i32) {
            break;
        }
        memcpy(
            buf.as_mut_ptr() as *mut core::ffi::c_void,
            &mut *buf.as_mut_ptr().offset(nSamplesIn as isize) as *mut i32
                as *const core::ffi::c_void,
            (4 as i32 as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
        );
    }
    memcpy(
        S as *mut core::ffi::c_void,
        &mut *buf.as_mut_ptr().offset(nSamplesIn as isize) as *mut i32 as *const core::ffi::c_void,
        (4 as i32 as u64).wrapping_mul(::core::mem::size_of::<i32>() as u64),
    );
}
