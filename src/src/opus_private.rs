#[c2rust::src_loc = "108:9"]
pub const MODE_SILK_ONLY: libc::c_int = 1000 as libc::c_int;
#[c2rust::src_loc = "109:9"]
pub const MODE_HYBRID: libc::c_int = 1001 as libc::c_int;
#[c2rust::src_loc = "110:9"]
pub const MODE_CELT_ONLY: libc::c_int = 1002 as libc::c_int;

pub const OPUS_SET_VOICE_RATIO_REQUEST: libc::c_int = 11018;
#[c2rust::src_loc = "113:9"]
pub const OPUS_GET_VOICE_RATIO_REQUEST: libc::c_int = 11019;
#[c2rust::src_loc = "132:9"]
pub const OPUS_SET_FORCE_MODE_REQUEST: libc::c_int = 11002 as libc::c_int;

#[inline]
#[c2rust::src_loc = "154:1"]
pub unsafe extern "C" fn align(i: libc::c_int) -> libc::c_int {
    let alignment: libc::c_uint = 8 as libc::c_ulong as libc::c_uint;
    return (i as libc::c_uint)
        .wrapping_add(alignment)
        .wrapping_sub(1 as libc::c_int as libc::c_uint)
        .wrapping_div(alignment)
        .wrapping_mul(alignment) as libc::c_int;
}
