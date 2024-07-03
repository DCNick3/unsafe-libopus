use crate::silk::float::inner_product_FLP::silk_inner_product_FLP;

pub unsafe fn silk_autocorrelation_FLP(
    results: *mut f32,
    inputData: *const f32,
    inputDataSize: i32,
    mut correlationCount: i32,
) {
    let mut i: i32 = 0;
    if correlationCount > inputDataSize {
        correlationCount = inputDataSize;
    }
    i = 0;
    while i < correlationCount {
        let size = (inputDataSize - i) as usize;

        *results.offset(i as isize) = silk_inner_product_FLP(
            std::slice::from_raw_parts(inputData, size),
            std::slice::from_raw_parts(inputData.offset(i as isize), size),
        ) as f32;
        i += 1;
    }
}
