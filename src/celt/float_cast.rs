pub const CELT_SIG_SCALE: libc::c_float = 32768.0f32;

#[inline]
pub fn FLOAT2INT16(x: libc::c_float) -> i16 {
    (x * 32768.0) as i16
}
#[inline]
pub fn float2int(x: libc::c_float) -> i32 {
    x as i32
}
