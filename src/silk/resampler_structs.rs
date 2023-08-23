pub const SILK_RESAMPLER_MAX_FIR_ORDER: usize = 36;
pub const SILK_RESAMPLER_MAX_IIR_ORDER: usize = 6;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct silk_resampler_state_struct {
    pub sIIR: [i32; SILK_RESAMPLER_MAX_IIR_ORDER],
    pub sFIR: sFIR_union,
    pub delayBuf: [i16; 48],
    pub resampler_function: i32,
    pub batchSize: i32,
    pub invRatio_Q16: i32,
    pub FIR_Order: i32,
    pub FIR_Fracs: i32,
    pub Fs_in_kHz: i32,
    pub Fs_out_kHz: i32,
    pub inputDelay: i32,
    pub Coefs: &'static [i16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sFIR_union {
    pub i32_0: [i32; SILK_RESAMPLER_MAX_FIR_ORDER],
    pub i16_0: [i16; SILK_RESAMPLER_MAX_FIR_ORDER],
}
