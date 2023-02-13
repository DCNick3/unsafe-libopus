pub mod typedef_h {
    pub const silk_int16_MIN: i32 = 0x8000 as i32;
    pub const silk_int16_MAX: i32 = 0x7fff as i32;
}
use crate::celt::celt::celt_fatal;

pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};

use crate::externs::memset;
pub unsafe fn silk_LPC_analysis_filter(
    out: *mut i16,
    in_0: *const i16,
    B: *const i16,
    len: i32,
    d: i32,
    _arch: i32,
) {
    let mut j: i32 = 0;
    let mut ix: i32 = 0;
    let mut out32_Q12: i32 = 0;
    let mut out32: i32 = 0;
    let mut in_ptr: *const i16 = 0 as *const i16;
    if !(d >= 6 as i32) {
        celt_fatal(
            b"assertion failed: d >= 6\0" as *const u8 as *const i8,
            b"silk/LPC_analysis_filter.c\0" as *const u8 as *const i8,
            67 as i32,
        );
    }
    if !(d & 1 as i32 == 0 as i32) {
        celt_fatal(
            b"assertion failed: (d & 1) == 0\0" as *const u8 as *const i8,
            b"silk/LPC_analysis_filter.c\0" as *const u8 as *const i8,
            68 as i32,
        );
    }
    if !(d <= len) {
        celt_fatal(
            b"assertion failed: d <= len\0" as *const u8 as *const i8,
            b"silk/LPC_analysis_filter.c\0" as *const u8 as *const i8,
            69 as i32,
        );
    }
    ix = d;
    while ix < len {
        in_ptr = &*in_0.offset((ix - 1 as i32) as isize) as *const i16;
        out32_Q12 = *in_ptr.offset(0 as i32 as isize) as i32 * *B.offset(0 as i32 as isize) as i32;
        out32_Q12 = (out32_Q12 as u32).wrapping_add(
            (*in_ptr.offset(-(1 as i32) as isize) as i32 * *B.offset(1 as i32 as isize) as i32)
                as u32,
        ) as i32;
        out32_Q12 = (out32_Q12 as u32).wrapping_add(
            (*in_ptr.offset(-(2 as i32) as isize) as i32 * *B.offset(2 as i32 as isize) as i32)
                as u32,
        ) as i32;
        out32_Q12 = (out32_Q12 as u32).wrapping_add(
            (*in_ptr.offset(-(3 as i32) as isize) as i32 * *B.offset(3 as i32 as isize) as i32)
                as u32,
        ) as i32;
        out32_Q12 = (out32_Q12 as u32).wrapping_add(
            (*in_ptr.offset(-(4 as i32) as isize) as i32 * *B.offset(4 as i32 as isize) as i32)
                as u32,
        ) as i32;
        out32_Q12 = (out32_Q12 as u32).wrapping_add(
            (*in_ptr.offset(-(5 as i32) as isize) as i32 * *B.offset(5 as i32 as isize) as i32)
                as u32,
        ) as i32;
        j = 6 as i32;
        while j < d {
            out32_Q12 = (out32_Q12 as u32).wrapping_add(
                (*in_ptr.offset(-j as isize) as i32 * *B.offset(j as isize) as i32) as u32,
            ) as i32;
            out32_Q12 = (out32_Q12 as u32).wrapping_add(
                (*in_ptr.offset((-j - 1 as i32) as isize) as i32
                    * *B.offset((j + 1 as i32) as isize) as i32) as u32,
            ) as i32;
            j += 2 as i32;
        }
        out32_Q12 = (((*in_ptr.offset(1 as i32 as isize) as i32 as u32) << 12 as i32) as i32 as u32)
            .wrapping_sub(out32_Q12 as u32) as i32;
        out32 = if 12 as i32 == 1 as i32 {
            (out32_Q12 >> 1 as i32) + (out32_Q12 & 1 as i32)
        } else {
            (out32_Q12 >> 12 as i32 - 1 as i32) + 1 as i32 >> 1 as i32
        };
        *out.offset(ix as isize) = (if out32 > silk_int16_MAX {
            silk_int16_MAX
        } else if out32 < silk_int16_MIN {
            silk_int16_MIN
        } else {
            out32
        }) as i16;
        ix += 1;
    }
    memset(
        out as *mut core::ffi::c_void,
        0 as i32,
        (d as u64).wrapping_mul(::core::mem::size_of::<i16>() as u64),
    );
}
