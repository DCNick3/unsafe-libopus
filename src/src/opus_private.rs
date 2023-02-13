#[c2rust::src_loc = "108:9"]
pub const MODE_SILK_ONLY: i32 = 1000 as i32;
#[c2rust::src_loc = "109:9"]
pub const MODE_HYBRID: i32 = 1001 as i32;
#[c2rust::src_loc = "110:9"]
pub const MODE_CELT_ONLY: i32 = 1002 as i32;

pub const OPUS_SET_VOICE_RATIO_REQUEST: i32 = 11018;
#[c2rust::src_loc = "113:9"]
pub const OPUS_GET_VOICE_RATIO_REQUEST: i32 = 11019;
#[c2rust::src_loc = "132:9"]
pub const OPUS_SET_FORCE_MODE_REQUEST: i32 = 11002 as i32;

#[inline]
#[c2rust::src_loc = "154:1"]
pub unsafe fn align(i: i32) -> i32 {
    let alignment: u32 = 8 as u64 as u32;
    return (i as u32)
        .wrapping_add(alignment)
        .wrapping_sub(1 as i32 as u32)
        .wrapping_div(alignment)
        .wrapping_mul(alignment) as i32;
}
