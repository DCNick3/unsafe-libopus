use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "41:1"]
    pub type __int32_t = libc::c_int;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
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
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::__uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_types.h:32"]
pub mod opus_types_h {
    #[c2rust::src_loc = "53:4"]
    pub type opus_int16 = int16_t;
    #[c2rust::src_loc = "55:4"]
    pub type opus_int32 = int32_t;
    #[c2rust::src_loc = "56:4"]
    pub type opus_uint32 = uint32_t;
    #[c2rust::src_loc = "57:4"]
    pub type opus_int64 = int64_t;
    use super::stdint_intn_h::{int16_t, int32_t, int64_t};
    use super::stdint_uintn_h::uint32_t;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:32"]
pub mod resampler_structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct _silk_resampler_state_struct {
        pub sIIR: [opus_int32; 6],
        pub sFIR: C2RustUnnamed,
        pub delayBuf: [opus_int16; 48],
        pub resampler_function: libc::c_int,
        pub batchSize: libc::c_int,
        pub invRatio_Q16: opus_int32,
        pub FIR_Order: libc::c_int,
        pub FIR_Fracs: libc::c_int,
        pub Fs_in_kHz: libc::c_int,
        pub Fs_out_kHz: libc::c_int,
        pub inputDelay: libc::c_int,
        pub Coefs: *const opus_int16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:5"]
    pub union C2RustUnnamed {
        pub i32_0: [opus_int32; 36],
        pub i16_0: [opus_int16; 36],
    }
    #[c2rust::src_loc = "38:1"]
    pub type silk_resampler_state_struct = _silk_resampler_state_struct;
    use super::opus_types_h::{opus_int16, opus_int32};
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
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_rom.h:33"]
pub mod resampler_rom_h {
    #[c2rust::src_loc = "42:9"]
    pub const RESAMPLER_ORDER_FIR_12: libc::c_int = 8 as libc::c_int;
    use super::opus_types_h::opus_int16;
    extern "C" {
        #[c2rust::src_loc = "62:25"]
        pub static silk_resampler_frac_FIR_12: [[opus_int16; 4]; 12];
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_private.h:33"]
pub mod resampler_private_h {
    use super::opus_types_h::{opus_int16, opus_int32};
    extern "C" {
        #[c2rust::src_loc = "69:1"]
        pub fn silk_resampler_private_up2_HQ(
            S: *mut opus_int32,
            out: *mut opus_int16,
            in_0: *const opus_int16,
            len: opus_int32,
        );
    }
}
pub use self::opus_types_h::{opus_int16, opus_int32, opus_int64, opus_uint32};
use self::resampler_private_h::silk_resampler_private_up2_HQ;
pub use self::resampler_rom_h::{silk_resampler_frac_FIR_12, RESAMPLER_ORDER_FIR_12};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::uint32_t;
use self::string_h::memcpy;
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __uint32_t};
#[inline]
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn silk_resampler_private_IIR_FIR_INTERPOL(
    mut out: *mut opus_int16,
    buf: *mut opus_int16,
    max_index_Q16: opus_int32,
    index_increment_Q16: opus_int32,
) -> *mut opus_int16 {
    let mut index_Q16: opus_int32 = 0;
    let mut res_Q15: opus_int32 = 0;
    let mut buf_ptr: *mut opus_int16 = 0 as *mut opus_int16;
    let mut table_index: opus_int32 = 0;
    index_Q16 = 0 as libc::c_int;
    while index_Q16 < max_index_Q16 {
        table_index = ((index_Q16 & 0xffff as libc::c_int) as libc::c_long
            * 12 as libc::c_int as opus_int16 as opus_int64
            >> 16 as libc::c_int) as opus_int32;
        buf_ptr = &mut *buf.offset((index_Q16 >> 16 as libc::c_int) as isize) as *mut opus_int16;
        res_Q15 = *buf_ptr.offset(0 as libc::c_int as isize) as opus_int32
            * silk_resampler_frac_FIR_12[table_index as usize][0 as libc::c_int as usize]
                as opus_int32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(1 as libc::c_int as isize) as opus_int32
                * silk_resampler_frac_FIR_12[table_index as usize][1 as libc::c_int as usize]
                    as opus_int32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(2 as libc::c_int as isize) as opus_int32
                * silk_resampler_frac_FIR_12[table_index as usize][2 as libc::c_int as usize]
                    as opus_int32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(3 as libc::c_int as isize) as opus_int32
                * silk_resampler_frac_FIR_12[table_index as usize][3 as libc::c_int as usize]
                    as opus_int32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(4 as libc::c_int as isize) as opus_int32
                * silk_resampler_frac_FIR_12[(11 as libc::c_int - table_index) as usize]
                    [3 as libc::c_int as usize] as opus_int32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(5 as libc::c_int as isize) as opus_int32
                * silk_resampler_frac_FIR_12[(11 as libc::c_int - table_index) as usize]
                    [2 as libc::c_int as usize] as opus_int32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(6 as libc::c_int as isize) as opus_int32
                * silk_resampler_frac_FIR_12[(11 as libc::c_int - table_index) as usize]
                    [1 as libc::c_int as usize] as opus_int32;
        res_Q15 = res_Q15
            + *buf_ptr.offset(7 as libc::c_int as isize) as opus_int32
                * silk_resampler_frac_FIR_12[(11 as libc::c_int - table_index) as usize]
                    [0 as libc::c_int as usize] as opus_int32;
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
        }) as opus_int16;
        index_Q16 += index_increment_Q16;
    }
    return out;
}
#[no_mangle]
#[c2rust::src_loc = "65:1"]
pub unsafe extern "C" fn silk_resampler_private_IIR_FIR(
    SS: *mut libc::c_void,
    mut out: *mut opus_int16,
    mut in_0: *const opus_int16,
    mut inLen: opus_int32,
) {
    let S: *mut silk_resampler_state_struct = SS as *mut silk_resampler_state_struct;
    let mut nSamplesIn: opus_int32 = 0;
    let mut max_index_Q16: opus_int32 = 0;
    let mut index_increment_Q16: opus_int32 = 0;
    let vla = (2 as libc::c_int * (*S).batchSize + 8 as libc::c_int) as usize;
    let mut buf: Vec<opus_int16> = ::std::vec::from_elem(0, vla);
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        ((*S).sFIR.i16_0).as_mut_ptr() as *const libc::c_void,
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
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
        max_index_Q16 =
            ((nSamplesIn as opus_uint32) << 16 as libc::c_int + 1 as libc::c_int) as opus_int32;
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
                .offset((nSamplesIn << 1 as libc::c_int) as isize) as *mut opus_int16
                as *const libc::c_void,
            (8 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
        );
    }
    memcpy(
        ((*S).sFIR.i16_0).as_mut_ptr() as *mut libc::c_void,
        &mut *buf
            .as_mut_ptr()
            .offset((nSamplesIn << 1 as libc::c_int) as isize) as *mut opus_int16
            as *const libc::c_void,
        (8 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
    );
}
