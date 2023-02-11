pub const EC_CLZ0: i32 = 32;

#[inline]
#[c2rust::src_loc = "120:1"]
pub unsafe extern "C" fn silk_CLZ32(in32: i32) -> i32 {
    return if in32 != 0 {
        32 - (EC_CLZ0 - (in32.leading_zeros() as i32))
    } else {
        32
    };
}
