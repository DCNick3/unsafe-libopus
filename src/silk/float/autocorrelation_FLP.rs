use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/float/SigProc_FLP.h:33"]
pub mod SigProc_FLP_h {
    extern "C" {
        #[c2rust::src_loc = "127:1"]
        pub fn silk_inner_product_FLP(
            data1: *const libc::c_float,
            data2: *const libc::c_float,
            dataSize: libc::c_int,
        ) -> libc::c_double;
    }
}
use self::SigProc_FLP_h::silk_inner_product_FLP;
#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn silk_autocorrelation_FLP(
    mut results: *mut libc::c_float,
    mut inputData: *const libc::c_float,
    mut inputDataSize: libc::c_int,
    mut correlationCount: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if correlationCount > inputDataSize {
        correlationCount = inputDataSize;
    }
    i = 0 as libc::c_int;
    while i < correlationCount {
        *results
            .offset(
                i as isize,
            ) = silk_inner_product_FLP(
            inputData,
            inputData.offset(i as isize),
            inputDataSize - i,
        ) as libc::c_float;
        i += 1;
    }
}
