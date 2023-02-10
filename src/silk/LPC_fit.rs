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
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/SigProc_FIX.h:32"]
pub mod SigProc_FIX_h {
    extern "C" {
        #[c2rust::src_loc = "140:1"]
        pub fn silk_bwexpander_32(ar: *mut i32, d: libc::c_int, chirp_Q16: i32);
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/typedef.h:32"]
pub mod typedef_h {
    #[c2rust::src_loc = "45:9"]
    pub const silk_int16_MIN: libc::c_int = 0x8000 as libc::c_int;
    #[c2rust::src_loc = "44:9"]
    pub const silk_int16_MAX: libc::c_int = 0x7fff as libc::c_int;
}
pub use self::stdint_intn_h::{int16_t, int32_t, int64_t};
pub use self::stdint_uintn_h::uint32_t;
pub use self::typedef_h::{silk_int16_MAX, silk_int16_MIN};
pub use self::types_h::{__int16_t, __int32_t, __int64_t, __uint32_t};
use self::SigProc_FIX_h::silk_bwexpander_32;
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_LPC_fit(
    a_QOUT: *mut i16,
    a_QIN: *mut i32,
    QOUT: libc::c_int,
    QIN: libc::c_int,
    d: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut idx: libc::c_int = 0 as libc::c_int;
    let mut maxabs: i32 = 0;
    let mut absval: i32 = 0;
    let mut chirp_Q16: i32 = 0;
    i = 0 as libc::c_int;
    while i < 10 as libc::c_int {
        maxabs = 0 as libc::c_int;
        k = 0 as libc::c_int;
        while k < d {
            absval = if *a_QIN.offset(k as isize) > 0 as libc::c_int {
                *a_QIN.offset(k as isize)
            } else {
                -*a_QIN.offset(k as isize)
            };
            if absval > maxabs {
                maxabs = absval;
                idx = k;
            }
            k += 1;
        }
        maxabs = if QIN - QOUT == 1 as libc::c_int {
            (maxabs >> 1 as libc::c_int) + (maxabs & 1 as libc::c_int)
        } else {
            (maxabs >> QIN - QOUT - 1 as libc::c_int) + 1 as libc::c_int >> 1 as libc::c_int
        };
        if !(maxabs > silk_int16_MAX) {
            break;
        }
        maxabs = if maxabs < 163838 as libc::c_int {
            maxabs
        } else {
            163838 as libc::c_int
        };
        chirp_Q16 = (0.999f64 * ((1 as libc::c_int as i64) << 16 as libc::c_int) as libc::c_double
            + 0.5f64) as i32
            - (((maxabs - 0x7fff as libc::c_int) as u32) << 14 as libc::c_int) as i32
                / (maxabs * (idx + 1 as libc::c_int) >> 2 as libc::c_int);
        silk_bwexpander_32(a_QIN, d, chirp_Q16);
        i += 1;
    }
    if i == 10 as libc::c_int {
        k = 0 as libc::c_int;
        while k < d {
            *a_QOUT.offset(k as isize) = (if (if QIN - QOUT == 1 as libc::c_int {
                (*a_QIN.offset(k as isize) >> 1 as libc::c_int)
                    + (*a_QIN.offset(k as isize) & 1 as libc::c_int)
            } else {
                (*a_QIN.offset(k as isize) >> QIN - QOUT - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) > silk_int16_MAX
            {
                silk_int16_MAX
            } else if (if QIN - QOUT == 1 as libc::c_int {
                (*a_QIN.offset(k as isize) >> 1 as libc::c_int)
                    + (*a_QIN.offset(k as isize) & 1 as libc::c_int)
            } else {
                (*a_QIN.offset(k as isize) >> QIN - QOUT - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) < silk_int16_MIN
            {
                silk_int16_MIN
            } else if QIN - QOUT == 1 as libc::c_int {
                (*a_QIN.offset(k as isize) >> 1 as libc::c_int)
                    + (*a_QIN.offset(k as isize) & 1 as libc::c_int)
            } else {
                (*a_QIN.offset(k as isize) >> QIN - QOUT - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) as i16;
            *a_QIN.offset(k as isize) =
                ((*a_QOUT.offset(k as isize) as i32 as u32) << QIN - QOUT) as i32;
            k += 1;
        }
    } else {
        k = 0 as libc::c_int;
        while k < d {
            *a_QOUT.offset(k as isize) = (if QIN - QOUT == 1 as libc::c_int {
                (*a_QIN.offset(k as isize) >> 1 as libc::c_int)
                    + (*a_QIN.offset(k as isize) & 1 as libc::c_int)
            } else {
                (*a_QIN.offset(k as isize) >> QIN - QOUT - 1 as libc::c_int) + 1 as libc::c_int
                    >> 1 as libc::c_int
            }) as i16;
            k += 1;
        }
    };
}
