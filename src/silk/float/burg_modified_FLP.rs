use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:32"]
pub mod arch_h {
    extern "C" {
        #[c2rust::src_loc = "63:1"]
        pub fn celt_fatal(
            str: *const libc::c_char,
            file: *const libc::c_char,
            line: libc::c_int,
        ) -> !;
    }
}
#[c2rust::header_src = "/usr/include/string.h:32"]
pub mod string_h {
    extern "C" {
        #[c2rust::src_loc = "61:14"]
        pub fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
        #[c2rust::src_loc = "43:14"]
        pub fn memcpy(
            _: *mut libc::c_void,
            _: *const libc::c_void,
            _: libc::c_ulong,
        ) -> *mut libc::c_void;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:32"]
pub mod SigProc_FLP_h {
    extern "C" {
        #[c2rust::src_loc = "127:1"]
        pub fn silk_inner_product_FLP(
            data1: *const libc::c_float,
            data2: *const libc::c_float,
            dataSize: libc::c_int,
        ) -> libc::c_double;
        #[c2rust::src_loc = "134:1"]
        pub fn silk_energy_FLP(data: *const libc::c_float, dataSize: libc::c_int)
            -> libc::c_double;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tuning_parameters.h:33"]
pub mod tuning_parameters_h {
    #[c2rust::src_loc = "54:9"]
    pub const FIND_LPC_COND_FAC: libc::c_float = 1e-5f32;
}
use self::arch_h::celt_fatal;
use self::string_h::{memcpy, memset};
pub use self::tuning_parameters_h::FIND_LPC_COND_FAC;
use self::SigProc_FLP_h::{silk_energy_FLP, silk_inner_product_FLP};
#[no_mangle]
#[c2rust::src_loc = "39:1"]
pub unsafe extern "C" fn silk_burg_modified_FLP(
    A: *mut libc::c_float,
    x: *const libc::c_float,
    minInvGain: libc::c_float,
    subfr_length: libc::c_int,
    nb_subfr: libc::c_int,
    D: libc::c_int,
) -> libc::c_float {
    let mut k: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut reached_max_gain: libc::c_int = 0;
    let mut C0: libc::c_double = 0.;
    let mut invGain: libc::c_double = 0.;
    let mut num: libc::c_double = 0.;
    let mut nrg_f: libc::c_double = 0.;
    let mut nrg_b: libc::c_double = 0.;
    let mut rc: libc::c_double = 0.;
    let mut Atmp: libc::c_double = 0.;
    let mut tmp1: libc::c_double = 0.;
    let mut tmp2: libc::c_double = 0.;
    let mut x_ptr: *const libc::c_float = 0 as *const libc::c_float;
    let mut C_first_row: [libc::c_double; 24] = [0.; 24];
    let mut C_last_row: [libc::c_double; 24] = [0.; 24];
    let mut CAf: [libc::c_double; 25] = [0.; 25];
    let mut CAb: [libc::c_double; 25] = [0.; 25];
    let mut Af: [libc::c_double; 24] = [0.; 24];
    if !(subfr_length * nb_subfr <= 384 as libc::c_int) {
        celt_fatal(
            b"assertion failed: subfr_length * nb_subfr <= MAX_FRAME_SIZE\0" as *const u8
                as *const libc::c_char,
            b"silk/float/burg_modified_FLP.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
        );
    }
    C0 = silk_energy_FLP(x, nb_subfr * subfr_length);
    memset(
        C_first_row.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        (24 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    s = 0 as libc::c_int;
    while s < nb_subfr {
        x_ptr = x.offset((s * subfr_length) as isize);
        n = 1 as libc::c_int;
        while n < D + 1 as libc::c_int {
            C_first_row[(n - 1 as libc::c_int) as usize] +=
                silk_inner_product_FLP(x_ptr, x_ptr.offset(n as isize), subfr_length - n);
            n += 1;
        }
        s += 1;
    }
    memcpy(
        C_last_row.as_mut_ptr() as *mut libc::c_void,
        C_first_row.as_mut_ptr() as *const libc::c_void,
        (24 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_double>() as libc::c_ulong),
    );
    CAf[0 as libc::c_int as usize] =
        C0 + FIND_LPC_COND_FAC as libc::c_double * C0 + 1e-9f32 as libc::c_double;
    CAb[0 as libc::c_int as usize] = CAf[0 as libc::c_int as usize];
    invGain = 1.0f32 as libc::c_double;
    reached_max_gain = 0 as libc::c_int;
    n = 0 as libc::c_int;
    while n < D {
        s = 0 as libc::c_int;
        while s < nb_subfr {
            x_ptr = x.offset((s * subfr_length) as isize);
            tmp1 = *x_ptr.offset(n as isize) as libc::c_double;
            tmp2 = *x_ptr.offset((subfr_length - n - 1 as libc::c_int) as isize) as libc::c_double;
            k = 0 as libc::c_int;
            while k < n {
                C_first_row[k as usize] -= (*x_ptr.offset(n as isize)
                    * *x_ptr.offset((n - k - 1 as libc::c_int) as isize))
                    as libc::c_double;
                C_last_row[k as usize] -= (*x_ptr
                    .offset((subfr_length - n - 1 as libc::c_int) as isize)
                    * *x_ptr.offset((subfr_length - n + k) as isize))
                    as libc::c_double;
                Atmp = Af[k as usize];
                tmp1 += *x_ptr.offset((n - k - 1 as libc::c_int) as isize) as libc::c_double * Atmp;
                tmp2 += *x_ptr.offset((subfr_length - n + k) as isize) as libc::c_double * Atmp;
                k += 1;
            }
            k = 0 as libc::c_int;
            while k <= n {
                CAf[k as usize] -= tmp1 * *x_ptr.offset((n - k) as isize) as libc::c_double;
                CAb[k as usize] -= tmp2
                    * *x_ptr.offset((subfr_length - n + k - 1 as libc::c_int) as isize)
                        as libc::c_double;
                k += 1;
            }
            s += 1;
        }
        tmp1 = C_first_row[n as usize];
        tmp2 = C_last_row[n as usize];
        k = 0 as libc::c_int;
        while k < n {
            Atmp = Af[k as usize];
            tmp1 += C_last_row[(n - k - 1 as libc::c_int) as usize] * Atmp;
            tmp2 += C_first_row[(n - k - 1 as libc::c_int) as usize] * Atmp;
            k += 1;
        }
        CAf[(n + 1 as libc::c_int) as usize] = tmp1;
        CAb[(n + 1 as libc::c_int) as usize] = tmp2;
        num = CAb[(n + 1 as libc::c_int) as usize];
        nrg_b = CAb[0 as libc::c_int as usize];
        nrg_f = CAf[0 as libc::c_int as usize];
        k = 0 as libc::c_int;
        while k < n {
            Atmp = Af[k as usize];
            num += CAb[(n - k) as usize] * Atmp;
            nrg_b += CAb[(k + 1 as libc::c_int) as usize] * Atmp;
            nrg_f += CAf[(k + 1 as libc::c_int) as usize] * Atmp;
            k += 1;
        }
        rc = -2.0f64 * num / (nrg_f + nrg_b);
        tmp1 = invGain * (1.0f64 - rc * rc);
        if tmp1 <= minInvGain as libc::c_double {
            rc = (1.0f64 - minInvGain as f64 / invGain).sqrt();
            if num > 0 as libc::c_int as libc::c_double {
                rc = -rc;
            }
            invGain = minInvGain as libc::c_double;
            reached_max_gain = 1 as libc::c_int;
        } else {
            invGain = tmp1;
        }
        k = 0 as libc::c_int;
        while k < n + 1 as libc::c_int >> 1 as libc::c_int {
            tmp1 = Af[k as usize];
            tmp2 = Af[(n - k - 1 as libc::c_int) as usize];
            Af[k as usize] = tmp1 + rc * tmp2;
            Af[(n - k - 1 as libc::c_int) as usize] = tmp2 + rc * tmp1;
            k += 1;
        }
        Af[n as usize] = rc;
        if reached_max_gain != 0 {
            k = n + 1 as libc::c_int;
            while k < D {
                Af[k as usize] = 0.0f64;
                k += 1;
            }
            break;
        } else {
            k = 0 as libc::c_int;
            while k <= n + 1 as libc::c_int {
                tmp1 = CAf[k as usize];
                CAf[k as usize] += rc * CAb[(n - k + 1 as libc::c_int) as usize];
                CAb[(n - k + 1 as libc::c_int) as usize] += rc * tmp1;
                k += 1;
            }
            n += 1;
        }
    }
    if reached_max_gain != 0 {
        k = 0 as libc::c_int;
        while k < D {
            *A.offset(k as isize) = -Af[k as usize] as libc::c_float;
            k += 1;
        }
        s = 0 as libc::c_int;
        while s < nb_subfr {
            C0 -= silk_energy_FLP(x.offset((s * subfr_length) as isize), D);
            s += 1;
        }
        nrg_f = C0 * invGain;
    } else {
        nrg_f = CAf[0 as libc::c_int as usize];
        tmp1 = 1.0f64;
        k = 0 as libc::c_int;
        while k < D {
            Atmp = Af[k as usize];
            nrg_f += CAf[(k + 1 as libc::c_int) as usize] * Atmp;
            tmp1 += Atmp * Atmp;
            *A.offset(k as isize) = -Atmp as libc::c_float;
            k += 1;
        }
        nrg_f -= FIND_LPC_COND_FAC as libc::c_double * C0 * tmp1;
    }
    return nrg_f as libc::c_float;
}
