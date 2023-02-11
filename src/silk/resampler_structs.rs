pub const SILK_RESAMPLER_MAX_FIR_ORDER: usize = 36;
pub const SILK_RESAMPLER_MAX_IIR_ORDER: usize = 6;

#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "38:16"]
pub struct silk_resampler_state_struct {
    pub sIIR: [i32; SILK_RESAMPLER_MAX_IIR_ORDER],
    pub sFIR: sFIR_union,
    pub delayBuf: [i16; 48],
    pub resampler_function: libc::c_int,
    pub batchSize: libc::c_int,
    pub invRatio_Q16: i32,
    pub FIR_Order: libc::c_int,
    pub FIR_Fracs: libc::c_int,
    pub Fs_in_kHz: libc::c_int,
    pub Fs_out_kHz: libc::c_int,
    pub inputDelay: libc::c_int,
    pub Coefs: *const i16,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "40:5"]
pub union sFIR_union {
    pub i32_0: [i32; SILK_RESAMPLER_MAX_FIR_ORDER],
    pub i16_0: [i16; SILK_RESAMPLER_MAX_FIR_ORDER],
}
