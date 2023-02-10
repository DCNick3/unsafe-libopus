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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "57:4"]
    pub type opus_int64 = int64_t;
    use super::stdint_intn_h::{int16_t, int32_t, int64_t};
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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_rom.h:33"]
pub mod resampler_rom_h {
    use super::opus_types_h::opus_int16;
    extern "C" {
        #[c2rust::src_loc = "59:25"]
        pub static silk_Resampler_2_3_COEFS_LQ: [opus_int16; 6];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_private.h:33"]
pub mod resampler_private_h {
    use super::opus_types_h::{opus_int16, opus_int32};
    extern "C" {
        #[c2rust::src_loc = "77:1"]
        pub fn silk_resampler_private_AR2(
            S: *mut opus_int32,
            out_Q8: *mut opus_int32,
            in_0: *const opus_int16,
            A_Q14: *const opus_int16,
            len: opus_int32,
        );
    }
}
pub use self::opus_types_h::{opus_int16, opus_int32, opus_int64};
use self::resampler_private_h::silk_resampler_private_AR2;
use self::resampler_rom_h::silk_Resampler_2_3_COEFS_LQ;
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
use self::string_h::memcpy;
pub use self::types_h::{__int16_t, __int32_t, __int64_t};
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn silk_resampler_down2_3(
    mut S: *mut opus_int32,
    mut out: *mut opus_int16,
    mut in_0: *const opus_int16,
    mut inLen: opus_int32,
) {
    let mut nSamplesIn: opus_int32 = 0;
    let mut counter: opus_int32 = 0;
    let mut res_Q6: opus_int32 = 0;
    let mut buf_ptr: *mut opus_int32 = 0 as *mut opus_int32;
    let mut buf: [opus_int32; 484] = [0; 484];
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        S as *const libc::c_void,
        (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int32>() as libc::c_ulong),
    );
    loop {
        nSamplesIn = if inLen < 10 as libc::c_int * 48 as libc::c_int {
            inLen
        } else {
            10 as libc::c_int * 48 as libc::c_int
        };
        silk_resampler_private_AR2(
            &mut *S.offset(4 as libc::c_int as isize),
            &mut *buf.as_mut_ptr().offset(4 as libc::c_int as isize),
            in_0,
            silk_Resampler_2_3_COEFS_LQ.as_ptr(),
            nSamplesIn,
        );
        buf_ptr = buf.as_mut_ptr();
        counter = nSamplesIn;
        while counter > 2 as libc::c_int {
            res_Q6 = (*buf_ptr.offset(0 as libc::c_int as isize) as libc::c_long
                * silk_Resampler_2_3_COEFS_LQ[2 as libc::c_int as usize] as opus_int64
                >> 16 as libc::c_int) as opus_int32;
            res_Q6 = (res_Q6 as libc::c_long
                + (*buf_ptr.offset(1 as libc::c_int as isize) as libc::c_long
                    * silk_Resampler_2_3_COEFS_LQ[3 as libc::c_int as usize] as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            res_Q6 = (res_Q6 as libc::c_long
                + (*buf_ptr.offset(2 as libc::c_int as isize) as libc::c_long
                    * silk_Resampler_2_3_COEFS_LQ[5 as libc::c_int as usize] as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            res_Q6 = (res_Q6 as libc::c_long
                + (*buf_ptr.offset(3 as libc::c_int as isize) as libc::c_long
                    * silk_Resampler_2_3_COEFS_LQ[4 as libc::c_int as usize] as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            let fresh0 = out;
            out = out.offset(1);
            *fresh0 = (if (if 6 as libc::c_int == 1 as libc::c_int {
                (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
            } else {
                (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) > 0x7fff as libc::c_int
            {
                0x7fff as libc::c_int
            } else if (if 6 as libc::c_int == 1 as libc::c_int {
                (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
            } else {
                (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) < 0x8000 as libc::c_int as opus_int16 as libc::c_int
            {
                0x8000 as libc::c_int as opus_int16 as libc::c_int
            } else if 6 as libc::c_int == 1 as libc::c_int {
                (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
            } else {
                (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) as opus_int16;
            res_Q6 = (*buf_ptr.offset(1 as libc::c_int as isize) as libc::c_long
                * silk_Resampler_2_3_COEFS_LQ[4 as libc::c_int as usize] as opus_int64
                >> 16 as libc::c_int) as opus_int32;
            res_Q6 = (res_Q6 as libc::c_long
                + (*buf_ptr.offset(2 as libc::c_int as isize) as libc::c_long
                    * silk_Resampler_2_3_COEFS_LQ[5 as libc::c_int as usize] as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            res_Q6 = (res_Q6 as libc::c_long
                + (*buf_ptr.offset(3 as libc::c_int as isize) as libc::c_long
                    * silk_Resampler_2_3_COEFS_LQ[3 as libc::c_int as usize] as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            res_Q6 = (res_Q6 as libc::c_long
                + (*buf_ptr.offset(4 as libc::c_int as isize) as libc::c_long
                    * silk_Resampler_2_3_COEFS_LQ[2 as libc::c_int as usize] as opus_int64
                    >> 16 as libc::c_int)) as opus_int32;
            let fresh1 = out;
            out = out.offset(1);
            *fresh1 = (if (if 6 as libc::c_int == 1 as libc::c_int {
                (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
            } else {
                (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) > 0x7fff as libc::c_int
            {
                0x7fff as libc::c_int
            } else if (if 6 as libc::c_int == 1 as libc::c_int {
                (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
            } else {
                (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) < 0x8000 as libc::c_int as opus_int16 as libc::c_int
            {
                0x8000 as libc::c_int as opus_int16 as libc::c_int
            } else if 6 as libc::c_int == 1 as libc::c_int {
                (res_Q6 >> 1 as libc::c_int) + (res_Q6 & 1 as libc::c_int)
            } else {
                (res_Q6 >> 6 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) as opus_int16;
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
            &mut *buf.as_mut_ptr().offset(nSamplesIn as isize) as *mut opus_int32
                as *const libc::c_void,
            (4 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_int32>() as libc::c_ulong),
        );
    }
    memcpy(
        S as *mut libc::c_void,
        &mut *buf.as_mut_ptr().offset(nSamplesIn as isize) as *mut opus_int32
            as *const libc::c_void,
        (4 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int32>() as libc::c_ulong),
    );
}
