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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/resampler_structs.h:32"]
pub mod resampler_structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "38:16"]
    pub struct _silk_resampler_state_struct {
        pub sIIR: [i32; 6],
        pub sFIR: C2RustUnnamed,
        pub delayBuf: [i16; 48],
        pub resampler_function: libc::c_int,
        pub batchSize: libc::c_int,
        pub invRatio_Q16: i32,
        pub FIR_Order: libc::c_int,
        pub FIR_Fracs: libc::c_int,
        pub Fs_in_kHz: libc::c_int,
        pub Fs_out_kHz: libc::c_int,
        pub inputDelay: libc::c_int,
        pub Coefs: *const i16,
    }
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "40:5"]
    pub union C2RustUnnamed {
        pub i32_0: [i32; 36],
        pub i16_0: [i16; 36],
    }
    #[c2rust::src_loc = "38:1"]
    pub type silk_resampler_state_struct = _silk_resampler_state_struct;
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
    #[c2rust::src_loc = "49:25"]
    pub static mut silk_resampler_up2_hq_0: [i16; 3] = [
        1746 as libc::c_int as i16,
        14986 as libc::c_int as i16,
        (39083 as libc::c_int - 65536 as libc::c_int) as i16,
    ];
    #[c2rust::src_loc = "50:25"]
    pub static mut silk_resampler_up2_hq_1: [i16; 3] = [
        6854 as libc::c_int as i16,
        25769 as libc::c_int as i16,
        (55542 as libc::c_int - 65536 as libc::c_int) as i16,
    ];
}
pub use self::resampler_rom_h::{silk_resampler_up2_hq_0, silk_resampler_up2_hq_1};
pub use self::resampler_structs_h::{
    _silk_resampler_state_struct, silk_resampler_state_struct, C2RustUnnamed,
};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __uint32_t};
#[no_mangle]
#[c2rust::src_loc = "38:1"]
pub unsafe extern "C" fn silk_resampler_private_up2_HQ(
    S: *mut i32,
    out: *mut i16,
    in_0: *const i16,
    len: i32,
) {
    let mut k: i32 = 0;
    let mut in32: i32 = 0;
    let mut out32_1: i32 = 0;
    let mut out32_2: i32 = 0;
    let mut Y: i32 = 0;
    let mut X: i32 = 0;
    k = 0 as libc::c_int;
    while k < len {
        in32 = ((*in_0.offset(k as isize) as i32 as u32) << 10 as libc::c_int) as i32;
        Y = in32 - *S.offset(0 as libc::c_int as isize);
        X = (Y as libc::c_long * silk_resampler_up2_hq_0[0 as libc::c_int as usize] as i64
            >> 16 as libc::c_int) as i32;
        out32_1 = *S.offset(0 as libc::c_int as isize) + X;
        *S.offset(0 as libc::c_int as isize) = in32 + X;
        Y = out32_1 - *S.offset(1 as libc::c_int as isize);
        X = (Y as libc::c_long * silk_resampler_up2_hq_0[1 as libc::c_int as usize] as i64
            >> 16 as libc::c_int) as i32;
        out32_2 = *S.offset(1 as libc::c_int as isize) + X;
        *S.offset(1 as libc::c_int as isize) = out32_1 + X;
        Y = out32_2 - *S.offset(2 as libc::c_int as isize);
        X = (Y as libc::c_long
            + (Y as libc::c_long * silk_resampler_up2_hq_0[2 as libc::c_int as usize] as i64
                >> 16 as libc::c_int)) as i32;
        out32_1 = *S.offset(2 as libc::c_int as isize) + X;
        *S.offset(2 as libc::c_int as isize) = out32_2 + X;
        *out.offset((2 as libc::c_int * k) as isize) = (if (if 10 as libc::c_int == 1 as libc::c_int
        {
            (out32_1 >> 1 as libc::c_int) + (out32_1 & 1 as libc::c_int)
        } else {
            (out32_1 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 10 as libc::c_int == 1 as libc::c_int {
            (out32_1 >> 1 as libc::c_int) + (out32_1 & 1 as libc::c_int)
        } else {
            (out32_1 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 10 as libc::c_int == 1 as libc::c_int {
            (out32_1 >> 1 as libc::c_int) + (out32_1 & 1 as libc::c_int)
        } else {
            (out32_1 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as i16;
        Y = in32 - *S.offset(3 as libc::c_int as isize);
        X = (Y as libc::c_long * silk_resampler_up2_hq_1[0 as libc::c_int as usize] as i64
            >> 16 as libc::c_int) as i32;
        out32_1 = *S.offset(3 as libc::c_int as isize) + X;
        *S.offset(3 as libc::c_int as isize) = in32 + X;
        Y = out32_1 - *S.offset(4 as libc::c_int as isize);
        X = (Y as libc::c_long * silk_resampler_up2_hq_1[1 as libc::c_int as usize] as i64
            >> 16 as libc::c_int) as i32;
        out32_2 = *S.offset(4 as libc::c_int as isize) + X;
        *S.offset(4 as libc::c_int as isize) = out32_1 + X;
        Y = out32_2 - *S.offset(5 as libc::c_int as isize);
        X = (Y as libc::c_long
            + (Y as libc::c_long * silk_resampler_up2_hq_1[2 as libc::c_int as usize] as i64
                >> 16 as libc::c_int)) as i32;
        out32_1 = *S.offset(5 as libc::c_int as isize) + X;
        *S.offset(5 as libc::c_int as isize) = out32_2 + X;
        *out.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize) = (if (if 10 as libc::c_int
            == 1 as libc::c_int
        {
            (out32_1 >> 1 as libc::c_int) + (out32_1 & 1 as libc::c_int)
        } else {
            (out32_1 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) > silk_int16_MAX
        {
            silk_int16_MAX
        } else if (if 10 as libc::c_int == 1 as libc::c_int {
            (out32_1 >> 1 as libc::c_int) + (out32_1 & 1 as libc::c_int)
        } else {
            (out32_1 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) < silk_int16_MIN
        {
            silk_int16_MIN
        } else if 10 as libc::c_int == 1 as libc::c_int {
            (out32_1 >> 1 as libc::c_int) + (out32_1 & 1 as libc::c_int)
        } else {
            (out32_1 >> 10 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as i16;
        k += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "104:1"]
pub unsafe extern "C" fn silk_resampler_private_up2_HQ_wrapper(
    SS: *mut libc::c_void,
    out: *mut i16,
    in_0: *const i16,
    len: i32,
) {
    let S: *mut silk_resampler_state_struct = SS as *mut silk_resampler_state_struct;
    silk_resampler_private_up2_HQ(((*S).sIIR).as_mut_ptr(), out, in_0, len);
}
