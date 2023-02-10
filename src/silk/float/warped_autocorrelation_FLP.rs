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
use self::arch_h::celt_fatal;
#[no_mangle]
#[c2rust::src_loc = "35:1"]
pub unsafe extern "C" fn silk_warped_autocorrelation_FLP(
    mut corr: *mut libc::c_float,
    mut input: *const libc::c_float,
    warping: libc::c_float,
    length: libc::c_int,
    order: libc::c_int,
) {
    let mut n: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut tmp1: libc::c_double = 0.;
    let mut tmp2: libc::c_double = 0.;
    let mut state: [libc::c_double; 25] = [
        0 as libc::c_int as libc::c_double,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    let mut C: [libc::c_double; 25] = [
        0 as libc::c_int as libc::c_double,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
        0.,
    ];
    if !(order & 1 as libc::c_int == 0 as libc::c_int) {
        celt_fatal(
            b"assertion failed: ( order & 1 ) == 0\0" as *const u8 as *const libc::c_char,
            b"silk/float/warped_autocorrelation_FLP.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
        );
    }
    n = 0 as libc::c_int;
    while n < length {
        tmp1 = *input.offset(n as isize) as libc::c_double;
        i = 0 as libc::c_int;
        while i < order {
            tmp2 = state[i as usize]
                + warping as libc::c_double * (state[(i + 1 as libc::c_int) as usize] - tmp1);
            state[i as usize] = tmp1;
            C[i as usize] += state[0 as libc::c_int as usize] * tmp1;
            tmp1 = state[(i + 1 as libc::c_int) as usize]
                + warping as libc::c_double * (state[(i + 2 as libc::c_int) as usize] - tmp2);
            state[(i + 1 as libc::c_int) as usize] = tmp2;
            C[(i + 1 as libc::c_int) as usize] += state[0 as libc::c_int as usize] * tmp2;
            i += 2 as libc::c_int;
        }
        state[order as usize] = tmp1;
        C[order as usize] += state[0 as libc::c_int as usize] * tmp1;
        n += 1;
    }
    i = 0 as libc::c_int;
    while i < order + 1 as libc::c_int {
        *corr.offset(i as isize) = C[i as usize] as libc::c_float;
        i += 1;
    }
}
