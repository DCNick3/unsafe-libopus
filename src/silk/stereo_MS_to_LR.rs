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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/structs.h:32"]
pub mod structs_h {
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "112:9"]
    pub struct stereo_dec_state {
        pub pred_prev_Q13: [opus_int16; 2],
        pub sMid: [opus_int16; 2],
        pub sSide: [opus_int16; 2],
    }
    use super::opus_types_h::opus_int16;
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
pub use self::opus_types_h::{opus_int16, opus_int32, opus_int64, opus_uint32};
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::uint32_t;
use self::string_h::memcpy;
pub use self::structs_h::stereo_dec_state;
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __uint32_t};
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_stereo_MS_to_LR(
    mut state: *mut stereo_dec_state,
    mut x1: *mut opus_int16,
    mut x2: *mut opus_int16,
    mut pred_Q13: *const opus_int32,
    mut fs_kHz: libc::c_int,
    mut frame_length: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut denom_Q16: libc::c_int = 0;
    let mut delta0_Q13: libc::c_int = 0;
    let mut delta1_Q13: libc::c_int = 0;
    let mut sum: opus_int32 = 0;
    let mut diff: opus_int32 = 0;
    let mut pred0_Q13: opus_int32 = 0;
    let mut pred1_Q13: opus_int32 = 0;
    memcpy(
        x1 as *mut libc::c_void,
        ((*state).sMid).as_mut_ptr() as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
    );
    memcpy(
        x2 as *mut libc::c_void,
        ((*state).sSide).as_mut_ptr() as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
    );
    memcpy(
        ((*state).sMid).as_mut_ptr() as *mut libc::c_void,
        &mut *x1.offset(frame_length as isize) as *mut opus_int16 as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
    );
    memcpy(
        ((*state).sSide).as_mut_ptr() as *mut libc::c_void,
        &mut *x2.offset(frame_length as isize) as *mut opus_int16 as *const libc::c_void,
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_int16>() as libc::c_ulong),
    );
    pred0_Q13 = (*state).pred_prev_Q13[0 as libc::c_int as usize] as opus_int32;
    pred1_Q13 = (*state).pred_prev_Q13[1 as libc::c_int as usize] as opus_int32;
    denom_Q16 = ((1 as libc::c_int) << 16 as libc::c_int) / (8 as libc::c_int * fs_kHz);
    delta0_Q13 = if 16 as libc::c_int == 1 as libc::c_int {
        ((*pred_Q13.offset(0 as libc::c_int as isize)
            - (*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int)
            as opus_int16 as opus_int32
            * denom_Q16 as opus_int16 as opus_int32
            >> 1 as libc::c_int)
            + ((*pred_Q13.offset(0 as libc::c_int as isize)
                - (*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int)
                as opus_int16 as opus_int32
                * denom_Q16 as opus_int16 as opus_int32
                & 1 as libc::c_int)
    } else {
        ((*pred_Q13.offset(0 as libc::c_int as isize)
            - (*state).pred_prev_Q13[0 as libc::c_int as usize] as libc::c_int)
            as opus_int16 as opus_int32
            * denom_Q16 as opus_int16 as opus_int32
            >> 16 as libc::c_int - 1 as libc::c_int)
            + 1 as libc::c_int
            >> 1 as libc::c_int
    };
    delta1_Q13 = if 16 as libc::c_int == 1 as libc::c_int {
        ((*pred_Q13.offset(1 as libc::c_int as isize)
            - (*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int)
            as opus_int16 as opus_int32
            * denom_Q16 as opus_int16 as opus_int32
            >> 1 as libc::c_int)
            + ((*pred_Q13.offset(1 as libc::c_int as isize)
                - (*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int)
                as opus_int16 as opus_int32
                * denom_Q16 as opus_int16 as opus_int32
                & 1 as libc::c_int)
    } else {
        ((*pred_Q13.offset(1 as libc::c_int as isize)
            - (*state).pred_prev_Q13[1 as libc::c_int as usize] as libc::c_int)
            as opus_int16 as opus_int32
            * denom_Q16 as opus_int16 as opus_int32
            >> 16 as libc::c_int - 1 as libc::c_int)
            + 1 as libc::c_int
            >> 1 as libc::c_int
    };
    n = 0 as libc::c_int;
    while n < 8 as libc::c_int * fs_kHz {
        pred0_Q13 += delta0_Q13;
        pred1_Q13 += delta1_Q13;
        sum = (((*x1.offset(n as isize) as libc::c_int
            + *x1.offset((n + 2 as libc::c_int) as isize) as libc::c_int
            + ((*x1.offset((n + 1 as libc::c_int) as isize) as opus_uint32) << 1 as libc::c_int)
                as opus_int32) as opus_uint32)
            << 9 as libc::c_int) as opus_int32;
        sum = (((*x2.offset((n + 1 as libc::c_int) as isize) as opus_int32 as opus_uint32)
            << 8 as libc::c_int) as opus_int32 as libc::c_long
            + (sum as libc::c_long * pred0_Q13 as opus_int16 as opus_int64 >> 16 as libc::c_int))
            as opus_int32;
        sum = (sum as libc::c_long
            + (((*x1.offset((n + 1 as libc::c_int) as isize) as opus_int32 as opus_uint32)
                << 11 as libc::c_int) as opus_int32 as libc::c_long
                * pred1_Q13 as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *x2.offset((n + 1 as libc::c_int) as isize) = (if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as opus_int16 as libc::c_int
        {
            0x8000 as libc::c_int as opus_int16 as libc::c_int
        } else if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as opus_int16;
        n += 1;
    }
    pred0_Q13 = *pred_Q13.offset(0 as libc::c_int as isize);
    pred1_Q13 = *pred_Q13.offset(1 as libc::c_int as isize);
    n = 8 as libc::c_int * fs_kHz;
    while n < frame_length {
        sum = (((*x1.offset(n as isize) as libc::c_int
            + *x1.offset((n + 2 as libc::c_int) as isize) as libc::c_int
            + ((*x1.offset((n + 1 as libc::c_int) as isize) as opus_uint32) << 1 as libc::c_int)
                as opus_int32) as opus_uint32)
            << 9 as libc::c_int) as opus_int32;
        sum = (((*x2.offset((n + 1 as libc::c_int) as isize) as opus_int32 as opus_uint32)
            << 8 as libc::c_int) as opus_int32 as libc::c_long
            + (sum as libc::c_long * pred0_Q13 as opus_int16 as opus_int64 >> 16 as libc::c_int))
            as opus_int32;
        sum = (sum as libc::c_long
            + (((*x1.offset((n + 1 as libc::c_int) as isize) as opus_int32 as opus_uint32)
                << 11 as libc::c_int) as opus_int32 as libc::c_long
                * pred1_Q13 as opus_int16 as opus_int64
                >> 16 as libc::c_int)) as opus_int32;
        *x2.offset((n + 1 as libc::c_int) as isize) = (if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) > 0x7fff as libc::c_int
        {
            0x7fff as libc::c_int
        } else if (if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) < 0x8000 as libc::c_int as opus_int16 as libc::c_int
        {
            0x8000 as libc::c_int as opus_int16 as libc::c_int
        } else if 8 as libc::c_int == 1 as libc::c_int {
            (sum >> 1 as libc::c_int) + (sum & 1 as libc::c_int)
        } else {
            (sum >> 8 as libc::c_int - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        }) as opus_int16;
        n += 1;
    }
    (*state).pred_prev_Q13[0 as libc::c_int as usize] =
        *pred_Q13.offset(0 as libc::c_int as isize) as opus_int16;
    (*state).pred_prev_Q13[1 as libc::c_int as usize] =
        *pred_Q13.offset(1 as libc::c_int as isize) as opus_int16;
    n = 0 as libc::c_int;
    while n < frame_length {
        sum = *x1.offset((n + 1 as libc::c_int) as isize) as libc::c_int
            + *x2.offset((n + 1 as libc::c_int) as isize) as opus_int32;
        diff = *x1.offset((n + 1 as libc::c_int) as isize) as libc::c_int
            - *x2.offset((n + 1 as libc::c_int) as isize) as opus_int32;
        *x1.offset((n + 1 as libc::c_int) as isize) = (if sum > 0x7fff as libc::c_int {
            0x7fff as libc::c_int
        } else if sum < 0x8000 as libc::c_int as opus_int16 as libc::c_int {
            0x8000 as libc::c_int as opus_int16 as libc::c_int
        } else {
            sum
        }) as opus_int16;
        *x2.offset((n + 1 as libc::c_int) as isize) = (if diff > 0x7fff as libc::c_int {
            0x7fff as libc::c_int
        } else if diff < 0x8000 as libc::c_int as opus_int16 as libc::c_int {
            0x8000 as libc::c_int as opus_int16 as libc::c_int
        } else {
            diff
        }) as opus_int16;
        n += 1;
    }
}
