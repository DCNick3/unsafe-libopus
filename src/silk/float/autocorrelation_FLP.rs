use crate::silk::float::inner_product_FLP::silk_inner_product_FLP;
use ::libc;

#[no_mangle]
#[c2rust::src_loc = "36:1"]
pub unsafe extern "C" fn silk_autocorrelation_FLP(
    results: *mut libc::c_float,
    inputData: *const libc::c_float,
    inputDataSize: libc::c_int,
    mut correlationCount: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    if correlationCount > inputDataSize {
        correlationCount = inputDataSize;
    }
    i = 0 as libc::c_int;
    while i < correlationCount {
        *results.offset(i as isize) =
            silk_inner_product_FLP(inputData, inputData.offset(i as isize), inputDataSize - i)
                as libc::c_float;
        i += 1;
    }
}
