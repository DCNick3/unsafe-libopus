use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "44:1"]
    pub type __int64_t = libc::c_long;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    #[c2rust::src_loc = "26:1"]
    pub type int32_t = __int32_t;
    #[c2rust::src_loc = "27:1"]
    pub type int64_t = __int64_t;
    use super::types_h::{__int16_t, __int32_t, __int64_t};
}
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_rom.h:33"]
pub mod resampler_rom_h {
    extern "C" {
        #[c2rust::src_loc = "59:25"]
        pub static silk_Resampler_2_3_COEFS_LQ: [i16; 6];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_private.h:33"]
pub mod resampler_private_h {
    extern "C" {
        #[c2rust::src_loc = "77:1"]
        pub fn silk_resampler_private_AR2(
            S: *mut i32,
            out_Q8: *mut i32,
            in_0: *const i16,
            A_Q14: *const i16,
            len: i32,
        );
    }
}
use self::resampler_private_h::silk_resampler_private_AR2;
use self::resampler_rom_h::silk_Resampler_2_3_COEFS_LQ;
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
use self::string_h::memcpy;
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
pub use self::types_h::{__int16_t, __int32_t, __int64_t};
#[c2rust::src_loc = "36:9"]
pub const ORDER_FIR: libc::c_int = 4 as libc::c_int;
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn silk_resampler_down2_3(
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
        buf.as_mut_ptr() as *mut libc::c_void,
        S as *const libc::c_void,
        (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
    );
    loop {
        nSamplesIn = if inLen < 10 as libc::c_int * 48 as libc::c_int {
            inLen
        } else {
            10 as libc::c_int * 48 as libc::c_int
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
        while counter > 2 as libc::c_int {
            res_Q6 = (*buf_ptr.offset(0 as libc::c_int as isize) as libc::c_long
                * silk_Resampler_2_3_COEFS_LQ[2 as libc::c_int as usize] as i64
                >> 16 as libc::c_int) as i32;
            res_Q6 = (res_Q6 as libc::c_long
                + (*buf_ptr.offset(1 as libc::c_int as isize) as libc::c_long
                    * silk_Resampler_2_3_COEFS_LQ[3 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            res_Q6 = (res_Q6 as libc::c_long
                + (*buf_ptr.offset(2 as libc::c_int as isize) as libc::c_long
                    * silk_Resampler_2_3_COEFS_LQ[5 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            res_Q6 = (res_Q6 as libc::c_long
                + (*buf_ptr.offset(3 as libc::c_int as isize) as libc::c_long
                    * silk_Resampler_2_3_COEFS_LQ[4 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            let fresh0 = out;
            out = out.offset(1);
            *fresh0 = (if (if 6 as libc::c_int == 1 as libc::c_int {
                (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
            } else {
                (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) > silk_int16_MAX
            {
                silk_int16_MAX
            } else if (if 6 as libc::c_int == 1 as libc::c_int {
                (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
            } else {
                (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) < silk_int16_MIN
            {
                silk_int16_MIN
            } else if 6 as libc::c_int == 1 as libc::c_int {
                (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
            } else {
                (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) as i16;
            res_Q6 = (*buf_ptr.offset(1 as libc::c_int as isize) as libc::c_long
                * silk_Resampler_2_3_COEFS_LQ[4 as libc::c_int as usize] as i64
                >> 16 as libc::c_int) as i32;
            res_Q6 = (res_Q6 as libc::c_long
                + (*buf_ptr.offset(2 as libc::c_int as isize) as libc::c_long
                    * silk_Resampler_2_3_COEFS_LQ[5 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            res_Q6 = (res_Q6 as libc::c_long
                + (*buf_ptr.offset(3 as libc::c_int as isize) as libc::c_long
                    * silk_Resampler_2_3_COEFS_LQ[3 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            res_Q6 = (res_Q6 as libc::c_long
                + (*buf_ptr.offset(4 as libc::c_int as isize) as libc::c_long
                    * silk_Resampler_2_3_COEFS_LQ[2 as libc::c_int as usize] as i64
                    >> 16 as libc::c_int)) as i32;
            let fresh1 = out;
            out = out.offset(1);
            *fresh1 = (if (if 6 as libc::c_int == 1 as libc::c_int {
                (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
            } else {
                (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) > silk_int16_MAX
            {
                silk_int16_MAX
            } else if (if 6 as libc::c_int == 1 as libc::c_int {
                (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
            } else {
                (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) < silk_int16_MIN
            {
                silk_int16_MIN
            } else if 6 as libc::c_int == 1 as libc::c_int {
                (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
            } else {
                (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) as i16;
            buf_ptr = buf_ptr.offset(3 as libc::c_int as isize);
            counter -= 3 as libc::c_int;
        }
        in_0 = in_0.offset(nSamplesIn as isize);
        inLen -= nSamplesIn;
        if !(inLen > 0 as libc::c_int) {
            break;
        }
        memcpy(
            buf.as_mut_ptr() as *mut libc::c_void,
            &mut *buf.as_mut_ptr().offset(nSamplesIn as isize) as *mut i32 as *const libc::c_void,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
        );
    }
    memcpy(
        S as *mut libc::c_void,
        &mut *buf.as_mut_ptr().offset(nSamplesIn as isize) as *mut i32 as *const libc::c_void,
        (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<i32>() as libc::c_ulong),
    );
}
