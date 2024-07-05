pub const EC_CLZ0: i32 = 32;

#[inline]
pub fn silk_CLZ32(in32: i32) -> i32 {
    return if in32 != 0 {
        32 - (EC_CLZ0 - (in32.leading_zeros() as i32))
    } else {
        32
    };
}

/// (a32 * (opus_int32)((opus_int16)(b32))) >> 16 output have to be 32bit int
#[inline]
pub fn silk_SMULWB(a32: i32, b32: i32) -> i32 {
    ((a32 as i64 * b32 as i16 as i64) >> 16) as i32
}

/// a32 + (b32 * (opus_int32)((opus_int16)(c32))) >> 16 output have to be 32bit int
#[inline]
pub fn silk_SMLAWB(a32: i32, b32: i32, c32: i32) -> i32 {
    (a32 as i64 + ((b32 as i64 * c32 as i16 as i64) >> 16)) as i32
}

/// (opus_int32)((opus_int16)(a3))) * (opus_int32)((opus_int16)(b32)) output have to be 32bit int
#[inline]
pub fn silk_SMULBB(a32: i32, b32: i32) -> i32 {
    a32 as i16 as i32 * b32 as i16 as i32
}

/// a32 + (b32 * (opus_int32)((opus_int16)(c32))) >> 16 output have to be 32bit int
#[inline]
pub fn silk_SMULWW(a32: i32, b32: i32) -> i32 {
    ((a32 as i64 * b32 as i64) >> 16) as i32
}
