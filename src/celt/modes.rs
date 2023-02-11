use ::libc;

#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "52:8"]
pub struct OpusCustomMode {
    pub Fs: i32,
    pub overlap: libc::c_int,
    pub nbEBands: libc::c_int,
    pub effEBands: libc::c_int,
    pub preemph: [opus_val16; 4],
    pub eBands: *const i16,
    pub maxLM: libc::c_int,
    pub nbShortMdcts: libc::c_int,
    pub shortMdctSize: libc::c_int,
    pub nbAllocVectors: libc::c_int,
    pub allocVectors: *const libc::c_uchar,
    pub logN: *const i16,
    pub window: *const opus_val16,
    pub mdct: mdct_lookup,
    pub cache: PulseCache,
}
#[derive(Copy, Clone)]
#[repr(C)]
#[c2rust::src_loc = "42:9"]
pub struct PulseCache {
    pub size: libc::c_int,
    pub index: *const i16,
    pub bits: *const libc::c_uchar,
    pub caps: *const libc::c_uchar,
}
pub const MAX_PERIOD: libc::c_int = 1024;

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:34"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/include/opus_defines.h:34"]
pub mod opus_defines_h {
    #[c2rust::src_loc = "46:9"]
    pub const OPUS_OK: libc::c_int = 0 as libc::c_int;
    #[c2rust::src_loc = "48:9"]
    pub const OPUS_BAD_ARG: libc::c_int = -(1 as libc::c_int);
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/stddef.h:34"]
pub mod stddef_h {
    #[c2rust::src_loc = "89:11"]
    pub const NULL: libc::c_int = 0 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/static_modes_float.h:69"]
pub mod static_modes_float_h {
    #[c2rust::src_loc = "14:25"]
    pub static mut window120: [opus_val16; 120] = [
        6.7286966e-05f32,
        0.00060551348f32,
        0.0016815970f32,
        0.0032947962f32,
        0.0054439943f32,
        0.0081276923f32,
        0.011344001f32,
        0.015090633f32,
        0.019364886f32,
        0.024163635f32,
        0.029483315f32,
        0.035319905f32,
        0.041668911f32,
        0.048525347f32,
        0.055883718f32,
        0.063737999f32,
        0.072081616f32,
        0.080907428f32,
        0.090207705f32,
        0.099974111f32,
        0.11019769f32,
        0.12086883f32,
        0.13197729f32,
        0.14351214f32,
        0.15546177f32,
        0.16781389f32,
        0.18055550f32,
        0.19367290f32,
        0.20715171f32,
        0.22097682f32,
        0.23513243f32,
        0.24960208f32,
        0.26436860f32,
        0.27941419f32,
        0.29472040f32,
        0.31026818f32,
        0.32603788f32,
        0.34200931f32,
        0.35816177f32,
        0.37447407f32,
        0.39092462f32,
        0.40749142f32,
        0.42415215f32,
        0.44088423f32,
        0.45766484f32,
        0.47447104f32,
        0.49127978f32,
        0.50806798f32,
        0.52481261f32,
        0.54149077f32,
        0.55807973f32,
        0.57455701f32,
        0.59090049f32,
        0.60708841f32,
        0.62309951f32,
        0.63891306f32,
        0.65450896f32,
        0.66986776f32,
        0.68497077f32,
        0.69980010f32,
        0.71433873f32,
        0.72857055f32,
        0.74248043f32,
        0.75605424f32,
        0.76927895f32,
        0.78214257f32,
        0.79463430f32,
        0.80674445f32,
        0.81846456f32,
        0.82978733f32,
        0.84070669f32,
        0.85121779f32,
        0.86131698f32,
        0.87100183f32,
        0.88027111f32,
        0.88912479f32,
        0.89756398f32,
        0.90559094f32,
        0.91320904f32,
        0.92042270f32,
        0.92723738f32,
        0.93365955f32,
        0.93969656f32,
        0.94535671f32,
        0.95064907f32,
        0.95558353f32,
        0.96017067f32,
        0.96442171f32,
        0.96834849f32,
        0.97196334f32,
        0.97527906f32,
        0.97830883f32,
        0.98106616f32,
        0.98356480f32,
        0.98581869f32,
        0.98784191f32,
        0.98964856f32,
        0.99125274f32,
        0.99266849f32,
        0.99390969f32,
        0.99499004f32,
        0.99592297f32,
        0.99672162f32,
        0.99739874f32,
        0.99796667f32,
        0.99843728f32,
        0.99882195f32,
        0.99913147f32,
        0.99937606f32,
        0.99956527f32,
        0.99970802f32,
        0.99981248f32,
        0.99988613f32,
        0.99993565f32,
        0.99996697f32,
        0.99998518f32,
        0.99999457f32,
        0.99999859f32,
        0.99999982f32,
        1.0000000f32,
    ];
    #[c2rust::src_loc = "44:25"]
    pub static mut logN400: [i16; 21] = [
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        8 as libc::c_int as i16,
        8 as libc::c_int as i16,
        8 as libc::c_int as i16,
        8 as libc::c_int as i16,
        16 as libc::c_int as i16,
        16 as libc::c_int as i16,
        16 as libc::c_int as i16,
        21 as libc::c_int as i16,
        21 as libc::c_int as i16,
        24 as libc::c_int as i16,
        29 as libc::c_int as i16,
        34 as libc::c_int as i16,
        36 as libc::c_int as i16,
    ];
    #[c2rust::src_loc = "50:25"]
    pub static mut cache_index50: [i16; 105] = [
        -(1 as libc::c_int) as i16,
        -(1 as libc::c_int) as i16,
        -(1 as libc::c_int) as i16,
        -(1 as libc::c_int) as i16,
        -(1 as libc::c_int) as i16,
        -(1 as libc::c_int) as i16,
        -(1 as libc::c_int) as i16,
        -(1 as libc::c_int) as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        41 as libc::c_int as i16,
        41 as libc::c_int as i16,
        41 as libc::c_int as i16,
        82 as libc::c_int as i16,
        82 as libc::c_int as i16,
        123 as libc::c_int as i16,
        164 as libc::c_int as i16,
        200 as libc::c_int as i16,
        222 as libc::c_int as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        0 as libc::c_int as i16,
        41 as libc::c_int as i16,
        41 as libc::c_int as i16,
        41 as libc::c_int as i16,
        41 as libc::c_int as i16,
        123 as libc::c_int as i16,
        123 as libc::c_int as i16,
        123 as libc::c_int as i16,
        164 as libc::c_int as i16,
        164 as libc::c_int as i16,
        240 as libc::c_int as i16,
        266 as libc::c_int as i16,
        283 as libc::c_int as i16,
        295 as libc::c_int as i16,
        41 as libc::c_int as i16,
        41 as libc::c_int as i16,
        41 as libc::c_int as i16,
        41 as libc::c_int as i16,
        41 as libc::c_int as i16,
        41 as libc::c_int as i16,
        41 as libc::c_int as i16,
        41 as libc::c_int as i16,
        123 as libc::c_int as i16,
        123 as libc::c_int as i16,
        123 as libc::c_int as i16,
        123 as libc::c_int as i16,
        240 as libc::c_int as i16,
        240 as libc::c_int as i16,
        240 as libc::c_int as i16,
        266 as libc::c_int as i16,
        266 as libc::c_int as i16,
        305 as libc::c_int as i16,
        318 as libc::c_int as i16,
        328 as libc::c_int as i16,
        336 as libc::c_int as i16,
        123 as libc::c_int as i16,
        123 as libc::c_int as i16,
        123 as libc::c_int as i16,
        123 as libc::c_int as i16,
        123 as libc::c_int as i16,
        123 as libc::c_int as i16,
        123 as libc::c_int as i16,
        123 as libc::c_int as i16,
        240 as libc::c_int as i16,
        240 as libc::c_int as i16,
        240 as libc::c_int as i16,
        240 as libc::c_int as i16,
        305 as libc::c_int as i16,
        305 as libc::c_int as i16,
        305 as libc::c_int as i16,
        318 as libc::c_int as i16,
        318 as libc::c_int as i16,
        343 as libc::c_int as i16,
        351 as libc::c_int as i16,
        358 as libc::c_int as i16,
        364 as libc::c_int as i16,
        240 as libc::c_int as i16,
        240 as libc::c_int as i16,
        240 as libc::c_int as i16,
        240 as libc::c_int as i16,
        240 as libc::c_int as i16,
        240 as libc::c_int as i16,
        240 as libc::c_int as i16,
        240 as libc::c_int as i16,
        305 as libc::c_int as i16,
        305 as libc::c_int as i16,
        305 as libc::c_int as i16,
        305 as libc::c_int as i16,
        343 as libc::c_int as i16,
        343 as libc::c_int as i16,
        343 as libc::c_int as i16,
        351 as libc::c_int as i16,
        351 as libc::c_int as i16,
        370 as libc::c_int as i16,
        376 as libc::c_int as i16,
        382 as libc::c_int as i16,
        387 as libc::c_int as i16,
    ];
    #[c2rust::src_loc = "59:28"]
    pub static mut cache_bits50: [libc::c_uchar; 392] = [
        40 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        40 as libc::c_int as libc::c_uchar,
        15 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar,
        31 as libc::c_int as libc::c_uchar,
        34 as libc::c_int as libc::c_uchar,
        36 as libc::c_int as libc::c_uchar,
        38 as libc::c_int as libc::c_uchar,
        39 as libc::c_int as libc::c_uchar,
        41 as libc::c_int as libc::c_uchar,
        42 as libc::c_int as libc::c_uchar,
        43 as libc::c_int as libc::c_uchar,
        44 as libc::c_int as libc::c_uchar,
        45 as libc::c_int as libc::c_uchar,
        46 as libc::c_int as libc::c_uchar,
        47 as libc::c_int as libc::c_uchar,
        47 as libc::c_int as libc::c_uchar,
        49 as libc::c_int as libc::c_uchar,
        50 as libc::c_int as libc::c_uchar,
        51 as libc::c_int as libc::c_uchar,
        52 as libc::c_int as libc::c_uchar,
        53 as libc::c_int as libc::c_uchar,
        54 as libc::c_int as libc::c_uchar,
        55 as libc::c_int as libc::c_uchar,
        55 as libc::c_int as libc::c_uchar,
        57 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        59 as libc::c_int as libc::c_uchar,
        60 as libc::c_int as libc::c_uchar,
        61 as libc::c_int as libc::c_uchar,
        62 as libc::c_int as libc::c_uchar,
        63 as libc::c_int as libc::c_uchar,
        63 as libc::c_int as libc::c_uchar,
        65 as libc::c_int as libc::c_uchar,
        66 as libc::c_int as libc::c_uchar,
        67 as libc::c_int as libc::c_uchar,
        68 as libc::c_int as libc::c_uchar,
        69 as libc::c_int as libc::c_uchar,
        70 as libc::c_int as libc::c_uchar,
        71 as libc::c_int as libc::c_uchar,
        71 as libc::c_int as libc::c_uchar,
        40 as libc::c_int as libc::c_uchar,
        20 as libc::c_int as libc::c_uchar,
        33 as libc::c_int as libc::c_uchar,
        41 as libc::c_int as libc::c_uchar,
        48 as libc::c_int as libc::c_uchar,
        53 as libc::c_int as libc::c_uchar,
        57 as libc::c_int as libc::c_uchar,
        61 as libc::c_int as libc::c_uchar,
        64 as libc::c_int as libc::c_uchar,
        66 as libc::c_int as libc::c_uchar,
        69 as libc::c_int as libc::c_uchar,
        71 as libc::c_int as libc::c_uchar,
        73 as libc::c_int as libc::c_uchar,
        75 as libc::c_int as libc::c_uchar,
        76 as libc::c_int as libc::c_uchar,
        78 as libc::c_int as libc::c_uchar,
        80 as libc::c_int as libc::c_uchar,
        82 as libc::c_int as libc::c_uchar,
        85 as libc::c_int as libc::c_uchar,
        87 as libc::c_int as libc::c_uchar,
        89 as libc::c_int as libc::c_uchar,
        91 as libc::c_int as libc::c_uchar,
        92 as libc::c_int as libc::c_uchar,
        94 as libc::c_int as libc::c_uchar,
        96 as libc::c_int as libc::c_uchar,
        98 as libc::c_int as libc::c_uchar,
        101 as libc::c_int as libc::c_uchar,
        103 as libc::c_int as libc::c_uchar,
        105 as libc::c_int as libc::c_uchar,
        107 as libc::c_int as libc::c_uchar,
        108 as libc::c_int as libc::c_uchar,
        110 as libc::c_int as libc::c_uchar,
        112 as libc::c_int as libc::c_uchar,
        114 as libc::c_int as libc::c_uchar,
        117 as libc::c_int as libc::c_uchar,
        119 as libc::c_int as libc::c_uchar,
        121 as libc::c_int as libc::c_uchar,
        123 as libc::c_int as libc::c_uchar,
        124 as libc::c_int as libc::c_uchar,
        126 as libc::c_int as libc::c_uchar,
        128 as libc::c_int as libc::c_uchar,
        40 as libc::c_int as libc::c_uchar,
        23 as libc::c_int as libc::c_uchar,
        39 as libc::c_int as libc::c_uchar,
        51 as libc::c_int as libc::c_uchar,
        60 as libc::c_int as libc::c_uchar,
        67 as libc::c_int as libc::c_uchar,
        73 as libc::c_int as libc::c_uchar,
        79 as libc::c_int as libc::c_uchar,
        83 as libc::c_int as libc::c_uchar,
        87 as libc::c_int as libc::c_uchar,
        91 as libc::c_int as libc::c_uchar,
        94 as libc::c_int as libc::c_uchar,
        97 as libc::c_int as libc::c_uchar,
        100 as libc::c_int as libc::c_uchar,
        102 as libc::c_int as libc::c_uchar,
        105 as libc::c_int as libc::c_uchar,
        107 as libc::c_int as libc::c_uchar,
        111 as libc::c_int as libc::c_uchar,
        115 as libc::c_int as libc::c_uchar,
        118 as libc::c_int as libc::c_uchar,
        121 as libc::c_int as libc::c_uchar,
        124 as libc::c_int as libc::c_uchar,
        126 as libc::c_int as libc::c_uchar,
        129 as libc::c_int as libc::c_uchar,
        131 as libc::c_int as libc::c_uchar,
        135 as libc::c_int as libc::c_uchar,
        139 as libc::c_int as libc::c_uchar,
        142 as libc::c_int as libc::c_uchar,
        145 as libc::c_int as libc::c_uchar,
        148 as libc::c_int as libc::c_uchar,
        150 as libc::c_int as libc::c_uchar,
        153 as libc::c_int as libc::c_uchar,
        155 as libc::c_int as libc::c_uchar,
        159 as libc::c_int as libc::c_uchar,
        163 as libc::c_int as libc::c_uchar,
        166 as libc::c_int as libc::c_uchar,
        169 as libc::c_int as libc::c_uchar,
        172 as libc::c_int as libc::c_uchar,
        174 as libc::c_int as libc::c_uchar,
        177 as libc::c_int as libc::c_uchar,
        179 as libc::c_int as libc::c_uchar,
        35 as libc::c_int as libc::c_uchar,
        28 as libc::c_int as libc::c_uchar,
        49 as libc::c_int as libc::c_uchar,
        65 as libc::c_int as libc::c_uchar,
        78 as libc::c_int as libc::c_uchar,
        89 as libc::c_int as libc::c_uchar,
        99 as libc::c_int as libc::c_uchar,
        107 as libc::c_int as libc::c_uchar,
        114 as libc::c_int as libc::c_uchar,
        120 as libc::c_int as libc::c_uchar,
        126 as libc::c_int as libc::c_uchar,
        132 as libc::c_int as libc::c_uchar,
        136 as libc::c_int as libc::c_uchar,
        141 as libc::c_int as libc::c_uchar,
        145 as libc::c_int as libc::c_uchar,
        149 as libc::c_int as libc::c_uchar,
        153 as libc::c_int as libc::c_uchar,
        159 as libc::c_int as libc::c_uchar,
        165 as libc::c_int as libc::c_uchar,
        171 as libc::c_int as libc::c_uchar,
        176 as libc::c_int as libc::c_uchar,
        180 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        189 as libc::c_int as libc::c_uchar,
        192 as libc::c_int as libc::c_uchar,
        199 as libc::c_int as libc::c_uchar,
        205 as libc::c_int as libc::c_uchar,
        211 as libc::c_int as libc::c_uchar,
        216 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        225 as libc::c_int as libc::c_uchar,
        229 as libc::c_int as libc::c_uchar,
        232 as libc::c_int as libc::c_uchar,
        239 as libc::c_int as libc::c_uchar,
        245 as libc::c_int as libc::c_uchar,
        251 as libc::c_int as libc::c_uchar,
        21 as libc::c_int as libc::c_uchar,
        33 as libc::c_int as libc::c_uchar,
        58 as libc::c_int as libc::c_uchar,
        79 as libc::c_int as libc::c_uchar,
        97 as libc::c_int as libc::c_uchar,
        112 as libc::c_int as libc::c_uchar,
        125 as libc::c_int as libc::c_uchar,
        137 as libc::c_int as libc::c_uchar,
        148 as libc::c_int as libc::c_uchar,
        157 as libc::c_int as libc::c_uchar,
        166 as libc::c_int as libc::c_uchar,
        174 as libc::c_int as libc::c_uchar,
        182 as libc::c_int as libc::c_uchar,
        189 as libc::c_int as libc::c_uchar,
        195 as libc::c_int as libc::c_uchar,
        201 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        217 as libc::c_int as libc::c_uchar,
        227 as libc::c_int as libc::c_uchar,
        235 as libc::c_int as libc::c_uchar,
        243 as libc::c_int as libc::c_uchar,
        251 as libc::c_int as libc::c_uchar,
        17 as libc::c_int as libc::c_uchar,
        35 as libc::c_int as libc::c_uchar,
        63 as libc::c_int as libc::c_uchar,
        86 as libc::c_int as libc::c_uchar,
        106 as libc::c_int as libc::c_uchar,
        123 as libc::c_int as libc::c_uchar,
        139 as libc::c_int as libc::c_uchar,
        152 as libc::c_int as libc::c_uchar,
        165 as libc::c_int as libc::c_uchar,
        177 as libc::c_int as libc::c_uchar,
        187 as libc::c_int as libc::c_uchar,
        197 as libc::c_int as libc::c_uchar,
        206 as libc::c_int as libc::c_uchar,
        214 as libc::c_int as libc::c_uchar,
        222 as libc::c_int as libc::c_uchar,
        230 as libc::c_int as libc::c_uchar,
        237 as libc::c_int as libc::c_uchar,
        250 as libc::c_int as libc::c_uchar,
        25 as libc::c_int as libc::c_uchar,
        31 as libc::c_int as libc::c_uchar,
        55 as libc::c_int as libc::c_uchar,
        75 as libc::c_int as libc::c_uchar,
        91 as libc::c_int as libc::c_uchar,
        105 as libc::c_int as libc::c_uchar,
        117 as libc::c_int as libc::c_uchar,
        128 as libc::c_int as libc::c_uchar,
        138 as libc::c_int as libc::c_uchar,
        146 as libc::c_int as libc::c_uchar,
        154 as libc::c_int as libc::c_uchar,
        161 as libc::c_int as libc::c_uchar,
        168 as libc::c_int as libc::c_uchar,
        174 as libc::c_int as libc::c_uchar,
        180 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        190 as libc::c_int as libc::c_uchar,
        200 as libc::c_int as libc::c_uchar,
        208 as libc::c_int as libc::c_uchar,
        215 as libc::c_int as libc::c_uchar,
        222 as libc::c_int as libc::c_uchar,
        229 as libc::c_int as libc::c_uchar,
        235 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        245 as libc::c_int as libc::c_uchar,
        255 as libc::c_int as libc::c_uchar,
        16 as libc::c_int as libc::c_uchar,
        36 as libc::c_int as libc::c_uchar,
        65 as libc::c_int as libc::c_uchar,
        89 as libc::c_int as libc::c_uchar,
        110 as libc::c_int as libc::c_uchar,
        128 as libc::c_int as libc::c_uchar,
        144 as libc::c_int as libc::c_uchar,
        159 as libc::c_int as libc::c_uchar,
        173 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        196 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        217 as libc::c_int as libc::c_uchar,
        226 as libc::c_int as libc::c_uchar,
        234 as libc::c_int as libc::c_uchar,
        242 as libc::c_int as libc::c_uchar,
        250 as libc::c_int as libc::c_uchar,
        11 as libc::c_int as libc::c_uchar,
        41 as libc::c_int as libc::c_uchar,
        74 as libc::c_int as libc::c_uchar,
        103 as libc::c_int as libc::c_uchar,
        128 as libc::c_int as libc::c_uchar,
        151 as libc::c_int as libc::c_uchar,
        172 as libc::c_int as libc::c_uchar,
        191 as libc::c_int as libc::c_uchar,
        209 as libc::c_int as libc::c_uchar,
        225 as libc::c_int as libc::c_uchar,
        241 as libc::c_int as libc::c_uchar,
        255 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        43 as libc::c_int as libc::c_uchar,
        79 as libc::c_int as libc::c_uchar,
        110 as libc::c_int as libc::c_uchar,
        138 as libc::c_int as libc::c_uchar,
        163 as libc::c_int as libc::c_uchar,
        186 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        227 as libc::c_int as libc::c_uchar,
        246 as libc::c_int as libc::c_uchar,
        12 as libc::c_int as libc::c_uchar,
        39 as libc::c_int as libc::c_uchar,
        71 as libc::c_int as libc::c_uchar,
        99 as libc::c_int as libc::c_uchar,
        123 as libc::c_int as libc::c_uchar,
        144 as libc::c_int as libc::c_uchar,
        164 as libc::c_int as libc::c_uchar,
        182 as libc::c_int as libc::c_uchar,
        198 as libc::c_int as libc::c_uchar,
        214 as libc::c_int as libc::c_uchar,
        228 as libc::c_int as libc::c_uchar,
        241 as libc::c_int as libc::c_uchar,
        253 as libc::c_int as libc::c_uchar,
        9 as libc::c_int as libc::c_uchar,
        44 as libc::c_int as libc::c_uchar,
        81 as libc::c_int as libc::c_uchar,
        113 as libc::c_int as libc::c_uchar,
        142 as libc::c_int as libc::c_uchar,
        168 as libc::c_int as libc::c_uchar,
        192 as libc::c_int as libc::c_uchar,
        214 as libc::c_int as libc::c_uchar,
        235 as libc::c_int as libc::c_uchar,
        255 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        49 as libc::c_int as libc::c_uchar,
        90 as libc::c_int as libc::c_uchar,
        127 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        191 as libc::c_int as libc::c_uchar,
        220 as libc::c_int as libc::c_uchar,
        247 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        51 as libc::c_int as libc::c_uchar,
        95 as libc::c_int as libc::c_uchar,
        134 as libc::c_int as libc::c_uchar,
        170 as libc::c_int as libc::c_uchar,
        203 as libc::c_int as libc::c_uchar,
        234 as libc::c_int as libc::c_uchar,
        7 as libc::c_int as libc::c_uchar,
        47 as libc::c_int as libc::c_uchar,
        87 as libc::c_int as libc::c_uchar,
        123 as libc::c_int as libc::c_uchar,
        155 as libc::c_int as libc::c_uchar,
        184 as libc::c_int as libc::c_uchar,
        212 as libc::c_int as libc::c_uchar,
        237 as libc::c_int as libc::c_uchar,
        6 as libc::c_int as libc::c_uchar,
        52 as libc::c_int as libc::c_uchar,
        97 as libc::c_int as libc::c_uchar,
        137 as libc::c_int as libc::c_uchar,
        174 as libc::c_int as libc::c_uchar,
        208 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        57 as libc::c_int as libc::c_uchar,
        106 as libc::c_int as libc::c_uchar,
        151 as libc::c_int as libc::c_uchar,
        192 as libc::c_int as libc::c_uchar,
        231 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        59 as libc::c_int as libc::c_uchar,
        111 as libc::c_int as libc::c_uchar,
        158 as libc::c_int as libc::c_uchar,
        202 as libc::c_int as libc::c_uchar,
        243 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        55 as libc::c_int as libc::c_uchar,
        103 as libc::c_int as libc::c_uchar,
        147 as libc::c_int as libc::c_uchar,
        187 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        5 as libc::c_int as libc::c_uchar,
        60 as libc::c_int as libc::c_uchar,
        113 as libc::c_int as libc::c_uchar,
        161 as libc::c_int as libc::c_uchar,
        206 as libc::c_int as libc::c_uchar,
        248 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        65 as libc::c_int as libc::c_uchar,
        122 as libc::c_int as libc::c_uchar,
        175 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        4 as libc::c_int as libc::c_uchar,
        67 as libc::c_int as libc::c_uchar,
        127 as libc::c_int as libc::c_uchar,
        182 as libc::c_int as libc::c_uchar,
        234 as libc::c_int as libc::c_uchar,
    ];
    #[c2rust::src_loc = "87:28"]
    pub static mut cache_caps50: [libc::c_uchar; 168] = [
        224 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        178 as libc::c_int as libc::c_uchar,
        178 as libc::c_int as libc::c_uchar,
        168 as libc::c_int as libc::c_uchar,
        134 as libc::c_int as libc::c_uchar,
        61 as libc::c_int as libc::c_uchar,
        37 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        224 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        198 as libc::c_int as libc::c_uchar,
        198 as libc::c_int as libc::c_uchar,
        183 as libc::c_int as libc::c_uchar,
        144 as libc::c_int as libc::c_uchar,
        66 as libc::c_int as libc::c_uchar,
        40 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        160 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        183 as libc::c_int as libc::c_uchar,
        183 as libc::c_int as libc::c_uchar,
        172 as libc::c_int as libc::c_uchar,
        138 as libc::c_int as libc::c_uchar,
        64 as libc::c_int as libc::c_uchar,
        38 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        240 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        180 as libc::c_int as libc::c_uchar,
        143 as libc::c_int as libc::c_uchar,
        66 as libc::c_int as libc::c_uchar,
        40 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        185 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        183 as libc::c_int as libc::c_uchar,
        183 as libc::c_int as libc::c_uchar,
        172 as libc::c_int as libc::c_uchar,
        138 as libc::c_int as libc::c_uchar,
        65 as libc::c_int as libc::c_uchar,
        39 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        207 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        201 as libc::c_int as libc::c_uchar,
        201 as libc::c_int as libc::c_uchar,
        201 as libc::c_int as libc::c_uchar,
        188 as libc::c_int as libc::c_uchar,
        188 as libc::c_int as libc::c_uchar,
        176 as libc::c_int as libc::c_uchar,
        141 as libc::c_int as libc::c_uchar,
        66 as libc::c_int as libc::c_uchar,
        40 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        193 as libc::c_int as libc::c_uchar,
        194 as libc::c_int as libc::c_uchar,
        194 as libc::c_int as libc::c_uchar,
        194 as libc::c_int as libc::c_uchar,
        184 as libc::c_int as libc::c_uchar,
        184 as libc::c_int as libc::c_uchar,
        173 as libc::c_int as libc::c_uchar,
        139 as libc::c_int as libc::c_uchar,
        65 as libc::c_int as libc::c_uchar,
        39 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        204 as libc::c_int as libc::c_uchar,
        201 as libc::c_int as libc::c_uchar,
        201 as libc::c_int as libc::c_uchar,
        201 as libc::c_int as libc::c_uchar,
        201 as libc::c_int as libc::c_uchar,
        198 as libc::c_int as libc::c_uchar,
        198 as libc::c_int as libc::c_uchar,
        198 as libc::c_int as libc::c_uchar,
        187 as libc::c_int as libc::c_uchar,
        187 as libc::c_int as libc::c_uchar,
        175 as libc::c_int as libc::c_uchar,
        140 as libc::c_int as libc::c_uchar,
        66 as libc::c_int as libc::c_uchar,
        40 as libc::c_int as libc::c_uchar,
    ];
    #[c2rust::src_loc = "104:31"]
    pub static mut fft_twiddles48000_960: [kiss_twiddle_cpx; 480] = [
        {
            let init = kiss_twiddle_cpx {
                r: 1.0000000f32,
                i: -0.0000000f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99991433f32,
                i: -0.013089596f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99965732f32,
                i: -0.026176948f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99922904f32,
                i: -0.039259816f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99862953f32,
                i: -0.052335956f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99785892f32,
                i: -0.065403129f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99691733f32,
                i: -0.078459096f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99580493f32,
                i: -0.091501619f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99452190f32,
                i: -0.10452846f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99306846f32,
                i: -0.11753740f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99144486f32,
                i: -0.13052619f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.98965139f32,
                i: -0.14349262f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.98768834f32,
                i: -0.15643447f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.98555606f32,
                i: -0.16934950f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.98325491f32,
                i: -0.18223553f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.98078528f32,
                i: -0.19509032f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.97814760f32,
                i: -0.20791169f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.97534232f32,
                i: -0.22069744f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.97236992f32,
                i: -0.23344536f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.96923091f32,
                i: -0.24615329f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.96592583f32,
                i: -0.25881905f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.96245524f32,
                i: -0.27144045f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.95881973f32,
                i: -0.28401534f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.95501994f32,
                i: -0.29654157f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.95105652f32,
                i: -0.30901699f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.94693013f32,
                i: -0.32143947f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.94264149f32,
                i: -0.33380686f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.93819134f32,
                i: -0.34611706f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.93358043f32,
                i: -0.35836795f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.92880955f32,
                i: -0.37055744f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.92387953f32,
                i: -0.38268343f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.91879121f32,
                i: -0.39474386f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.91354546f32,
                i: -0.40673664f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.90814317f32,
                i: -0.41865974f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.90258528f32,
                i: -0.43051110f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.89687274f32,
                i: -0.44228869f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.89100652f32,
                i: -0.45399050f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.88498764f32,
                i: -0.46561452f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.87881711f32,
                i: -0.47715876f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.87249601f32,
                i: -0.48862124f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.86602540f32,
                i: -0.50000000f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.85940641f32,
                i: -0.51129309f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.85264016f32,
                i: -0.52249856f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.84572782f32,
                i: -0.53361452f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.83867057f32,
                i: -0.54463904f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.83146961f32,
                i: -0.55557023f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.82412619f32,
                i: -0.56640624f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.81664156f32,
                i: -0.57714519f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.80901699f32,
                i: -0.58778525f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.80125381f32,
                i: -0.59832460f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.79335334f32,
                i: -0.60876143f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.78531693f32,
                i: -0.61909395f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.77714596f32,
                i: -0.62932039f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.76884183f32,
                i: -0.63943900f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.76040597f32,
                i: -0.64944805f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.75183981f32,
                i: -0.65934582f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.74314483f32,
                i: -0.66913061f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.73432251f32,
                i: -0.67880075f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.72537437f32,
                i: -0.68835458f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.71630194f32,
                i: -0.69779046f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.70710678f32,
                i: -0.70710678f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.69779046f32,
                i: -0.71630194f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.68835458f32,
                i: -0.72537437f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.67880075f32,
                i: -0.73432251f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.66913061f32,
                i: -0.74314483f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.65934582f32,
                i: -0.75183981f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.64944805f32,
                i: -0.76040597f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.63943900f32,
                i: -0.76884183f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.62932039f32,
                i: -0.77714596f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.61909395f32,
                i: -0.78531693f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.60876143f32,
                i: -0.79335334f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.59832460f32,
                i: -0.80125381f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.58778525f32,
                i: -0.80901699f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.57714519f32,
                i: -0.81664156f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.56640624f32,
                i: -0.82412619f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.55557023f32,
                i: -0.83146961f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.54463904f32,
                i: -0.83867057f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.53361452f32,
                i: -0.84572782f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.52249856f32,
                i: -0.85264016f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.51129309f32,
                i: -0.85940641f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.50000000f32,
                i: -0.86602540f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.48862124f32,
                i: -0.87249601f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.47715876f32,
                i: -0.87881711f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.46561452f32,
                i: -0.88498764f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.45399050f32,
                i: -0.89100652f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.44228869f32,
                i: -0.89687274f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.43051110f32,
                i: -0.90258528f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.41865974f32,
                i: -0.90814317f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.40673664f32,
                i: -0.91354546f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.39474386f32,
                i: -0.91879121f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.38268343f32,
                i: -0.92387953f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.37055744f32,
                i: -0.92880955f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.35836795f32,
                i: -0.93358043f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.34611706f32,
                i: -0.93819134f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.33380686f32,
                i: -0.94264149f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.32143947f32,
                i: -0.94693013f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.30901699f32,
                i: -0.95105652f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.29654157f32,
                i: -0.95501994f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.28401534f32,
                i: -0.95881973f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.27144045f32,
                i: -0.96245524f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.25881905f32,
                i: -0.96592583f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.24615329f32,
                i: -0.96923091f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.23344536f32,
                i: -0.97236992f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.22069744f32,
                i: -0.97534232f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.20791169f32,
                i: -0.97814760f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.19509032f32,
                i: -0.98078528f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.18223553f32,
                i: -0.98325491f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.16934950f32,
                i: -0.98555606f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.15643447f32,
                i: -0.98768834f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.14349262f32,
                i: -0.98965139f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.13052619f32,
                i: -0.99144486f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.11753740f32,
                i: -0.99306846f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.10452846f32,
                i: -0.99452190f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.091501619f32,
                i: -0.99580493f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.078459096f32,
                i: -0.99691733f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.065403129f32,
                i: -0.99785892f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.052335956f32,
                i: -0.99862953f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.039259816f32,
                i: -0.99922904f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.026176948f32,
                i: -0.99965732f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.013089596f32,
                i: -0.99991433f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 6.1230318e-17f32,
                i: -1.0000000f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.013089596f32,
                i: -0.99991433f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.026176948f32,
                i: -0.99965732f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.039259816f32,
                i: -0.99922904f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.052335956f32,
                i: -0.99862953f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.065403129f32,
                i: -0.99785892f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.078459096f32,
                i: -0.99691733f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.091501619f32,
                i: -0.99580493f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.10452846f32,
                i: -0.99452190f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.11753740f32,
                i: -0.99306846f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.13052619f32,
                i: -0.99144486f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.14349262f32,
                i: -0.98965139f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.15643447f32,
                i: -0.98768834f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.16934950f32,
                i: -0.98555606f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.18223553f32,
                i: -0.98325491f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.19509032f32,
                i: -0.98078528f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.20791169f32,
                i: -0.97814760f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.22069744f32,
                i: -0.97534232f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.23344536f32,
                i: -0.97236992f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.24615329f32,
                i: -0.96923091f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.25881905f32,
                i: -0.96592583f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.27144045f32,
                i: -0.96245524f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.28401534f32,
                i: -0.95881973f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.29654157f32,
                i: -0.95501994f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.30901699f32,
                i: -0.95105652f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.32143947f32,
                i: -0.94693013f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.33380686f32,
                i: -0.94264149f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.34611706f32,
                i: -0.93819134f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.35836795f32,
                i: -0.93358043f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.37055744f32,
                i: -0.92880955f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.38268343f32,
                i: -0.92387953f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.39474386f32,
                i: -0.91879121f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.40673664f32,
                i: -0.91354546f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.41865974f32,
                i: -0.90814317f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.43051110f32,
                i: -0.90258528f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.44228869f32,
                i: -0.89687274f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.45399050f32,
                i: -0.89100652f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.46561452f32,
                i: -0.88498764f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.47715876f32,
                i: -0.87881711f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.48862124f32,
                i: -0.87249601f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.50000000f32,
                i: -0.86602540f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.51129309f32,
                i: -0.85940641f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.52249856f32,
                i: -0.85264016f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.53361452f32,
                i: -0.84572782f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.54463904f32,
                i: -0.83867057f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.55557023f32,
                i: -0.83146961f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.56640624f32,
                i: -0.82412619f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.57714519f32,
                i: -0.81664156f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.58778525f32,
                i: -0.80901699f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.59832460f32,
                i: -0.80125381f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.60876143f32,
                i: -0.79335334f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.61909395f32,
                i: -0.78531693f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.62932039f32,
                i: -0.77714596f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.63943900f32,
                i: -0.76884183f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.64944805f32,
                i: -0.76040597f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.65934582f32,
                i: -0.75183981f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.66913061f32,
                i: -0.74314483f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.67880075f32,
                i: -0.73432251f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.68835458f32,
                i: -0.72537437f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.69779046f32,
                i: -0.71630194f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.70710678f32,
                i: -0.70710678f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.71630194f32,
                i: -0.69779046f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.72537437f32,
                i: -0.68835458f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.73432251f32,
                i: -0.67880075f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.74314483f32,
                i: -0.66913061f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.75183981f32,
                i: -0.65934582f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.76040597f32,
                i: -0.64944805f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.76884183f32,
                i: -0.63943900f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.77714596f32,
                i: -0.62932039f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.78531693f32,
                i: -0.61909395f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.79335334f32,
                i: -0.60876143f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.80125381f32,
                i: -0.59832460f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.80901699f32,
                i: -0.58778525f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.81664156f32,
                i: -0.57714519f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.82412619f32,
                i: -0.56640624f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.83146961f32,
                i: -0.55557023f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.83867057f32,
                i: -0.54463904f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.84572782f32,
                i: -0.53361452f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.85264016f32,
                i: -0.52249856f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.85940641f32,
                i: -0.51129309f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.86602540f32,
                i: -0.50000000f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.87249601f32,
                i: -0.48862124f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.87881711f32,
                i: -0.47715876f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.88498764f32,
                i: -0.46561452f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.89100652f32,
                i: -0.45399050f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.89687274f32,
                i: -0.44228869f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.90258528f32,
                i: -0.43051110f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.90814317f32,
                i: -0.41865974f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.91354546f32,
                i: -0.40673664f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.91879121f32,
                i: -0.39474386f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.92387953f32,
                i: -0.38268343f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.92880955f32,
                i: -0.37055744f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.93358043f32,
                i: -0.35836795f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.93819134f32,
                i: -0.34611706f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.94264149f32,
                i: -0.33380686f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.94693013f32,
                i: -0.32143947f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.95105652f32,
                i: -0.30901699f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.95501994f32,
                i: -0.29654157f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.95881973f32,
                i: -0.28401534f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.96245524f32,
                i: -0.27144045f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.96592583f32,
                i: -0.25881905f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.96923091f32,
                i: -0.24615329f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.97236992f32,
                i: -0.23344536f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.97534232f32,
                i: -0.22069744f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.97814760f32,
                i: -0.20791169f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.98078528f32,
                i: -0.19509032f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.98325491f32,
                i: -0.18223553f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.98555606f32,
                i: -0.16934950f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.98768834f32,
                i: -0.15643447f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.98965139f32,
                i: -0.14349262f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99144486f32,
                i: -0.13052619f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99306846f32,
                i: -0.11753740f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99452190f32,
                i: -0.10452846f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99580493f32,
                i: -0.091501619f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99691733f32,
                i: -0.078459096f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99785892f32,
                i: -0.065403129f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99862953f32,
                i: -0.052335956f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99922904f32,
                i: -0.039259816f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99965732f32,
                i: -0.026176948f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99991433f32,
                i: -0.013089596f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -1.0000000f32,
                i: -1.2246064e-16f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99991433f32,
                i: 0.013089596f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99965732f32,
                i: 0.026176948f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99922904f32,
                i: 0.039259816f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99862953f32,
                i: 0.052335956f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99785892f32,
                i: 0.065403129f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99691733f32,
                i: 0.078459096f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99580493f32,
                i: 0.091501619f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99452190f32,
                i: 0.10452846f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99306846f32,
                i: 0.11753740f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.99144486f32,
                i: 0.13052619f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.98965139f32,
                i: 0.14349262f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.98768834f32,
                i: 0.15643447f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.98555606f32,
                i: 0.16934950f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.98325491f32,
                i: 0.18223553f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.98078528f32,
                i: 0.19509032f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.97814760f32,
                i: 0.20791169f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.97534232f32,
                i: 0.22069744f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.97236992f32,
                i: 0.23344536f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.96923091f32,
                i: 0.24615329f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.96592583f32,
                i: 0.25881905f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.96245524f32,
                i: 0.27144045f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.95881973f32,
                i: 0.28401534f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.95501994f32,
                i: 0.29654157f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.95105652f32,
                i: 0.30901699f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.94693013f32,
                i: 0.32143947f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.94264149f32,
                i: 0.33380686f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.93819134f32,
                i: 0.34611706f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.93358043f32,
                i: 0.35836795f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.92880955f32,
                i: 0.37055744f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.92387953f32,
                i: 0.38268343f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.91879121f32,
                i: 0.39474386f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.91354546f32,
                i: 0.40673664f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.90814317f32,
                i: 0.41865974f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.90258528f32,
                i: 0.43051110f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.89687274f32,
                i: 0.44228869f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.89100652f32,
                i: 0.45399050f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.88498764f32,
                i: 0.46561452f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.87881711f32,
                i: 0.47715876f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.87249601f32,
                i: 0.48862124f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.86602540f32,
                i: 0.50000000f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.85940641f32,
                i: 0.51129309f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.85264016f32,
                i: 0.52249856f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.84572782f32,
                i: 0.53361452f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.83867057f32,
                i: 0.54463904f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.83146961f32,
                i: 0.55557023f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.82412619f32,
                i: 0.56640624f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.81664156f32,
                i: 0.57714519f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.80901699f32,
                i: 0.58778525f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.80125381f32,
                i: 0.59832460f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.79335334f32,
                i: 0.60876143f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.78531693f32,
                i: 0.61909395f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.77714596f32,
                i: 0.62932039f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.76884183f32,
                i: 0.63943900f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.76040597f32,
                i: 0.64944805f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.75183981f32,
                i: 0.65934582f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.74314483f32,
                i: 0.66913061f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.73432251f32,
                i: 0.67880075f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.72537437f32,
                i: 0.68835458f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.71630194f32,
                i: 0.69779046f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.70710678f32,
                i: 0.70710678f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.69779046f32,
                i: 0.71630194f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.68835458f32,
                i: 0.72537437f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.67880075f32,
                i: 0.73432251f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.66913061f32,
                i: 0.74314483f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.65934582f32,
                i: 0.75183981f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.64944805f32,
                i: 0.76040597f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.63943900f32,
                i: 0.76884183f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.62932039f32,
                i: 0.77714596f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.61909395f32,
                i: 0.78531693f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.60876143f32,
                i: 0.79335334f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.59832460f32,
                i: 0.80125381f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.58778525f32,
                i: 0.80901699f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.57714519f32,
                i: 0.81664156f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.56640624f32,
                i: 0.82412619f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.55557023f32,
                i: 0.83146961f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.54463904f32,
                i: 0.83867057f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.53361452f32,
                i: 0.84572782f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.52249856f32,
                i: 0.85264016f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.51129309f32,
                i: 0.85940641f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.50000000f32,
                i: 0.86602540f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.48862124f32,
                i: 0.87249601f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.47715876f32,
                i: 0.87881711f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.46561452f32,
                i: 0.88498764f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.45399050f32,
                i: 0.89100652f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.44228869f32,
                i: 0.89687274f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.43051110f32,
                i: 0.90258528f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.41865974f32,
                i: 0.90814317f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.40673664f32,
                i: 0.91354546f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.39474386f32,
                i: 0.91879121f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.38268343f32,
                i: 0.92387953f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.37055744f32,
                i: 0.92880955f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.35836795f32,
                i: 0.93358043f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.34611706f32,
                i: 0.93819134f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.33380686f32,
                i: 0.94264149f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.32143947f32,
                i: 0.94693013f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.30901699f32,
                i: 0.95105652f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.29654157f32,
                i: 0.95501994f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.28401534f32,
                i: 0.95881973f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.27144045f32,
                i: 0.96245524f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.25881905f32,
                i: 0.96592583f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.24615329f32,
                i: 0.96923091f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.23344536f32,
                i: 0.97236992f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.22069744f32,
                i: 0.97534232f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.20791169f32,
                i: 0.97814760f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.19509032f32,
                i: 0.98078528f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.18223553f32,
                i: 0.98325491f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.16934950f32,
                i: 0.98555606f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.15643447f32,
                i: 0.98768834f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.14349262f32,
                i: 0.98965139f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.13052619f32,
                i: 0.99144486f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.11753740f32,
                i: 0.99306846f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.10452846f32,
                i: 0.99452190f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.091501619f32,
                i: 0.99580493f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.078459096f32,
                i: 0.99691733f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.065403129f32,
                i: 0.99785892f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.052335956f32,
                i: 0.99862953f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.039259816f32,
                i: 0.99922904f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.026176948f32,
                i: 0.99965732f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -0.013089596f32,
                i: 0.99991433f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: -1.8369095e-16f32,
                i: 1.0000000f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.013089596f32,
                i: 0.99991433f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.026176948f32,
                i: 0.99965732f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.039259816f32,
                i: 0.99922904f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.052335956f32,
                i: 0.99862953f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.065403129f32,
                i: 0.99785892f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.078459096f32,
                i: 0.99691733f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.091501619f32,
                i: 0.99580493f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.10452846f32,
                i: 0.99452190f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.11753740f32,
                i: 0.99306846f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.13052619f32,
                i: 0.99144486f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.14349262f32,
                i: 0.98965139f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.15643447f32,
                i: 0.98768834f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.16934950f32,
                i: 0.98555606f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.18223553f32,
                i: 0.98325491f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.19509032f32,
                i: 0.98078528f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.20791169f32,
                i: 0.97814760f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.22069744f32,
                i: 0.97534232f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.23344536f32,
                i: 0.97236992f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.24615329f32,
                i: 0.96923091f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.25881905f32,
                i: 0.96592583f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.27144045f32,
                i: 0.96245524f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.28401534f32,
                i: 0.95881973f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.29654157f32,
                i: 0.95501994f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.30901699f32,
                i: 0.95105652f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.32143947f32,
                i: 0.94693013f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.33380686f32,
                i: 0.94264149f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.34611706f32,
                i: 0.93819134f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.35836795f32,
                i: 0.93358043f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.37055744f32,
                i: 0.92880955f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.38268343f32,
                i: 0.92387953f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.39474386f32,
                i: 0.91879121f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.40673664f32,
                i: 0.91354546f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.41865974f32,
                i: 0.90814317f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.43051110f32,
                i: 0.90258528f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.44228869f32,
                i: 0.89687274f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.45399050f32,
                i: 0.89100652f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.46561452f32,
                i: 0.88498764f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.47715876f32,
                i: 0.87881711f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.48862124f32,
                i: 0.87249601f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.50000000f32,
                i: 0.86602540f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.51129309f32,
                i: 0.85940641f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.52249856f32,
                i: 0.85264016f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.53361452f32,
                i: 0.84572782f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.54463904f32,
                i: 0.83867057f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.55557023f32,
                i: 0.83146961f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.56640624f32,
                i: 0.82412619f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.57714519f32,
                i: 0.81664156f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.58778525f32,
                i: 0.80901699f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.59832460f32,
                i: 0.80125381f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.60876143f32,
                i: 0.79335334f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.61909395f32,
                i: 0.78531693f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.62932039f32,
                i: 0.77714596f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.63943900f32,
                i: 0.76884183f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.64944805f32,
                i: 0.76040597f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.65934582f32,
                i: 0.75183981f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.66913061f32,
                i: 0.74314483f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.67880075f32,
                i: 0.73432251f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.68835458f32,
                i: 0.72537437f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.69779046f32,
                i: 0.71630194f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.70710678f32,
                i: 0.70710678f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.71630194f32,
                i: 0.69779046f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.72537437f32,
                i: 0.68835458f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.73432251f32,
                i: 0.67880075f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.74314483f32,
                i: 0.66913061f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.75183981f32,
                i: 0.65934582f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.76040597f32,
                i: 0.64944805f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.76884183f32,
                i: 0.63943900f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.77714596f32,
                i: 0.62932039f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.78531693f32,
                i: 0.61909395f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.79335334f32,
                i: 0.60876143f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.80125381f32,
                i: 0.59832460f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.80901699f32,
                i: 0.58778525f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.81664156f32,
                i: 0.57714519f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.82412619f32,
                i: 0.56640624f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.83146961f32,
                i: 0.55557023f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.83867057f32,
                i: 0.54463904f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.84572782f32,
                i: 0.53361452f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.85264016f32,
                i: 0.52249856f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.85940641f32,
                i: 0.51129309f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.86602540f32,
                i: 0.50000000f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.87249601f32,
                i: 0.48862124f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.87881711f32,
                i: 0.47715876f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.88498764f32,
                i: 0.46561452f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.89100652f32,
                i: 0.45399050f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.89687274f32,
                i: 0.44228869f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.90258528f32,
                i: 0.43051110f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.90814317f32,
                i: 0.41865974f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.91354546f32,
                i: 0.40673664f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.91879121f32,
                i: 0.39474386f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.92387953f32,
                i: 0.38268343f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.92880955f32,
                i: 0.37055744f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.93358043f32,
                i: 0.35836795f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.93819134f32,
                i: 0.34611706f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.94264149f32,
                i: 0.33380686f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.94693013f32,
                i: 0.32143947f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.95105652f32,
                i: 0.30901699f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.95501994f32,
                i: 0.29654157f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.95881973f32,
                i: 0.28401534f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.96245524f32,
                i: 0.27144045f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.96592583f32,
                i: 0.25881905f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.96923091f32,
                i: 0.24615329f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.97236992f32,
                i: 0.23344536f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.97534232f32,
                i: 0.22069744f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.97814760f32,
                i: 0.20791169f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.98078528f32,
                i: 0.19509032f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.98325491f32,
                i: 0.18223553f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.98555606f32,
                i: 0.16934950f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.98768834f32,
                i: 0.15643447f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.98965139f32,
                i: 0.14349262f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99144486f32,
                i: 0.13052619f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99306846f32,
                i: 0.11753740f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99452190f32,
                i: 0.10452846f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99580493f32,
                i: 0.091501619f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99691733f32,
                i: 0.078459096f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99785892f32,
                i: 0.065403129f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99862953f32,
                i: 0.052335956f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99922904f32,
                i: 0.039259816f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99965732f32,
                i: 0.026176948f32,
            };
            init
        },
        {
            let init = kiss_twiddle_cpx {
                r: 0.99991433f32,
                i: 0.013089596f32,
            };
            init
        },
    ];
    #[c2rust::src_loc = "348:25"]
    pub static mut fft_bitrev480: [i16; 480] = [
        0 as libc::c_int as i16,
        96 as libc::c_int as i16,
        192 as libc::c_int as i16,
        288 as libc::c_int as i16,
        384 as libc::c_int as i16,
        32 as libc::c_int as i16,
        128 as libc::c_int as i16,
        224 as libc::c_int as i16,
        320 as libc::c_int as i16,
        416 as libc::c_int as i16,
        64 as libc::c_int as i16,
        160 as libc::c_int as i16,
        256 as libc::c_int as i16,
        352 as libc::c_int as i16,
        448 as libc::c_int as i16,
        8 as libc::c_int as i16,
        104 as libc::c_int as i16,
        200 as libc::c_int as i16,
        296 as libc::c_int as i16,
        392 as libc::c_int as i16,
        40 as libc::c_int as i16,
        136 as libc::c_int as i16,
        232 as libc::c_int as i16,
        328 as libc::c_int as i16,
        424 as libc::c_int as i16,
        72 as libc::c_int as i16,
        168 as libc::c_int as i16,
        264 as libc::c_int as i16,
        360 as libc::c_int as i16,
        456 as libc::c_int as i16,
        16 as libc::c_int as i16,
        112 as libc::c_int as i16,
        208 as libc::c_int as i16,
        304 as libc::c_int as i16,
        400 as libc::c_int as i16,
        48 as libc::c_int as i16,
        144 as libc::c_int as i16,
        240 as libc::c_int as i16,
        336 as libc::c_int as i16,
        432 as libc::c_int as i16,
        80 as libc::c_int as i16,
        176 as libc::c_int as i16,
        272 as libc::c_int as i16,
        368 as libc::c_int as i16,
        464 as libc::c_int as i16,
        24 as libc::c_int as i16,
        120 as libc::c_int as i16,
        216 as libc::c_int as i16,
        312 as libc::c_int as i16,
        408 as libc::c_int as i16,
        56 as libc::c_int as i16,
        152 as libc::c_int as i16,
        248 as libc::c_int as i16,
        344 as libc::c_int as i16,
        440 as libc::c_int as i16,
        88 as libc::c_int as i16,
        184 as libc::c_int as i16,
        280 as libc::c_int as i16,
        376 as libc::c_int as i16,
        472 as libc::c_int as i16,
        4 as libc::c_int as i16,
        100 as libc::c_int as i16,
        196 as libc::c_int as i16,
        292 as libc::c_int as i16,
        388 as libc::c_int as i16,
        36 as libc::c_int as i16,
        132 as libc::c_int as i16,
        228 as libc::c_int as i16,
        324 as libc::c_int as i16,
        420 as libc::c_int as i16,
        68 as libc::c_int as i16,
        164 as libc::c_int as i16,
        260 as libc::c_int as i16,
        356 as libc::c_int as i16,
        452 as libc::c_int as i16,
        12 as libc::c_int as i16,
        108 as libc::c_int as i16,
        204 as libc::c_int as i16,
        300 as libc::c_int as i16,
        396 as libc::c_int as i16,
        44 as libc::c_int as i16,
        140 as libc::c_int as i16,
        236 as libc::c_int as i16,
        332 as libc::c_int as i16,
        428 as libc::c_int as i16,
        76 as libc::c_int as i16,
        172 as libc::c_int as i16,
        268 as libc::c_int as i16,
        364 as libc::c_int as i16,
        460 as libc::c_int as i16,
        20 as libc::c_int as i16,
        116 as libc::c_int as i16,
        212 as libc::c_int as i16,
        308 as libc::c_int as i16,
        404 as libc::c_int as i16,
        52 as libc::c_int as i16,
        148 as libc::c_int as i16,
        244 as libc::c_int as i16,
        340 as libc::c_int as i16,
        436 as libc::c_int as i16,
        84 as libc::c_int as i16,
        180 as libc::c_int as i16,
        276 as libc::c_int as i16,
        372 as libc::c_int as i16,
        468 as libc::c_int as i16,
        28 as libc::c_int as i16,
        124 as libc::c_int as i16,
        220 as libc::c_int as i16,
        316 as libc::c_int as i16,
        412 as libc::c_int as i16,
        60 as libc::c_int as i16,
        156 as libc::c_int as i16,
        252 as libc::c_int as i16,
        348 as libc::c_int as i16,
        444 as libc::c_int as i16,
        92 as libc::c_int as i16,
        188 as libc::c_int as i16,
        284 as libc::c_int as i16,
        380 as libc::c_int as i16,
        476 as libc::c_int as i16,
        1 as libc::c_int as i16,
        97 as libc::c_int as i16,
        193 as libc::c_int as i16,
        289 as libc::c_int as i16,
        385 as libc::c_int as i16,
        33 as libc::c_int as i16,
        129 as libc::c_int as i16,
        225 as libc::c_int as i16,
        321 as libc::c_int as i16,
        417 as libc::c_int as i16,
        65 as libc::c_int as i16,
        161 as libc::c_int as i16,
        257 as libc::c_int as i16,
        353 as libc::c_int as i16,
        449 as libc::c_int as i16,
        9 as libc::c_int as i16,
        105 as libc::c_int as i16,
        201 as libc::c_int as i16,
        297 as libc::c_int as i16,
        393 as libc::c_int as i16,
        41 as libc::c_int as i16,
        137 as libc::c_int as i16,
        233 as libc::c_int as i16,
        329 as libc::c_int as i16,
        425 as libc::c_int as i16,
        73 as libc::c_int as i16,
        169 as libc::c_int as i16,
        265 as libc::c_int as i16,
        361 as libc::c_int as i16,
        457 as libc::c_int as i16,
        17 as libc::c_int as i16,
        113 as libc::c_int as i16,
        209 as libc::c_int as i16,
        305 as libc::c_int as i16,
        401 as libc::c_int as i16,
        49 as libc::c_int as i16,
        145 as libc::c_int as i16,
        241 as libc::c_int as i16,
        337 as libc::c_int as i16,
        433 as libc::c_int as i16,
        81 as libc::c_int as i16,
        177 as libc::c_int as i16,
        273 as libc::c_int as i16,
        369 as libc::c_int as i16,
        465 as libc::c_int as i16,
        25 as libc::c_int as i16,
        121 as libc::c_int as i16,
        217 as libc::c_int as i16,
        313 as libc::c_int as i16,
        409 as libc::c_int as i16,
        57 as libc::c_int as i16,
        153 as libc::c_int as i16,
        249 as libc::c_int as i16,
        345 as libc::c_int as i16,
        441 as libc::c_int as i16,
        89 as libc::c_int as i16,
        185 as libc::c_int as i16,
        281 as libc::c_int as i16,
        377 as libc::c_int as i16,
        473 as libc::c_int as i16,
        5 as libc::c_int as i16,
        101 as libc::c_int as i16,
        197 as libc::c_int as i16,
        293 as libc::c_int as i16,
        389 as libc::c_int as i16,
        37 as libc::c_int as i16,
        133 as libc::c_int as i16,
        229 as libc::c_int as i16,
        325 as libc::c_int as i16,
        421 as libc::c_int as i16,
        69 as libc::c_int as i16,
        165 as libc::c_int as i16,
        261 as libc::c_int as i16,
        357 as libc::c_int as i16,
        453 as libc::c_int as i16,
        13 as libc::c_int as i16,
        109 as libc::c_int as i16,
        205 as libc::c_int as i16,
        301 as libc::c_int as i16,
        397 as libc::c_int as i16,
        45 as libc::c_int as i16,
        141 as libc::c_int as i16,
        237 as libc::c_int as i16,
        333 as libc::c_int as i16,
        429 as libc::c_int as i16,
        77 as libc::c_int as i16,
        173 as libc::c_int as i16,
        269 as libc::c_int as i16,
        365 as libc::c_int as i16,
        461 as libc::c_int as i16,
        21 as libc::c_int as i16,
        117 as libc::c_int as i16,
        213 as libc::c_int as i16,
        309 as libc::c_int as i16,
        405 as libc::c_int as i16,
        53 as libc::c_int as i16,
        149 as libc::c_int as i16,
        245 as libc::c_int as i16,
        341 as libc::c_int as i16,
        437 as libc::c_int as i16,
        85 as libc::c_int as i16,
        181 as libc::c_int as i16,
        277 as libc::c_int as i16,
        373 as libc::c_int as i16,
        469 as libc::c_int as i16,
        29 as libc::c_int as i16,
        125 as libc::c_int as i16,
        221 as libc::c_int as i16,
        317 as libc::c_int as i16,
        413 as libc::c_int as i16,
        61 as libc::c_int as i16,
        157 as libc::c_int as i16,
        253 as libc::c_int as i16,
        349 as libc::c_int as i16,
        445 as libc::c_int as i16,
        93 as libc::c_int as i16,
        189 as libc::c_int as i16,
        285 as libc::c_int as i16,
        381 as libc::c_int as i16,
        477 as libc::c_int as i16,
        2 as libc::c_int as i16,
        98 as libc::c_int as i16,
        194 as libc::c_int as i16,
        290 as libc::c_int as i16,
        386 as libc::c_int as i16,
        34 as libc::c_int as i16,
        130 as libc::c_int as i16,
        226 as libc::c_int as i16,
        322 as libc::c_int as i16,
        418 as libc::c_int as i16,
        66 as libc::c_int as i16,
        162 as libc::c_int as i16,
        258 as libc::c_int as i16,
        354 as libc::c_int as i16,
        450 as libc::c_int as i16,
        10 as libc::c_int as i16,
        106 as libc::c_int as i16,
        202 as libc::c_int as i16,
        298 as libc::c_int as i16,
        394 as libc::c_int as i16,
        42 as libc::c_int as i16,
        138 as libc::c_int as i16,
        234 as libc::c_int as i16,
        330 as libc::c_int as i16,
        426 as libc::c_int as i16,
        74 as libc::c_int as i16,
        170 as libc::c_int as i16,
        266 as libc::c_int as i16,
        362 as libc::c_int as i16,
        458 as libc::c_int as i16,
        18 as libc::c_int as i16,
        114 as libc::c_int as i16,
        210 as libc::c_int as i16,
        306 as libc::c_int as i16,
        402 as libc::c_int as i16,
        50 as libc::c_int as i16,
        146 as libc::c_int as i16,
        242 as libc::c_int as i16,
        338 as libc::c_int as i16,
        434 as libc::c_int as i16,
        82 as libc::c_int as i16,
        178 as libc::c_int as i16,
        274 as libc::c_int as i16,
        370 as libc::c_int as i16,
        466 as libc::c_int as i16,
        26 as libc::c_int as i16,
        122 as libc::c_int as i16,
        218 as libc::c_int as i16,
        314 as libc::c_int as i16,
        410 as libc::c_int as i16,
        58 as libc::c_int as i16,
        154 as libc::c_int as i16,
        250 as libc::c_int as i16,
        346 as libc::c_int as i16,
        442 as libc::c_int as i16,
        90 as libc::c_int as i16,
        186 as libc::c_int as i16,
        282 as libc::c_int as i16,
        378 as libc::c_int as i16,
        474 as libc::c_int as i16,
        6 as libc::c_int as i16,
        102 as libc::c_int as i16,
        198 as libc::c_int as i16,
        294 as libc::c_int as i16,
        390 as libc::c_int as i16,
        38 as libc::c_int as i16,
        134 as libc::c_int as i16,
        230 as libc::c_int as i16,
        326 as libc::c_int as i16,
        422 as libc::c_int as i16,
        70 as libc::c_int as i16,
        166 as libc::c_int as i16,
        262 as libc::c_int as i16,
        358 as libc::c_int as i16,
        454 as libc::c_int as i16,
        14 as libc::c_int as i16,
        110 as libc::c_int as i16,
        206 as libc::c_int as i16,
        302 as libc::c_int as i16,
        398 as libc::c_int as i16,
        46 as libc::c_int as i16,
        142 as libc::c_int as i16,
        238 as libc::c_int as i16,
        334 as libc::c_int as i16,
        430 as libc::c_int as i16,
        78 as libc::c_int as i16,
        174 as libc::c_int as i16,
        270 as libc::c_int as i16,
        366 as libc::c_int as i16,
        462 as libc::c_int as i16,
        22 as libc::c_int as i16,
        118 as libc::c_int as i16,
        214 as libc::c_int as i16,
        310 as libc::c_int as i16,
        406 as libc::c_int as i16,
        54 as libc::c_int as i16,
        150 as libc::c_int as i16,
        246 as libc::c_int as i16,
        342 as libc::c_int as i16,
        438 as libc::c_int as i16,
        86 as libc::c_int as i16,
        182 as libc::c_int as i16,
        278 as libc::c_int as i16,
        374 as libc::c_int as i16,
        470 as libc::c_int as i16,
        30 as libc::c_int as i16,
        126 as libc::c_int as i16,
        222 as libc::c_int as i16,
        318 as libc::c_int as i16,
        414 as libc::c_int as i16,
        62 as libc::c_int as i16,
        158 as libc::c_int as i16,
        254 as libc::c_int as i16,
        350 as libc::c_int as i16,
        446 as libc::c_int as i16,
        94 as libc::c_int as i16,
        190 as libc::c_int as i16,
        286 as libc::c_int as i16,
        382 as libc::c_int as i16,
        478 as libc::c_int as i16,
        3 as libc::c_int as i16,
        99 as libc::c_int as i16,
        195 as libc::c_int as i16,
        291 as libc::c_int as i16,
        387 as libc::c_int as i16,
        35 as libc::c_int as i16,
        131 as libc::c_int as i16,
        227 as libc::c_int as i16,
        323 as libc::c_int as i16,
        419 as libc::c_int as i16,
        67 as libc::c_int as i16,
        163 as libc::c_int as i16,
        259 as libc::c_int as i16,
        355 as libc::c_int as i16,
        451 as libc::c_int as i16,
        11 as libc::c_int as i16,
        107 as libc::c_int as i16,
        203 as libc::c_int as i16,
        299 as libc::c_int as i16,
        395 as libc::c_int as i16,
        43 as libc::c_int as i16,
        139 as libc::c_int as i16,
        235 as libc::c_int as i16,
        331 as libc::c_int as i16,
        427 as libc::c_int as i16,
        75 as libc::c_int as i16,
        171 as libc::c_int as i16,
        267 as libc::c_int as i16,
        363 as libc::c_int as i16,
        459 as libc::c_int as i16,
        19 as libc::c_int as i16,
        115 as libc::c_int as i16,
        211 as libc::c_int as i16,
        307 as libc::c_int as i16,
        403 as libc::c_int as i16,
        51 as libc::c_int as i16,
        147 as libc::c_int as i16,
        243 as libc::c_int as i16,
        339 as libc::c_int as i16,
        435 as libc::c_int as i16,
        83 as libc::c_int as i16,
        179 as libc::c_int as i16,
        275 as libc::c_int as i16,
        371 as libc::c_int as i16,
        467 as libc::c_int as i16,
        27 as libc::c_int as i16,
        123 as libc::c_int as i16,
        219 as libc::c_int as i16,
        315 as libc::c_int as i16,
        411 as libc::c_int as i16,
        59 as libc::c_int as i16,
        155 as libc::c_int as i16,
        251 as libc::c_int as i16,
        347 as libc::c_int as i16,
        443 as libc::c_int as i16,
        91 as libc::c_int as i16,
        187 as libc::c_int as i16,
        283 as libc::c_int as i16,
        379 as libc::c_int as i16,
        475 as libc::c_int as i16,
        7 as libc::c_int as i16,
        103 as libc::c_int as i16,
        199 as libc::c_int as i16,
        295 as libc::c_int as i16,
        391 as libc::c_int as i16,
        39 as libc::c_int as i16,
        135 as libc::c_int as i16,
        231 as libc::c_int as i16,
        327 as libc::c_int as i16,
        423 as libc::c_int as i16,
        71 as libc::c_int as i16,
        167 as libc::c_int as i16,
        263 as libc::c_int as i16,
        359 as libc::c_int as i16,
        455 as libc::c_int as i16,
        15 as libc::c_int as i16,
        111 as libc::c_int as i16,
        207 as libc::c_int as i16,
        303 as libc::c_int as i16,
        399 as libc::c_int as i16,
        47 as libc::c_int as i16,
        143 as libc::c_int as i16,
        239 as libc::c_int as i16,
        335 as libc::c_int as i16,
        431 as libc::c_int as i16,
        79 as libc::c_int as i16,
        175 as libc::c_int as i16,
        271 as libc::c_int as i16,
        367 as libc::c_int as i16,
        463 as libc::c_int as i16,
        23 as libc::c_int as i16,
        119 as libc::c_int as i16,
        215 as libc::c_int as i16,
        311 as libc::c_int as i16,
        407 as libc::c_int as i16,
        55 as libc::c_int as i16,
        151 as libc::c_int as i16,
        247 as libc::c_int as i16,
        343 as libc::c_int as i16,
        439 as libc::c_int as i16,
        87 as libc::c_int as i16,
        183 as libc::c_int as i16,
        279 as libc::c_int as i16,
        375 as libc::c_int as i16,
        471 as libc::c_int as i16,
        31 as libc::c_int as i16,
        127 as libc::c_int as i16,
        223 as libc::c_int as i16,
        319 as libc::c_int as i16,
        415 as libc::c_int as i16,
        63 as libc::c_int as i16,
        159 as libc::c_int as i16,
        255 as libc::c_int as i16,
        351 as libc::c_int as i16,
        447 as libc::c_int as i16,
        95 as libc::c_int as i16,
        191 as libc::c_int as i16,
        287 as libc::c_int as i16,
        383 as libc::c_int as i16,
        479 as libc::c_int as i16,
    ];
    #[c2rust::src_loc = "386:25"]
    pub static mut fft_bitrev240: [i16; 240] = [
        0 as libc::c_int as i16,
        48 as libc::c_int as i16,
        96 as libc::c_int as i16,
        144 as libc::c_int as i16,
        192 as libc::c_int as i16,
        16 as libc::c_int as i16,
        64 as libc::c_int as i16,
        112 as libc::c_int as i16,
        160 as libc::c_int as i16,
        208 as libc::c_int as i16,
        32 as libc::c_int as i16,
        80 as libc::c_int as i16,
        128 as libc::c_int as i16,
        176 as libc::c_int as i16,
        224 as libc::c_int as i16,
        4 as libc::c_int as i16,
        52 as libc::c_int as i16,
        100 as libc::c_int as i16,
        148 as libc::c_int as i16,
        196 as libc::c_int as i16,
        20 as libc::c_int as i16,
        68 as libc::c_int as i16,
        116 as libc::c_int as i16,
        164 as libc::c_int as i16,
        212 as libc::c_int as i16,
        36 as libc::c_int as i16,
        84 as libc::c_int as i16,
        132 as libc::c_int as i16,
        180 as libc::c_int as i16,
        228 as libc::c_int as i16,
        8 as libc::c_int as i16,
        56 as libc::c_int as i16,
        104 as libc::c_int as i16,
        152 as libc::c_int as i16,
        200 as libc::c_int as i16,
        24 as libc::c_int as i16,
        72 as libc::c_int as i16,
        120 as libc::c_int as i16,
        168 as libc::c_int as i16,
        216 as libc::c_int as i16,
        40 as libc::c_int as i16,
        88 as libc::c_int as i16,
        136 as libc::c_int as i16,
        184 as libc::c_int as i16,
        232 as libc::c_int as i16,
        12 as libc::c_int as i16,
        60 as libc::c_int as i16,
        108 as libc::c_int as i16,
        156 as libc::c_int as i16,
        204 as libc::c_int as i16,
        28 as libc::c_int as i16,
        76 as libc::c_int as i16,
        124 as libc::c_int as i16,
        172 as libc::c_int as i16,
        220 as libc::c_int as i16,
        44 as libc::c_int as i16,
        92 as libc::c_int as i16,
        140 as libc::c_int as i16,
        188 as libc::c_int as i16,
        236 as libc::c_int as i16,
        1 as libc::c_int as i16,
        49 as libc::c_int as i16,
        97 as libc::c_int as i16,
        145 as libc::c_int as i16,
        193 as libc::c_int as i16,
        17 as libc::c_int as i16,
        65 as libc::c_int as i16,
        113 as libc::c_int as i16,
        161 as libc::c_int as i16,
        209 as libc::c_int as i16,
        33 as libc::c_int as i16,
        81 as libc::c_int as i16,
        129 as libc::c_int as i16,
        177 as libc::c_int as i16,
        225 as libc::c_int as i16,
        5 as libc::c_int as i16,
        53 as libc::c_int as i16,
        101 as libc::c_int as i16,
        149 as libc::c_int as i16,
        197 as libc::c_int as i16,
        21 as libc::c_int as i16,
        69 as libc::c_int as i16,
        117 as libc::c_int as i16,
        165 as libc::c_int as i16,
        213 as libc::c_int as i16,
        37 as libc::c_int as i16,
        85 as libc::c_int as i16,
        133 as libc::c_int as i16,
        181 as libc::c_int as i16,
        229 as libc::c_int as i16,
        9 as libc::c_int as i16,
        57 as libc::c_int as i16,
        105 as libc::c_int as i16,
        153 as libc::c_int as i16,
        201 as libc::c_int as i16,
        25 as libc::c_int as i16,
        73 as libc::c_int as i16,
        121 as libc::c_int as i16,
        169 as libc::c_int as i16,
        217 as libc::c_int as i16,
        41 as libc::c_int as i16,
        89 as libc::c_int as i16,
        137 as libc::c_int as i16,
        185 as libc::c_int as i16,
        233 as libc::c_int as i16,
        13 as libc::c_int as i16,
        61 as libc::c_int as i16,
        109 as libc::c_int as i16,
        157 as libc::c_int as i16,
        205 as libc::c_int as i16,
        29 as libc::c_int as i16,
        77 as libc::c_int as i16,
        125 as libc::c_int as i16,
        173 as libc::c_int as i16,
        221 as libc::c_int as i16,
        45 as libc::c_int as i16,
        93 as libc::c_int as i16,
        141 as libc::c_int as i16,
        189 as libc::c_int as i16,
        237 as libc::c_int as i16,
        2 as libc::c_int as i16,
        50 as libc::c_int as i16,
        98 as libc::c_int as i16,
        146 as libc::c_int as i16,
        194 as libc::c_int as i16,
        18 as libc::c_int as i16,
        66 as libc::c_int as i16,
        114 as libc::c_int as i16,
        162 as libc::c_int as i16,
        210 as libc::c_int as i16,
        34 as libc::c_int as i16,
        82 as libc::c_int as i16,
        130 as libc::c_int as i16,
        178 as libc::c_int as i16,
        226 as libc::c_int as i16,
        6 as libc::c_int as i16,
        54 as libc::c_int as i16,
        102 as libc::c_int as i16,
        150 as libc::c_int as i16,
        198 as libc::c_int as i16,
        22 as libc::c_int as i16,
        70 as libc::c_int as i16,
        118 as libc::c_int as i16,
        166 as libc::c_int as i16,
        214 as libc::c_int as i16,
        38 as libc::c_int as i16,
        86 as libc::c_int as i16,
        134 as libc::c_int as i16,
        182 as libc::c_int as i16,
        230 as libc::c_int as i16,
        10 as libc::c_int as i16,
        58 as libc::c_int as i16,
        106 as libc::c_int as i16,
        154 as libc::c_int as i16,
        202 as libc::c_int as i16,
        26 as libc::c_int as i16,
        74 as libc::c_int as i16,
        122 as libc::c_int as i16,
        170 as libc::c_int as i16,
        218 as libc::c_int as i16,
        42 as libc::c_int as i16,
        90 as libc::c_int as i16,
        138 as libc::c_int as i16,
        186 as libc::c_int as i16,
        234 as libc::c_int as i16,
        14 as libc::c_int as i16,
        62 as libc::c_int as i16,
        110 as libc::c_int as i16,
        158 as libc::c_int as i16,
        206 as libc::c_int as i16,
        30 as libc::c_int as i16,
        78 as libc::c_int as i16,
        126 as libc::c_int as i16,
        174 as libc::c_int as i16,
        222 as libc::c_int as i16,
        46 as libc::c_int as i16,
        94 as libc::c_int as i16,
        142 as libc::c_int as i16,
        190 as libc::c_int as i16,
        238 as libc::c_int as i16,
        3 as libc::c_int as i16,
        51 as libc::c_int as i16,
        99 as libc::c_int as i16,
        147 as libc::c_int as i16,
        195 as libc::c_int as i16,
        19 as libc::c_int as i16,
        67 as libc::c_int as i16,
        115 as libc::c_int as i16,
        163 as libc::c_int as i16,
        211 as libc::c_int as i16,
        35 as libc::c_int as i16,
        83 as libc::c_int as i16,
        131 as libc::c_int as i16,
        179 as libc::c_int as i16,
        227 as libc::c_int as i16,
        7 as libc::c_int as i16,
        55 as libc::c_int as i16,
        103 as libc::c_int as i16,
        151 as libc::c_int as i16,
        199 as libc::c_int as i16,
        23 as libc::c_int as i16,
        71 as libc::c_int as i16,
        119 as libc::c_int as i16,
        167 as libc::c_int as i16,
        215 as libc::c_int as i16,
        39 as libc::c_int as i16,
        87 as libc::c_int as i16,
        135 as libc::c_int as i16,
        183 as libc::c_int as i16,
        231 as libc::c_int as i16,
        11 as libc::c_int as i16,
        59 as libc::c_int as i16,
        107 as libc::c_int as i16,
        155 as libc::c_int as i16,
        203 as libc::c_int as i16,
        27 as libc::c_int as i16,
        75 as libc::c_int as i16,
        123 as libc::c_int as i16,
        171 as libc::c_int as i16,
        219 as libc::c_int as i16,
        43 as libc::c_int as i16,
        91 as libc::c_int as i16,
        139 as libc::c_int as i16,
        187 as libc::c_int as i16,
        235 as libc::c_int as i16,
        15 as libc::c_int as i16,
        63 as libc::c_int as i16,
        111 as libc::c_int as i16,
        159 as libc::c_int as i16,
        207 as libc::c_int as i16,
        31 as libc::c_int as i16,
        79 as libc::c_int as i16,
        127 as libc::c_int as i16,
        175 as libc::c_int as i16,
        223 as libc::c_int as i16,
        47 as libc::c_int as i16,
        95 as libc::c_int as i16,
        143 as libc::c_int as i16,
        191 as libc::c_int as i16,
        239 as libc::c_int as i16,
    ];
    #[c2rust::src_loc = "408:25"]
    pub static mut fft_bitrev120: [i16; 120] = [
        0 as libc::c_int as i16,
        24 as libc::c_int as i16,
        48 as libc::c_int as i16,
        72 as libc::c_int as i16,
        96 as libc::c_int as i16,
        8 as libc::c_int as i16,
        32 as libc::c_int as i16,
        56 as libc::c_int as i16,
        80 as libc::c_int as i16,
        104 as libc::c_int as i16,
        16 as libc::c_int as i16,
        40 as libc::c_int as i16,
        64 as libc::c_int as i16,
        88 as libc::c_int as i16,
        112 as libc::c_int as i16,
        4 as libc::c_int as i16,
        28 as libc::c_int as i16,
        52 as libc::c_int as i16,
        76 as libc::c_int as i16,
        100 as libc::c_int as i16,
        12 as libc::c_int as i16,
        36 as libc::c_int as i16,
        60 as libc::c_int as i16,
        84 as libc::c_int as i16,
        108 as libc::c_int as i16,
        20 as libc::c_int as i16,
        44 as libc::c_int as i16,
        68 as libc::c_int as i16,
        92 as libc::c_int as i16,
        116 as libc::c_int as i16,
        1 as libc::c_int as i16,
        25 as libc::c_int as i16,
        49 as libc::c_int as i16,
        73 as libc::c_int as i16,
        97 as libc::c_int as i16,
        9 as libc::c_int as i16,
        33 as libc::c_int as i16,
        57 as libc::c_int as i16,
        81 as libc::c_int as i16,
        105 as libc::c_int as i16,
        17 as libc::c_int as i16,
        41 as libc::c_int as i16,
        65 as libc::c_int as i16,
        89 as libc::c_int as i16,
        113 as libc::c_int as i16,
        5 as libc::c_int as i16,
        29 as libc::c_int as i16,
        53 as libc::c_int as i16,
        77 as libc::c_int as i16,
        101 as libc::c_int as i16,
        13 as libc::c_int as i16,
        37 as libc::c_int as i16,
        61 as libc::c_int as i16,
        85 as libc::c_int as i16,
        109 as libc::c_int as i16,
        21 as libc::c_int as i16,
        45 as libc::c_int as i16,
        69 as libc::c_int as i16,
        93 as libc::c_int as i16,
        117 as libc::c_int as i16,
        2 as libc::c_int as i16,
        26 as libc::c_int as i16,
        50 as libc::c_int as i16,
        74 as libc::c_int as i16,
        98 as libc::c_int as i16,
        10 as libc::c_int as i16,
        34 as libc::c_int as i16,
        58 as libc::c_int as i16,
        82 as libc::c_int as i16,
        106 as libc::c_int as i16,
        18 as libc::c_int as i16,
        42 as libc::c_int as i16,
        66 as libc::c_int as i16,
        90 as libc::c_int as i16,
        114 as libc::c_int as i16,
        6 as libc::c_int as i16,
        30 as libc::c_int as i16,
        54 as libc::c_int as i16,
        78 as libc::c_int as i16,
        102 as libc::c_int as i16,
        14 as libc::c_int as i16,
        38 as libc::c_int as i16,
        62 as libc::c_int as i16,
        86 as libc::c_int as i16,
        110 as libc::c_int as i16,
        22 as libc::c_int as i16,
        46 as libc::c_int as i16,
        70 as libc::c_int as i16,
        94 as libc::c_int as i16,
        118 as libc::c_int as i16,
        3 as libc::c_int as i16,
        27 as libc::c_int as i16,
        51 as libc::c_int as i16,
        75 as libc::c_int as i16,
        99 as libc::c_int as i16,
        11 as libc::c_int as i16,
        35 as libc::c_int as i16,
        59 as libc::c_int as i16,
        83 as libc::c_int as i16,
        107 as libc::c_int as i16,
        19 as libc::c_int as i16,
        43 as libc::c_int as i16,
        67 as libc::c_int as i16,
        91 as libc::c_int as i16,
        115 as libc::c_int as i16,
        7 as libc::c_int as i16,
        31 as libc::c_int as i16,
        55 as libc::c_int as i16,
        79 as libc::c_int as i16,
        103 as libc::c_int as i16,
        15 as libc::c_int as i16,
        39 as libc::c_int as i16,
        63 as libc::c_int as i16,
        87 as libc::c_int as i16,
        111 as libc::c_int as i16,
        23 as libc::c_int as i16,
        47 as libc::c_int as i16,
        71 as libc::c_int as i16,
        95 as libc::c_int as i16,
        119 as libc::c_int as i16,
    ];
    #[c2rust::src_loc = "422:25"]
    pub static mut fft_bitrev60: [i16; 60] = [
        0 as libc::c_int as i16,
        12 as libc::c_int as i16,
        24 as libc::c_int as i16,
        36 as libc::c_int as i16,
        48 as libc::c_int as i16,
        4 as libc::c_int as i16,
        16 as libc::c_int as i16,
        28 as libc::c_int as i16,
        40 as libc::c_int as i16,
        52 as libc::c_int as i16,
        8 as libc::c_int as i16,
        20 as libc::c_int as i16,
        32 as libc::c_int as i16,
        44 as libc::c_int as i16,
        56 as libc::c_int as i16,
        1 as libc::c_int as i16,
        13 as libc::c_int as i16,
        25 as libc::c_int as i16,
        37 as libc::c_int as i16,
        49 as libc::c_int as i16,
        5 as libc::c_int as i16,
        17 as libc::c_int as i16,
        29 as libc::c_int as i16,
        41 as libc::c_int as i16,
        53 as libc::c_int as i16,
        9 as libc::c_int as i16,
        21 as libc::c_int as i16,
        33 as libc::c_int as i16,
        45 as libc::c_int as i16,
        57 as libc::c_int as i16,
        2 as libc::c_int as i16,
        14 as libc::c_int as i16,
        26 as libc::c_int as i16,
        38 as libc::c_int as i16,
        50 as libc::c_int as i16,
        6 as libc::c_int as i16,
        18 as libc::c_int as i16,
        30 as libc::c_int as i16,
        42 as libc::c_int as i16,
        54 as libc::c_int as i16,
        10 as libc::c_int as i16,
        22 as libc::c_int as i16,
        34 as libc::c_int as i16,
        46 as libc::c_int as i16,
        58 as libc::c_int as i16,
        3 as libc::c_int as i16,
        15 as libc::c_int as i16,
        27 as libc::c_int as i16,
        39 as libc::c_int as i16,
        51 as libc::c_int as i16,
        7 as libc::c_int as i16,
        19 as libc::c_int as i16,
        31 as libc::c_int as i16,
        43 as libc::c_int as i16,
        55 as libc::c_int as i16,
        11 as libc::c_int as i16,
        23 as libc::c_int as i16,
        35 as libc::c_int as i16,
        47 as libc::c_int as i16,
        59 as libc::c_int as i16,
    ];
    #[c2rust::src_loc = "432:29"]
    pub static mut fft_state48000_960_0: kiss_fft_state = unsafe {
        {
            let init = kiss_fft_state {
                nfft: 480 as libc::c_int,
                scale: 0.002083333f32,
                shift: -(1 as libc::c_int),
                factors: [
                    5 as libc::c_int as i16,
                    96 as libc::c_int as i16,
                    3 as libc::c_int as i16,
                    32 as libc::c_int as i16,
                    4 as libc::c_int as i16,
                    8 as libc::c_int as i16,
                    2 as libc::c_int as i16,
                    4 as libc::c_int as i16,
                    4 as libc::c_int as i16,
                    1 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                ],
                bitrev: fft_bitrev480.as_ptr(),
                twiddles: fft_twiddles48000_960.as_ptr(),
                arch_fft: NULL as *mut arch_fft_state,
            };
            init
        }
    };
    #[c2rust::src_loc = "449:29"]
    pub static mut fft_state48000_960_1: kiss_fft_state = unsafe {
        {
            let init = kiss_fft_state {
                nfft: 240 as libc::c_int,
                scale: 0.004166667f32,
                shift: 1 as libc::c_int,
                factors: [
                    5 as libc::c_int as i16,
                    48 as libc::c_int as i16,
                    3 as libc::c_int as i16,
                    16 as libc::c_int as i16,
                    4 as libc::c_int as i16,
                    4 as libc::c_int as i16,
                    4 as libc::c_int as i16,
                    1 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                ],
                bitrev: fft_bitrev240.as_ptr(),
                twiddles: fft_twiddles48000_960.as_ptr(),
                arch_fft: NULL as *mut arch_fft_state,
            };
            init
        }
    };
    #[c2rust::src_loc = "466:29"]
    pub static mut fft_state48000_960_2: kiss_fft_state = unsafe {
        {
            let init = kiss_fft_state {
                nfft: 120 as libc::c_int,
                scale: 0.008333333f32,
                shift: 2 as libc::c_int,
                factors: [
                    5 as libc::c_int as i16,
                    24 as libc::c_int as i16,
                    3 as libc::c_int as i16,
                    8 as libc::c_int as i16,
                    2 as libc::c_int as i16,
                    4 as libc::c_int as i16,
                    4 as libc::c_int as i16,
                    1 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                ],
                bitrev: fft_bitrev120.as_ptr(),
                twiddles: fft_twiddles48000_960.as_ptr(),
                arch_fft: NULL as *mut arch_fft_state,
            };
            init
        }
    };
    #[c2rust::src_loc = "483:29"]
    pub static mut fft_state48000_960_3: kiss_fft_state = unsafe {
        {
            let init = kiss_fft_state {
                nfft: 60 as libc::c_int,
                scale: 0.016666667f32,
                shift: 3 as libc::c_int,
                factors: [
                    5 as libc::c_int as i16,
                    12 as libc::c_int as i16,
                    3 as libc::c_int as i16,
                    4 as libc::c_int as i16,
                    4 as libc::c_int as i16,
                    1 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                    0 as libc::c_int as i16,
                ],
                bitrev: fft_bitrev60.as_ptr(),
                twiddles: fft_twiddles48000_960.as_ptr(),
                arch_fft: NULL as *mut arch_fft_state,
            };
            init
        }
    };
    #[c2rust::src_loc = "502:25"]
    pub static mut mdct_twiddles960: [opus_val16; 1800] = [
        0.99999994f32,
        0.99999321f32,
        0.99997580f32,
        0.99994773f32,
        0.99990886f32,
        0.99985933f32,
        0.99979913f32,
        0.99972820f32,
        0.99964654f32,
        0.99955416f32,
        0.99945110f32,
        0.99933738f32,
        0.99921292f32,
        0.99907774f32,
        0.99893188f32,
        0.99877530f32,
        0.99860805f32,
        0.99843007f32,
        0.99824142f32,
        0.99804211f32,
        0.99783206f32,
        0.99761140f32,
        0.99737996f32,
        0.99713790f32,
        0.99688518f32,
        0.99662173f32,
        0.99634761f32,
        0.99606287f32,
        0.99576741f32,
        0.99546129f32,
        0.99514455f32,
        0.99481714f32,
        0.99447906f32,
        0.99413031f32,
        0.99377096f32,
        0.99340093f32,
        0.99302030f32,
        0.99262899f32,
        0.99222708f32,
        0.99181455f32,
        0.99139136f32,
        0.99095762f32,
        0.99051321f32,
        0.99005818f32,
        0.98959261f32,
        0.98911643f32,
        0.98862964f32,
        0.98813224f32,
        0.98762429f32,
        0.98710573f32,
        0.98657662f32,
        0.98603696f32,
        0.98548669f32,
        0.98492593f32,
        0.98435456f32,
        0.98377270f32,
        0.98318028f32,
        0.98257732f32,
        0.98196387f32,
        0.98133987f32,
        0.98070538f32,
        0.98006040f32,
        0.97940493f32,
        0.97873890f32,
        0.97806245f32,
        0.97737551f32,
        0.97667813f32,
        0.97597027f32,
        0.97525197f32,
        0.97452319f32,
        0.97378403f32,
        0.97303438f32,
        0.97227436f32,
        0.97150391f32,
        0.97072303f32,
        0.96993178f32,
        0.96913016f32,
        0.96831810f32,
        0.96749574f32,
        0.96666300f32,
        0.96581990f32,
        0.96496642f32,
        0.96410263f32,
        0.96322852f32,
        0.96234411f32,
        0.96144938f32,
        0.96054435f32,
        0.95962906f32,
        0.95870346f32,
        0.95776761f32,
        0.95682150f32,
        0.95586514f32,
        0.95489854f32,
        0.95392174f32,
        0.95293468f32,
        0.95193744f32,
        0.95093000f32,
        0.94991243f32,
        0.94888461f32,
        0.94784665f32,
        0.94679856f32,
        0.94574034f32,
        0.94467193f32,
        0.94359344f32,
        0.94250488f32,
        0.94140619f32,
        0.94029742f32,
        0.93917859f32,
        0.93804967f32,
        0.93691075f32,
        0.93576175f32,
        0.93460274f32,
        0.93343377f32,
        0.93225473f32,
        0.93106574f32,
        0.92986679f32,
        0.92865789f32,
        0.92743903f32,
        0.92621022f32,
        0.92497152f32,
        0.92372292f32,
        0.92246443f32,
        0.92119598f32,
        0.91991776f32,
        0.91862965f32,
        0.91733170f32,
        0.91602397f32,
        0.91470635f32,
        0.91337901f32,
        0.91204184f32,
        0.91069490f32,
        0.90933824f32,
        0.90797186f32,
        0.90659571f32,
        0.90520984f32,
        0.90381432f32,
        0.90240908f32,
        0.90099424f32,
        0.89956969f32,
        0.89813554f32,
        0.89669174f32,
        0.89523834f32,
        0.89377540f32,
        0.89230281f32,
        0.89082074f32,
        0.88932908f32,
        0.88782793f32,
        0.88631725f32,
        0.88479710f32,
        0.88326746f32,
        0.88172835f32,
        0.88017982f32,
        0.87862182f32,
        0.87705445f32,
        0.87547767f32,
        0.87389153f32,
        0.87229604f32,
        0.87069118f32,
        0.86907703f32,
        0.86745358f32,
        0.86582077f32,
        0.86417878f32,
        0.86252749f32,
        0.86086690f32,
        0.85919720f32,
        0.85751826f32,
        0.85583007f32,
        0.85413277f32,
        0.85242635f32,
        0.85071075f32,
        0.84898609f32,
        0.84725231f32,
        0.84550947f32,
        0.84375757f32,
        0.84199661f32,
        0.84022665f32,
        0.83844769f32,
        0.83665979f32,
        0.83486289f32,
        0.83305705f32,
        0.83124226f32,
        0.82941860f32,
        0.82758605f32,
        0.82574469f32,
        0.82389444f32,
        0.82203537f32,
        0.82016748f32,
        0.81829083f32,
        0.81640542f32,
        0.81451124f32,
        0.81260836f32,
        0.81069672f32,
        0.80877650f32,
        0.80684757f32,
        0.80490994f32,
        0.80296379f32,
        0.80100900f32,
        0.79904562f32,
        0.79707366f32,
        0.79509324f32,
        0.79310423f32,
        0.79110676f32,
        0.78910083f32,
        0.78708643f32,
        0.78506362f32,
        0.78303236f32,
        0.78099275f32,
        0.77894479f32,
        0.77688843f32,
        0.77482378f32,
        0.77275085f32,
        0.77066964f32,
        0.76858020f32,
        0.76648247f32,
        0.76437658f32,
        0.76226246f32,
        0.76014024f32,
        0.75800985f32,
        0.75587130f32,
        0.75372469f32,
        0.75157005f32,
        0.74940729f32,
        0.74723655f32,
        0.74505776f32,
        0.74287105f32,
        0.74067634f32,
        0.73847371f32,
        0.73626316f32,
        0.73404479f32,
        0.73181850f32,
        0.72958434f32,
        0.72734243f32,
        0.72509271f32,
        0.72283524f32,
        0.72057003f32,
        0.71829706f32,
        0.71601641f32,
        0.71372813f32,
        0.71143216f32,
        0.70912862f32,
        0.70681745f32,
        0.70449871f32,
        0.70217246f32,
        0.69983864f32,
        0.69749737f32,
        0.69514859f32,
        0.69279242f32,
        0.69042879f32,
        0.68805778f32,
        0.68567938f32,
        0.68329364f32,
        0.68090063f32,
        0.67850029f32,
        0.67609268f32,
        0.67367786f32,
        0.67125577f32,
        0.66882652f32,
        0.66639012f32,
        0.66394657f32,
        0.66149592f32,
        0.65903819f32,
        0.65657341f32,
        0.65410155f32,
        0.65162271f32,
        0.64913690f32,
        0.64664418f32,
        0.64414448f32,
        0.64163786f32,
        0.63912445f32,
        0.63660413f32,
        0.63407701f32,
        0.63154310f32,
        0.62900239f32,
        0.62645501f32,
        0.62390089f32,
        0.62134010f32,
        0.61877263f32,
        0.61619854f32,
        0.61361790f32,
        0.61103064f32,
        0.60843682f32,
        0.60583651f32,
        0.60322970f32,
        0.60061646f32,
        0.59799677f32,
        0.59537065f32,
        0.59273821f32,
        0.59009939f32,
        0.58745426f32,
        0.58480281f32,
        0.58214509f32,
        0.57948118f32,
        0.57681108f32,
        0.57413477f32,
        0.57145232f32,
        0.56876373f32,
        0.56606907f32,
        0.56336832f32,
        0.56066155f32,
        0.55794877f32,
        0.55523002f32,
        0.55250537f32,
        0.54977477f32,
        0.54703826f32,
        0.54429591f32,
        0.54154772f32,
        0.53879374f32,
        0.53603399f32,
        0.53326851f32,
        0.53049731f32,
        0.52772039f32,
        0.52493787f32,
        0.52214974f32,
        0.51935595f32,
        0.51655668f32,
        0.51375180f32,
        0.51094145f32,
        0.50812566f32,
        0.50530440f32,
        0.50247771f32,
        0.49964568f32,
        0.49680826f32,
        0.49396557f32,
        0.49111754f32,
        0.48826426f32,
        0.48540577f32,
        0.48254207f32,
        0.47967321f32,
        0.47679919f32,
        0.47392011f32,
        0.47103590f32,
        0.46814668f32,
        0.46525243f32,
        0.46235323f32,
        0.45944905f32,
        0.45653993f32,
        0.45362595f32,
        0.45070711f32,
        0.44778344f32,
        0.44485497f32,
        0.44192174f32,
        0.43898380f32,
        0.43604112f32,
        0.43309379f32,
        0.43014181f32,
        0.42718524f32,
        0.42422408f32,
        0.42125839f32,
        0.41828820f32,
        0.41531351f32,
        0.41233435f32,
        0.40935081f32,
        0.40636289f32,
        0.40337059f32,
        0.40037400f32,
        0.39737311f32,
        0.39436796f32,
        0.39135858f32,
        0.38834500f32,
        0.38532731f32,
        0.38230544f32,
        0.37927949f32,
        0.37624949f32,
        0.37321547f32,
        0.37017745f32,
        0.36713544f32,
        0.36408952f32,
        0.36103970f32,
        0.35798600f32,
        0.35492846f32,
        0.35186714f32,
        0.34880206f32,
        0.34573323f32,
        0.34266070f32,
        0.33958447f32,
        0.33650464f32,
        0.33342120f32,
        0.33033419f32,
        0.32724363f32,
        0.32414958f32,
        0.32105204f32,
        0.31795108f32,
        0.31484672f32,
        0.31173897f32,
        0.30862790f32,
        0.30551350f32,
        0.30239585f32,
        0.29927495f32,
        0.29615086f32,
        0.29302359f32,
        0.28989318f32,
        0.28675964f32,
        0.28362307f32,
        0.28048345f32,
        0.27734083f32,
        0.27419522f32,
        0.27104670f32,
        0.26789525f32,
        0.26474094f32,
        0.26158381f32,
        0.25842386f32,
        0.25526115f32,
        0.25209570f32,
        0.24892756f32,
        0.24575676f32,
        0.24258332f32,
        0.23940729f32,
        0.23622867f32,
        0.23304754f32,
        0.22986393f32,
        0.22667783f32,
        0.22348931f32,
        0.22029841f32,
        0.21710514f32,
        0.21390954f32,
        0.21071166f32,
        0.20751151f32,
        0.20430915f32,
        0.20110460f32,
        0.19789790f32,
        0.19468907f32,
        0.19147816f32,
        0.18826519f32,
        0.18505022f32,
        0.18183327f32,
        0.17861435f32,
        0.17539354f32,
        0.17217083f32,
        0.16894630f32,
        0.16571994f32,
        0.16249183f32,
        0.15926196f32,
        0.15603039f32,
        0.15279715f32,
        0.14956227f32,
        0.14632578f32,
        0.14308774f32,
        0.13984816f32,
        0.13660708f32,
        0.13336454f32,
        0.13012058f32,
        0.12687522f32,
        0.12362850f32,
        0.12038045f32,
        0.11713112f32,
        0.11388054f32,
        0.11062872f32,
        0.10737573f32,
        0.10412160f32,
        0.10086634f32,
        0.097609997f32,
        0.094352618f32,
        0.091094226f32,
        0.087834857f32,
        0.084574550f32,
        0.081313334f32,
        0.078051247f32,
        0.074788325f32,
        0.071524605f32,
        0.068260118f32,
        0.064994894f32,
        0.061728980f32,
        0.058462404f32,
        0.055195201f32,
        0.051927410f32,
        0.048659060f32,
        0.045390189f32,
        0.042120833f32,
        0.038851023f32,
        0.035580799f32,
        0.032310195f32,
        0.029039243f32,
        0.025767982f32,
        0.022496443f32,
        0.019224664f32,
        0.015952680f32,
        0.012680525f32,
        0.0094082337f32,
        0.0061358409f32,
        0.0028633832f32,
        -0.00040910527f32,
        -0.0036815894f32,
        -0.0069540343f32,
        -0.010226404f32,
        -0.013498665f32,
        -0.016770782f32,
        -0.020042717f32,
        -0.023314439f32,
        -0.026585912f32,
        -0.029857099f32,
        -0.033127967f32,
        -0.036398482f32,
        -0.039668605f32,
        -0.042938303f32,
        -0.046207540f32,
        -0.049476285f32,
        -0.052744497f32,
        -0.056012146f32,
        -0.059279196f32,
        -0.062545612f32,
        -0.065811358f32,
        -0.069076397f32,
        -0.072340697f32,
        -0.075604223f32,
        -0.078866936f32,
        -0.082128808f32,
        -0.085389800f32,
        -0.088649876f32,
        -0.091909006f32,
        -0.095167145f32,
        -0.098424271f32,
        -0.10168034f32,
        -0.10493532f32,
        -0.10818918f32,
        -0.11144188f32,
        -0.11469338f32,
        -0.11794366f32,
        -0.12119267f32,
        -0.12444039f32,
        -0.12768677f32,
        -0.13093179f32,
        -0.13417540f32,
        -0.13741758f32,
        -0.14065829f32,
        -0.14389749f32,
        -0.14713514f32,
        -0.15037122f32,
        -0.15360570f32,
        -0.15683852f32,
        -0.16006967f32,
        -0.16329910f32,
        -0.16652679f32,
        -0.16975269f32,
        -0.17297678f32,
        -0.17619900f32,
        -0.17941935f32,
        -0.18263777f32,
        -0.18585424f32,
        -0.18906870f32,
        -0.19228116f32,
        -0.19549155f32,
        -0.19869985f32,
        -0.20190603f32,
        -0.20511003f32,
        -0.20831184f32,
        -0.21151142f32,
        -0.21470875f32,
        -0.21790376f32,
        -0.22109644f32,
        -0.22428675f32,
        -0.22747467f32,
        -0.23066014f32,
        -0.23384315f32,
        -0.23702365f32,
        -0.24020162f32,
        -0.24337701f32,
        -0.24654980f32,
        -0.24971995f32,
        -0.25288740f32,
        -0.25605217f32,
        -0.25921419f32,
        -0.26237345f32,
        -0.26552987f32,
        -0.26868346f32,
        -0.27183419f32,
        -0.27498198f32,
        -0.27812684f32,
        -0.28126872f32,
        -0.28440759f32,
        -0.28754342f32,
        -0.29067615f32,
        -0.29380578f32,
        -0.29693225f32,
        -0.30005556f32,
        -0.30317566f32,
        -0.30629250f32,
        -0.30940607f32,
        -0.31251630f32,
        -0.31562322f32,
        -0.31872672f32,
        -0.32182685f32,
        -0.32492352f32,
        -0.32801670f32,
        -0.33110636f32,
        -0.33419248f32,
        -0.33727503f32,
        -0.34035397f32,
        -0.34342924f32,
        -0.34650084f32,
        -0.34956875f32,
        -0.35263291f32,
        -0.35569328f32,
        -0.35874987f32,
        -0.36180258f32,
        -0.36485144f32,
        -0.36789638f32,
        -0.37093741f32,
        -0.37397444f32,
        -0.37700745f32,
        -0.38003644f32,
        -0.38306138f32,
        -0.38608220f32,
        -0.38909888f32,
        -0.39211139f32,
        -0.39511973f32,
        -0.39812380f32,
        -0.40112361f32,
        -0.40411916f32,
        -0.40711036f32,
        -0.41009718f32,
        -0.41307965f32,
        -0.41605768f32,
        -0.41903123f32,
        -0.42200032f32,
        -0.42496487f32,
        -0.42792490f32,
        -0.43088034f32,
        -0.43383113f32,
        -0.43677729f32,
        -0.43971881f32,
        -0.44265559f32,
        -0.44558764f32,
        -0.44851488f32,
        -0.45143735f32,
        -0.45435500f32,
        -0.45726776f32,
        -0.46017563f32,
        -0.46307856f32,
        -0.46597654f32,
        -0.46886954f32,
        -0.47175750f32,
        -0.47464043f32,
        -0.47751826f32,
        -0.48039100f32,
        -0.48325855f32,
        -0.48612097f32,
        -0.48897815f32,
        -0.49183011f32,
        -0.49467680f32,
        -0.49751821f32,
        -0.50035429f32,
        -0.50318497f32,
        -0.50601029f32,
        -0.50883019f32,
        -0.51164466f32,
        -0.51445359f32,
        -0.51725709f32,
        -0.52005500f32,
        -0.52284735f32,
        -0.52563411f32,
        -0.52841520f32,
        -0.53119069f32,
        -0.53396046f32,
        -0.53672451f32,
        -0.53948283f32,
        -0.54223537f32,
        -0.54498214f32,
        -0.54772300f32,
        -0.55045801f32,
        -0.55318713f32,
        -0.55591035f32,
        -0.55862761f32,
        -0.56133890f32,
        -0.56404412f32,
        -0.56674337f32,
        -0.56943649f32,
        -0.57212353f32,
        -0.57480448f32,
        -0.57747924f32,
        -0.58014780f32,
        -0.58281022f32,
        -0.58546633f32,
        -0.58811617f32,
        -0.59075975f32,
        -0.59339696f32,
        -0.59602785f32,
        -0.59865236f32,
        -0.60127044f32,
        -0.60388207f32,
        -0.60648727f32,
        -0.60908598f32,
        -0.61167812f32,
        -0.61426371f32,
        -0.61684275f32,
        -0.61941516f32,
        -0.62198097f32,
        -0.62454009f32,
        -0.62709254f32,
        -0.62963831f32,
        -0.63217729f32,
        -0.63470948f32,
        -0.63723493f32,
        -0.63975352f32,
        -0.64226526f32,
        -0.64477009f32,
        -0.64726806f32,
        -0.64975911f32,
        -0.65224314f32,
        -0.65472025f32,
        -0.65719032f32,
        -0.65965337f32,
        -0.66210932f32,
        -0.66455823f32,
        -0.66700000f32,
        -0.66943461f32,
        -0.67186207f32,
        -0.67428231f32,
        -0.67669535f32,
        -0.67910111f32,
        -0.68149966f32,
        -0.68389088f32,
        -0.68627477f32,
        -0.68865126f32,
        -0.69102043f32,
        -0.69338220f32,
        -0.69573659f32,
        -0.69808346f32,
        -0.70042288f32,
        -0.70275480f32,
        -0.70507920f32,
        -0.70739603f32,
        -0.70970529f32,
        -0.71200693f32,
        -0.71430099f32,
        -0.71658736f32,
        -0.71886611f32,
        -0.72113711f32,
        -0.72340041f32,
        -0.72565591f32,
        -0.72790372f32,
        -0.73014367f32,
        -0.73237586f32,
        -0.73460019f32,
        -0.73681659f32,
        -0.73902518f32,
        -0.74122584f32,
        -0.74341851f32,
        -0.74560326f32,
        -0.74778003f32,
        -0.74994880f32,
        -0.75210953f32,
        -0.75426215f32,
        -0.75640678f32,
        -0.75854325f32,
        -0.76067162f32,
        -0.76279181f32,
        -0.76490390f32,
        -0.76700771f32,
        -0.76910341f32,
        -0.77119076f32,
        -0.77326995f32,
        -0.77534080f32,
        -0.77740335f32,
        -0.77945763f32,
        -0.78150350f32,
        -0.78354102f32,
        -0.78557014f32,
        -0.78759086f32,
        -0.78960317f32,
        -0.79160696f32,
        -0.79360235f32,
        -0.79558921f32,
        -0.79756755f32,
        -0.79953730f32,
        -0.80149853f32,
        -0.80345118f32,
        -0.80539525f32,
        -0.80733067f32,
        -0.80925739f32,
        -0.81117553f32,
        -0.81308490f32,
        -0.81498563f32,
        -0.81687760f32,
        -0.81876087f32,
        -0.82063532f32,
        -0.82250100f32,
        -0.82435787f32,
        -0.82620591f32,
        -0.82804507f32,
        -0.82987541f32,
        -0.83169687f32,
        -0.83350939f32,
        -0.83531296f32,
        -0.83710766f32,
        -0.83889335f32,
        -0.84067005f32,
        -0.84243774f32,
        -0.84419644f32,
        -0.84594607f32,
        -0.84768665f32,
        -0.84941816f32,
        -0.85114056f32,
        -0.85285389f32,
        -0.85455805f32,
        -0.85625303f32,
        -0.85793889f32,
        -0.85961550f32,
        -0.86128294f32,
        -0.86294121f32,
        -0.86459017f32,
        -0.86622989f32,
        -0.86786032f32,
        -0.86948150f32,
        -0.87109333f32,
        -0.87269586f32,
        -0.87428904f32,
        -0.87587279f32,
        -0.87744725f32,
        -0.87901229f32,
        -0.88056785f32,
        -0.88211405f32,
        -0.88365078f32,
        -0.88517809f32,
        -0.88669586f32,
        -0.88820416f32,
        -0.88970292f32,
        -0.89119220f32,
        -0.89267188f32,
        -0.89414203f32,
        -0.89560264f32,
        -0.89705360f32,
        -0.89849502f32,
        -0.89992678f32,
        -0.90134889f32,
        -0.90276134f32,
        -0.90416414f32,
        -0.90555727f32,
        -0.90694070f32,
        -0.90831441f32,
        -0.90967834f32,
        -0.91103262f32,
        -0.91237706f32,
        -0.91371179f32,
        -0.91503674f32,
        -0.91635185f32,
        -0.91765714f32,
        -0.91895264f32,
        -0.92023826f32,
        -0.92151409f32,
        -0.92277998f32,
        -0.92403603f32,
        -0.92528218f32,
        -0.92651838f32,
        -0.92774469f32,
        -0.92896110f32,
        -0.93016750f32,
        -0.93136400f32,
        -0.93255049f32,
        -0.93372697f32,
        -0.93489349f32,
        -0.93604994f32,
        -0.93719643f32,
        -0.93833286f32,
        -0.93945926f32,
        -0.94057560f32,
        -0.94168180f32,
        -0.94277799f32,
        -0.94386405f32,
        -0.94494003f32,
        -0.94600588f32,
        -0.94706154f32,
        -0.94810712f32,
        -0.94914252f32,
        -0.95016778f32,
        -0.95118284f32,
        -0.95218778f32,
        -0.95318246f32,
        -0.95416695f32,
        -0.95514119f32,
        -0.95610523f32,
        -0.95705903f32,
        -0.95800257f32,
        -0.95893586f32,
        -0.95985889f32,
        -0.96077162f32,
        -0.96167403f32,
        -0.96256620f32,
        -0.96344805f32,
        -0.96431959f32,
        -0.96518075f32,
        -0.96603161f32,
        -0.96687216f32,
        -0.96770233f32,
        -0.96852213f32,
        -0.96933156f32,
        -0.97013056f32,
        -0.97091925f32,
        -0.97169751f32,
        -0.97246534f32,
        -0.97322279f32,
        -0.97396982f32,
        -0.97470641f32,
        -0.97543252f32,
        -0.97614825f32,
        -0.97685349f32,
        -0.97754824f32,
        -0.97823256f32,
        -0.97890645f32,
        -0.97956979f32,
        -0.98022264f32,
        -0.98086500f32,
        -0.98149687f32,
        -0.98211825f32,
        -0.98272908f32,
        -0.98332942f32,
        -0.98391914f32,
        -0.98449844f32,
        -0.98506713f32,
        -0.98562527f32,
        -0.98617285f32,
        -0.98670989f32,
        -0.98723638f32,
        -0.98775226f32,
        -0.98825759f32,
        -0.98875231f32,
        -0.98923647f32,
        -0.98971003f32,
        -0.99017298f32,
        -0.99062532f32,
        -0.99106705f32,
        -0.99149817f32,
        -0.99191868f32,
        -0.99232858f32,
        -0.99272782f32,
        -0.99311644f32,
        -0.99349445f32,
        -0.99386179f32,
        -0.99421853f32,
        -0.99456459f32,
        -0.99489999f32,
        -0.99522477f32,
        -0.99553883f32,
        -0.99584228f32,
        -0.99613506f32,
        -0.99641716f32,
        -0.99668860f32,
        -0.99694937f32,
        -0.99719942f32,
        -0.99743885f32,
        -0.99766755f32,
        -0.99788558f32,
        -0.99809295f32,
        -0.99828959f32,
        -0.99847561f32,
        -0.99865085f32,
        -0.99881548f32,
        -0.99896932f32,
        -0.99911255f32,
        -0.99924499f32,
        -0.99936682f32,
        -0.99947786f32,
        -0.99957830f32,
        -0.99966794f32,
        -0.99974692f32,
        -0.99981517f32,
        -0.99987274f32,
        -0.99991959f32,
        -0.99995571f32,
        -0.99998116f32,
        -0.99999589f32,
        0.99999964f32,
        0.99997288f32,
        0.99990326f32,
        0.99979085f32,
        0.99963558f32,
        0.99943751f32,
        0.99919659f32,
        0.99891287f32,
        0.99858636f32,
        0.99821711f32,
        0.99780506f32,
        0.99735034f32,
        0.99685282f32,
        0.99631262f32,
        0.99572974f32,
        0.99510419f32,
        0.99443603f32,
        0.99372530f32,
        0.99297196f32,
        0.99217612f32,
        0.99133772f32,
        0.99045694f32,
        0.98953366f32,
        0.98856801f32,
        0.98756003f32,
        0.98650974f32,
        0.98541719f32,
        0.98428243f32,
        0.98310548f32,
        0.98188645f32,
        0.98062533f32,
        0.97932225f32,
        0.97797716f32,
        0.97659022f32,
        0.97516143f32,
        0.97369087f32,
        0.97217858f32,
        0.97062469f32,
        0.96902919f32,
        0.96739221f32,
        0.96571374f32,
        0.96399397f32,
        0.96223283f32,
        0.96043050f32,
        0.95858705f32,
        0.95670253f32,
        0.95477700f32,
        0.95281059f32,
        0.95080340f32,
        0.94875544f32,
        0.94666684f32,
        0.94453770f32,
        0.94236809f32,
        0.94015813f32,
        0.93790787f32,
        0.93561745f32,
        0.93328691f32,
        0.93091643f32,
        0.92850608f32,
        0.92605597f32,
        0.92356616f32,
        0.92103678f32,
        0.91846794f32,
        0.91585976f32,
        0.91321236f32,
        0.91052586f32,
        0.90780038f32,
        0.90503591f32,
        0.90223277f32,
        0.89939094f32,
        0.89651060f32,
        0.89359182f32,
        0.89063478f32,
        0.88763964f32,
        0.88460642f32,
        0.88153529f32,
        0.87842643f32,
        0.87527996f32,
        0.87209594f32,
        0.86887461f32,
        0.86561602f32,
        0.86232042f32,
        0.85898781f32,
        0.85561842f32,
        0.85221243f32,
        0.84876984f32,
        0.84529096f32,
        0.84177583f32,
        0.83822471f32,
        0.83463764f32,
        0.83101481f32,
        0.82735640f32,
        0.82366252f32,
        0.81993335f32,
        0.81616908f32,
        0.81236988f32,
        0.80853581f32,
        0.80466717f32,
        0.80076402f32,
        0.79682660f32,
        0.79285502f32,
        0.78884947f32,
        0.78481019f32,
        0.78073722f32,
        0.77663082f32,
        0.77249116f32,
        0.76831841f32,
        0.76411277f32,
        0.75987434f32,
        0.75560343f32,
        0.75130010f32,
        0.74696463f32,
        0.74259710f32,
        0.73819780f32,
        0.73376691f32,
        0.72930455f32,
        0.72481096f32,
        0.72028631f32,
        0.71573079f32,
        0.71114463f32,
        0.70652801f32,
        0.70188117f32,
        0.69720417f32,
        0.69249737f32,
        0.68776089f32,
        0.68299496f32,
        0.67819971f32,
        0.67337549f32,
        0.66852236f32,
        0.66364062f32,
        0.65873051f32,
        0.65379208f32,
        0.64882571f32,
        0.64383155f32,
        0.63880974f32,
        0.63376063f32,
        0.62868434f32,
        0.62358117f32,
        0.61845124f32,
        0.61329484f32,
        0.60811216f32,
        0.60290343f32,
        0.59766883f32,
        0.59240872f32,
        0.58712316f32,
        0.58181250f32,
        0.57647687f32,
        0.57111657f32,
        0.56573176f32,
        0.56032276f32,
        0.55488980f32,
        0.54943299f32,
        0.54395270f32,
        0.53844911f32,
        0.53292239f32,
        0.52737290f32,
        0.52180082f32,
        0.51620632f32,
        0.51058978f32,
        0.50495136f32,
        0.49929130f32,
        0.49360985f32,
        0.48790723f32,
        0.48218375f32,
        0.47643960f32,
        0.47067502f32,
        0.46489030f32,
        0.45908567f32,
        0.45326138f32,
        0.44741765f32,
        0.44155475f32,
        0.43567297f32,
        0.42977250f32,
        0.42385364f32,
        0.41791660f32,
        0.41196167f32,
        0.40598908f32,
        0.39999911f32,
        0.39399201f32,
        0.38796803f32,
        0.38192743f32,
        0.37587047f32,
        0.36979741f32,
        0.36370850f32,
        0.35760403f32,
        0.35148421f32,
        0.34534934f32,
        0.33919969f32,
        0.33303553f32,
        0.32685706f32,
        0.32066461f32,
        0.31445843f32,
        0.30823877f32,
        0.30200592f32,
        0.29576012f32,
        0.28950164f32,
        0.28323078f32,
        0.27694780f32,
        0.27065292f32,
        0.26434645f32,
        0.25802869f32,
        0.25169984f32,
        0.24536023f32,
        0.23901010f32,
        0.23264973f32,
        0.22627939f32,
        0.21989937f32,
        0.21350993f32,
        0.20711134f32,
        0.20070387f32,
        0.19428782f32,
        0.18786344f32,
        0.18143101f32,
        0.17499080f32,
        0.16854310f32,
        0.16208819f32,
        0.15562633f32,
        0.14915779f32,
        0.14268288f32,
        0.13620184f32,
        0.12971498f32,
        0.12322257f32,
        0.11672486f32,
        0.11022217f32,
        0.10371475f32,
        0.097202882f32,
        0.090686858f32,
        0.084166944f32,
        0.077643424f32,
        0.071116582f32,
        0.064586692f32,
        0.058054037f32,
        0.051518895f32,
        0.044981543f32,
        0.038442269f32,
        0.031901345f32,
        0.025359053f32,
        0.018815678f32,
        0.012271495f32,
        0.0057267868f32,
        -0.00081816671f32,
        -0.0073630852f32,
        -0.013907688f32,
        -0.020451695f32,
        -0.026994826f32,
        -0.033536803f32,
        -0.040077340f32,
        -0.046616159f32,
        -0.053152986f32,
        -0.059687532f32,
        -0.066219524f32,
        -0.072748676f32,
        -0.079274714f32,
        -0.085797355f32,
        -0.092316322f32,
        -0.098831341f32,
        -0.10534211f32,
        -0.11184838f32,
        -0.11834986f32,
        -0.12484626f32,
        -0.13133731f32,
        -0.13782275f32,
        -0.14430228f32,
        -0.15077563f32,
        -0.15724251f32,
        -0.16370267f32,
        -0.17015581f32,
        -0.17660165f32,
        -0.18303993f32,
        -0.18947038f32,
        -0.19589271f32,
        -0.20230664f32,
        -0.20871192f32,
        -0.21510825f32,
        -0.22149536f32,
        -0.22787298f32,
        -0.23424086f32,
        -0.24059868f32,
        -0.24694622f32,
        -0.25328314f32,
        -0.25960925f32,
        -0.26592422f32,
        -0.27222782f32,
        -0.27851975f32,
        -0.28479972f32,
        -0.29106751f32,
        -0.29732284f32,
        -0.30356544f32,
        -0.30979502f32,
        -0.31601134f32,
        -0.32221413f32,
        -0.32840309f32,
        -0.33457801f32,
        -0.34073856f32,
        -0.34688455f32,
        -0.35301566f32,
        -0.35913166f32,
        -0.36523229f32,
        -0.37131724f32,
        -0.37738630f32,
        -0.38343921f32,
        -0.38947567f32,
        -0.39549544f32,
        -0.40149832f32,
        -0.40748394f32,
        -0.41345215f32,
        -0.41940263f32,
        -0.42533514f32,
        -0.43124944f32,
        -0.43714526f32,
        -0.44302234f32,
        -0.44888046f32,
        -0.45471936f32,
        -0.46053877f32,
        -0.46633846f32,
        -0.47211814f32,
        -0.47787762f32,
        -0.48361665f32,
        -0.48933494f32,
        -0.49503228f32,
        -0.50070840f32,
        -0.50636309f32,
        -0.51199609f32,
        -0.51760709f32,
        -0.52319598f32,
        -0.52876246f32,
        -0.53430629f32,
        -0.53982723f32,
        -0.54532504f32,
        -0.55079949f32,
        -0.55625033f32,
        -0.56167740f32,
        -0.56708032f32,
        -0.57245898f32,
        -0.57781315f32,
        -0.58314258f32,
        -0.58844697f32,
        -0.59372622f32,
        -0.59897995f32,
        -0.60420811f32,
        -0.60941035f32,
        -0.61458647f32,
        -0.61973625f32,
        -0.62485951f32,
        -0.62995601f32,
        -0.63502556f32,
        -0.64006782f32,
        -0.64508271f32,
        -0.65007001f32,
        -0.65502942f32,
        -0.65996075f32,
        -0.66486382f32,
        -0.66973841f32,
        -0.67458433f32,
        -0.67940134f32,
        -0.68418926f32,
        -0.68894786f32,
        -0.69367695f32,
        -0.69837630f32,
        -0.70304573f32,
        -0.70768511f32,
        -0.71229410f32,
        -0.71687263f32,
        -0.72142041f32,
        -0.72593731f32,
        -0.73042315f32,
        -0.73487765f32,
        -0.73930067f32,
        -0.74369204f32,
        -0.74805158f32,
        -0.75237900f32,
        -0.75667429f32,
        -0.76093709f32,
        -0.76516730f32,
        -0.76936477f32,
        -0.77352923f32,
        -0.77766061f32,
        -0.78175867f32,
        -0.78582323f32,
        -0.78985411f32,
        -0.79385114f32,
        -0.79781419f32,
        -0.80174309f32,
        -0.80563760f32,
        -0.80949765f32,
        -0.81332302f32,
        -0.81711352f32,
        -0.82086903f32,
        -0.82458937f32,
        -0.82827437f32,
        -0.83192390f32,
        -0.83553779f32,
        -0.83911592f32,
        -0.84265804f32,
        -0.84616417f32,
        -0.84963393f32,
        -0.85306740f32,
        -0.85646427f32,
        -0.85982448f32,
        -0.86314780f32,
        -0.86643422f32,
        -0.86968350f32,
        -0.87289548f32,
        -0.87607014f32,
        -0.87920725f32,
        -0.88230664f32,
        -0.88536829f32,
        -0.88839203f32,
        -0.89137769f32,
        -0.89432514f32,
        -0.89723432f32,
        -0.90010506f32,
        -0.90293723f32,
        -0.90573072f32,
        -0.90848541f32,
        -0.91120118f32,
        -0.91387796f32,
        -0.91651553f32,
        -0.91911387f32,
        -0.92167282f32,
        -0.92419231f32,
        -0.92667222f32,
        -0.92911243f32,
        -0.93151283f32,
        -0.93387336f32,
        -0.93619382f32,
        -0.93847424f32,
        -0.94071442f32,
        -0.94291431f32,
        -0.94507378f32,
        -0.94719279f32,
        -0.94927126f32,
        -0.95130903f32,
        -0.95330608f32,
        -0.95526224f32,
        -0.95717752f32,
        -0.95905179f32,
        -0.96088499f32,
        -0.96267700f32,
        -0.96442777f32,
        -0.96613729f32,
        -0.96780539f32,
        -0.96943200f32,
        -0.97101706f32,
        -0.97256058f32,
        -0.97406244f32,
        -0.97552258f32,
        -0.97694093f32,
        -0.97831738f32,
        -0.97965199f32,
        -0.98094457f32,
        -0.98219514f32,
        -0.98340368f32,
        -0.98457009f32,
        -0.98569429f32,
        -0.98677629f32,
        -0.98781598f32,
        -0.98881340f32,
        -0.98976845f32,
        -0.99068111f32,
        -0.99155134f32,
        -0.99237907f32,
        -0.99316430f32,
        -0.99390697f32,
        -0.99460709f32,
        -0.99526459f32,
        -0.99587947f32,
        -0.99645168f32,
        -0.99698120f32,
        -0.99746799f32,
        -0.99791211f32,
        -0.99831343f32,
        -0.99867201f32,
        -0.99898779f32,
        -0.99926084f32,
        -0.99949104f32,
        -0.99967843f32,
        -0.99982297f32,
        -0.99992472f32,
        -0.99998361f32,
        0.99999869f32,
        0.99989158f32,
        0.99961317f32,
        0.99916345f32,
        0.99854255f32,
        0.99775058f32,
        0.99678761f32,
        0.99565387f32,
        0.99434954f32,
        0.99287480f32,
        0.99122995f32,
        0.98941529f32,
        0.98743105f32,
        0.98527765f32,
        0.98295540f32,
        0.98046476f32,
        0.97780609f32,
        0.97497988f32,
        0.97198665f32,
        0.96882683f32,
        0.96550101f32,
        0.96200979f32,
        0.95835376f32,
        0.95453346f32,
        0.95054960f32,
        0.94640291f32,
        0.94209403f32,
        0.93762374f32,
        0.93299282f32,
        0.92820197f32,
        0.92325211f32,
        0.91814411f32,
        0.91287869f32,
        0.90745693f32,
        0.90187967f32,
        0.89614785f32,
        0.89026248f32,
        0.88422459f32,
        0.87803519f32,
        0.87169534f32,
        0.86520612f32,
        0.85856867f32,
        0.85178405f32,
        0.84485358f32,
        0.83777827f32,
        0.83055943f32,
        0.82319832f32,
        0.81569612f32,
        0.80805415f32,
        0.80027372f32,
        0.79235619f32,
        0.78430289f32,
        0.77611518f32,
        0.76779449f32,
        0.75934225f32,
        0.75075996f32,
        0.74204898f32,
        0.73321080f32,
        0.72424710f32,
        0.71515924f32,
        0.70594883f32,
        0.69661748f32,
        0.68716675f32,
        0.67759830f32,
        0.66791373f32,
        0.65811473f32,
        0.64820296f32,
        0.63818014f32,
        0.62804794f32,
        0.61780810f32,
        0.60746247f32,
        0.59701276f32,
        0.58646071f32,
        0.57580817f32,
        0.56505698f32,
        0.55420899f32,
        0.54326600f32,
        0.53222996f32,
        0.52110273f32,
        0.50988621f32,
        0.49858227f32,
        0.48719296f32,
        0.47572014f32,
        0.46416581f32,
        0.45253196f32,
        0.44082057f32,
        0.42903364f32,
        0.41717321f32,
        0.40524128f32,
        0.39323992f32,
        0.38117120f32,
        0.36903715f32,
        0.35683987f32,
        0.34458145f32,
        0.33226398f32,
        0.31988961f32,
        0.30746040f32,
        0.29497850f32,
        0.28244606f32,
        0.26986524f32,
        0.25723818f32,
        0.24456702f32,
        0.23185398f32,
        0.21910121f32,
        0.20631088f32,
        0.19348522f32,
        0.18062639f32,
        0.16773662f32,
        0.15481812f32,
        0.14187308f32,
        0.12890373f32,
        0.11591230f32,
        0.10290100f32,
        0.089872077f32,
        0.076827750f32,
        0.063770257f32,
        0.050701842f32,
        0.037624735f32,
        0.024541186f32,
        0.011453429f32,
        -0.0016362892f32,
        -0.014725727f32,
        -0.027812643f32,
        -0.040894791f32,
        -0.053969935f32,
        -0.067035832f32,
        -0.080090240f32,
        -0.093130924f32,
        -0.10615565f32,
        -0.11916219f32,
        -0.13214831f32,
        -0.14511178f32,
        -0.15805040f32,
        -0.17096193f32,
        -0.18384418f32,
        -0.19669491f32,
        -0.20951195f32,
        -0.22229309f32,
        -0.23503613f32,
        -0.24773891f32,
        -0.26039925f32,
        -0.27301496f32,
        -0.28558388f32,
        -0.29810387f32,
        -0.31057280f32,
        -0.32298848f32,
        -0.33534884f32,
        -0.34765175f32,
        -0.35989508f32,
        -0.37207675f32,
        -0.38419467f32,
        -0.39624676f32,
        -0.40823093f32,
        -0.42014518f32,
        -0.43198743f32,
        -0.44375566f32,
        -0.45544785f32,
        -0.46706200f32,
        -0.47859612f32,
        -0.49004826f32,
        -0.50141639f32,
        -0.51269865f32,
        -0.52389306f32,
        -0.53499764f32,
        -0.54601061f32,
        -0.55693001f32,
        -0.56775403f32,
        -0.57848072f32,
        -0.58910829f32,
        -0.59963489f32,
        -0.61005878f32,
        -0.62037814f32,
        -0.63059121f32,
        -0.64069623f32,
        -0.65069145f32,
        -0.66057515f32,
        -0.67034572f32,
        -0.68000144f32,
        -0.68954057f32,
        -0.69896162f32,
        -0.70826286f32,
        -0.71744281f32,
        -0.72649974f32,
        -0.73543227f32,
        -0.74423873f32,
        -0.75291771f32,
        -0.76146764f32,
        -0.76988715f32,
        -0.77817470f32,
        -0.78632891f32,
        -0.79434842f32,
        -0.80223179f32,
        -0.80997771f32,
        -0.81758487f32,
        -0.82505190f32,
        -0.83237761f32,
        -0.83956063f32,
        -0.84659988f32,
        -0.85349399f32,
        -0.86024189f32,
        -0.86684239f32,
        -0.87329435f32,
        -0.87959671f32,
        -0.88574833f32,
        -0.89174819f32,
        -0.89759529f32,
        -0.90328854f32,
        -0.90882701f32,
        -0.91420978f32,
        -0.91943592f32,
        -0.92450452f32,
        -0.92941469f32,
        -0.93416560f32,
        -0.93875647f32,
        -0.94318646f32,
        -0.94745487f32,
        -0.95156091f32,
        -0.95550388f32,
        -0.95928317f32,
        -0.96289814f32,
        -0.96634805f32,
        -0.96963239f32,
        -0.97275060f32,
        -0.97570217f32,
        -0.97848648f32,
        -0.98110318f32,
        -0.98355180f32,
        -0.98583186f32,
        -0.98794299f32,
        -0.98988485f32,
        -0.99165714f32,
        -0.99325943f32,
        -0.99469161f32,
        -0.99595332f32,
        -0.99704438f32,
        -0.99796462f32,
        -0.99871385f32,
        -0.99929196f32,
        -0.99969882f32,
        -0.99993443f32,
        0.99999464f32,
        0.99956632f32,
        0.99845290f32,
        0.99665523f32,
        0.99417448f32,
        0.99101239f32,
        0.98717111f32,
        0.98265326f32,
        0.97746199f32,
        0.97160077f32,
        0.96507365f32,
        0.95788515f32,
        0.95004016f32,
        0.94154406f32,
        0.93240267f32,
        0.92262226f32,
        0.91220951f32,
        0.90117162f32,
        0.88951606f32,
        0.87725091f32,
        0.86438453f32,
        0.85092574f32,
        0.83688372f32,
        0.82226819f32,
        0.80708915f32,
        0.79135692f32,
        0.77508235f32,
        0.75827658f32,
        0.74095112f32,
        0.72311783f32,
        0.70478898f32,
        0.68597710f32,
        0.66669506f32,
        0.64695615f32,
        0.62677377f32,
        0.60616189f32,
        0.58513457f32,
        0.56370622f32,
        0.54189157f32,
        0.51970547f32,
        0.49716324f32,
        0.47428027f32,
        0.45107225f32,
        0.42755505f32,
        0.40374488f32,
        0.37965798f32,
        0.35531086f32,
        0.33072025f32,
        0.30590299f32,
        0.28087607f32,
        0.25565663f32,
        0.23026201f32,
        0.20470956f32,
        0.17901683f32,
        0.15320139f32,
        0.12728097f32,
        0.10127331f32,
        0.075196236f32,
        0.049067631f32,
        0.022905400f32,
        -0.0032725304f32,
        -0.029448219f32,
        -0.055603724f32,
        -0.081721120f32,
        -0.10778251f32,
        -0.13377003f32,
        -0.15966587f32,
        -0.18545228f32,
        -0.21111161f32,
        -0.23662624f32,
        -0.26197869f32,
        -0.28715160f32,
        -0.31212771f32,
        -0.33688989f32,
        -0.36142120f32,
        -0.38570482f32,
        -0.40972409f32,
        -0.43346253f32,
        -0.45690393f32,
        -0.48003218f32,
        -0.50283146f32,
        -0.52528608f32,
        -0.54738069f32,
        -0.56910020f32,
        -0.59042966f32,
        -0.61135447f32,
        -0.63186026f32,
        -0.65193301f32,
        -0.67155898f32,
        -0.69072473f32,
        -0.70941705f32,
        -0.72762316f32,
        -0.74533063f32,
        -0.76252723f32,
        -0.77920127f32,
        -0.79534131f32,
        -0.81093621f32,
        -0.82597536f32,
        -0.84044844f32,
        -0.85434550f32,
        -0.86765707f32,
        -0.88037395f32,
        -0.89248747f32,
        -0.90398932f32,
        -0.91487163f32,
        -0.92512697f32,
        -0.93474823f32,
        -0.94372886f32,
        -0.95206273f32,
        -0.95974404f32,
        -0.96676767f32,
        -0.97312868f32,
        -0.97882277f32,
        -0.98384601f32,
        -0.98819500f32,
        -0.99186671f32,
        -0.99485862f32,
        -0.99716878f32,
        -0.99879545f32,
        -0.99973762f32,
    ];
    #[c2rust::src_loc = "866:23"]
    pub static mut mode48000_960_120: OpusCustomMode = unsafe {
        {
            let init = OpusCustomMode {
                Fs: 48000 as libc::c_int,
                overlap: 120 as libc::c_int,
                nbEBands: 21 as libc::c_int,
                effEBands: 21 as libc::c_int,
                preemph: [0.85000610f32, 0.0000000f32, 1.0000000f32, 1.0000000f32],
                eBands: eband5ms.as_ptr(),
                maxLM: 3 as libc::c_int,
                nbShortMdcts: 8 as libc::c_int,
                shortMdctSize: 120 as libc::c_int,
                nbAllocVectors: 11 as libc::c_int,
                allocVectors: band_allocation.as_ptr(),
                logN: logN400.as_ptr(),
                window: window120.as_ptr(),
                mdct: {
                    let init = mdct_lookup {
                        n: 1920 as libc::c_int,
                        maxshift: 3 as libc::c_int,
                        kfft: [
                            &fft_state48000_960_0 as *const kiss_fft_state,
                            &fft_state48000_960_1 as *const kiss_fft_state,
                            &fft_state48000_960_2 as *const kiss_fft_state,
                            &fft_state48000_960_3 as *const kiss_fft_state,
                        ],
                        trig: mdct_twiddles960.as_ptr(),
                    };
                    init
                },
                cache: {
                    let init = PulseCache {
                        size: 392 as libc::c_int,
                        index: cache_index50.as_ptr(),
                        bits: cache_bits50.as_ptr(),
                        caps: cache_caps50.as_ptr(),
                    };
                    init
                },
            };
            init
        }
    };
    #[c2rust::src_loc = "885:9"]
    pub const TOTAL_MODES: libc::c_int = 1 as libc::c_int;
    #[c2rust::src_loc = "886:31"]
    pub static mut static_mode_list: [*const OpusCustomMode; 1] =
        unsafe { [&mode48000_960_120 as *const OpusCustomMode] };

    use super::arch_h::opus_val16;
    use super::stddef_h::NULL;
    use super::{band_allocation, eband5ms};
    use crate::celt::kiss_fft::{arch_fft_state, kiss_fft_state, kiss_twiddle_cpx};
    use crate::celt::mdct::mdct_lookup;
    use crate::celt::modes::{OpusCustomMode, PulseCache};
}
pub use self::arch_h::opus_val16;
pub use self::opus_defines_h::{OPUS_BAD_ARG, OPUS_OK};
pub use self::static_modes_float_h::{
    cache_bits50, cache_caps50, cache_index50, fft_bitrev120, fft_bitrev240, fft_bitrev480,
    fft_bitrev60, fft_state48000_960_0, fft_state48000_960_1, fft_state48000_960_2,
    fft_state48000_960_3, fft_twiddles48000_960, logN400, mdct_twiddles960, mode48000_960_120,
    static_mode_list, window120, TOTAL_MODES,
};
pub use self::stddef_h::NULL;
use crate::celt::mdct::mdct_lookup;

#[c2rust::src_loc = "42:25"]
static mut eband5ms: [i16; 22] = [
    0 as libc::c_int as i16,
    1 as libc::c_int as i16,
    2 as libc::c_int as i16,
    3 as libc::c_int as i16,
    4 as libc::c_int as i16,
    5 as libc::c_int as i16,
    6 as libc::c_int as i16,
    7 as libc::c_int as i16,
    8 as libc::c_int as i16,
    10 as libc::c_int as i16,
    12 as libc::c_int as i16,
    14 as libc::c_int as i16,
    16 as libc::c_int as i16,
    20 as libc::c_int as i16,
    24 as libc::c_int as i16,
    28 as libc::c_int as i16,
    34 as libc::c_int as i16,
    40 as libc::c_int as i16,
    48 as libc::c_int as i16,
    60 as libc::c_int as i16,
    78 as libc::c_int as i16,
    100 as libc::c_int as i16,
];
#[c2rust::src_loc = "50:28"]
static mut band_allocation: [libc::c_uchar; 231] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    90 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    75 as libc::c_int as libc::c_uchar,
    69 as libc::c_int as libc::c_uchar,
    63 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    40 as libc::c_int as libc::c_uchar,
    34 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    18 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    90 as libc::c_int as libc::c_uchar,
    84 as libc::c_int as libc::c_uchar,
    78 as libc::c_int as libc::c_uchar,
    71 as libc::c_int as libc::c_uchar,
    65 as libc::c_int as libc::c_uchar,
    58 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    110 as libc::c_int as libc::c_uchar,
    103 as libc::c_int as libc::c_uchar,
    93 as libc::c_int as libc::c_uchar,
    86 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    75 as libc::c_int as libc::c_uchar,
    70 as libc::c_int as libc::c_uchar,
    65 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    53 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    40 as libc::c_int as libc::c_uchar,
    31 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    4 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    126 as libc::c_int as libc::c_uchar,
    119 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
    95 as libc::c_int as libc::c_uchar,
    89 as libc::c_int as libc::c_uchar,
    83 as libc::c_int as libc::c_uchar,
    78 as libc::c_int as libc::c_uchar,
    72 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    60 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    32 as libc::c_int as libc::c_uchar,
    25 as libc::c_int as libc::c_uchar,
    17 as libc::c_int as libc::c_uchar,
    12 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    134 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    120 as libc::c_int as libc::c_uchar,
    114 as libc::c_int as libc::c_uchar,
    103 as libc::c_int as libc::c_uchar,
    97 as libc::c_int as libc::c_uchar,
    91 as libc::c_int as libc::c_uchar,
    85 as libc::c_int as libc::c_uchar,
    78 as libc::c_int as libc::c_uchar,
    72 as libc::c_int as libc::c_uchar,
    66 as libc::c_int as libc::c_uchar,
    60 as libc::c_int as libc::c_uchar,
    54 as libc::c_int as libc::c_uchar,
    47 as libc::c_int as libc::c_uchar,
    41 as libc::c_int as libc::c_uchar,
    35 as libc::c_int as libc::c_uchar,
    29 as libc::c_int as libc::c_uchar,
    23 as libc::c_int as libc::c_uchar,
    16 as libc::c_int as libc::c_uchar,
    10 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    144 as libc::c_int as libc::c_uchar,
    137 as libc::c_int as libc::c_uchar,
    130 as libc::c_int as libc::c_uchar,
    124 as libc::c_int as libc::c_uchar,
    113 as libc::c_int as libc::c_uchar,
    107 as libc::c_int as libc::c_uchar,
    101 as libc::c_int as libc::c_uchar,
    95 as libc::c_int as libc::c_uchar,
    88 as libc::c_int as libc::c_uchar,
    82 as libc::c_int as libc::c_uchar,
    76 as libc::c_int as libc::c_uchar,
    70 as libc::c_int as libc::c_uchar,
    64 as libc::c_int as libc::c_uchar,
    57 as libc::c_int as libc::c_uchar,
    51 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    39 as libc::c_int as libc::c_uchar,
    33 as libc::c_int as libc::c_uchar,
    26 as libc::c_int as libc::c_uchar,
    15 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    152 as libc::c_int as libc::c_uchar,
    145 as libc::c_int as libc::c_uchar,
    138 as libc::c_int as libc::c_uchar,
    132 as libc::c_int as libc::c_uchar,
    123 as libc::c_int as libc::c_uchar,
    117 as libc::c_int as libc::c_uchar,
    111 as libc::c_int as libc::c_uchar,
    105 as libc::c_int as libc::c_uchar,
    98 as libc::c_int as libc::c_uchar,
    92 as libc::c_int as libc::c_uchar,
    86 as libc::c_int as libc::c_uchar,
    80 as libc::c_int as libc::c_uchar,
    74 as libc::c_int as libc::c_uchar,
    67 as libc::c_int as libc::c_uchar,
    61 as libc::c_int as libc::c_uchar,
    55 as libc::c_int as libc::c_uchar,
    49 as libc::c_int as libc::c_uchar,
    43 as libc::c_int as libc::c_uchar,
    36 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    162 as libc::c_int as libc::c_uchar,
    155 as libc::c_int as libc::c_uchar,
    148 as libc::c_int as libc::c_uchar,
    142 as libc::c_int as libc::c_uchar,
    133 as libc::c_int as libc::c_uchar,
    127 as libc::c_int as libc::c_uchar,
    121 as libc::c_int as libc::c_uchar,
    115 as libc::c_int as libc::c_uchar,
    108 as libc::c_int as libc::c_uchar,
    102 as libc::c_int as libc::c_uchar,
    96 as libc::c_int as libc::c_uchar,
    90 as libc::c_int as libc::c_uchar,
    84 as libc::c_int as libc::c_uchar,
    77 as libc::c_int as libc::c_uchar,
    71 as libc::c_int as libc::c_uchar,
    65 as libc::c_int as libc::c_uchar,
    59 as libc::c_int as libc::c_uchar,
    53 as libc::c_int as libc::c_uchar,
    46 as libc::c_int as libc::c_uchar,
    30 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    172 as libc::c_int as libc::c_uchar,
    165 as libc::c_int as libc::c_uchar,
    158 as libc::c_int as libc::c_uchar,
    152 as libc::c_int as libc::c_uchar,
    143 as libc::c_int as libc::c_uchar,
    137 as libc::c_int as libc::c_uchar,
    131 as libc::c_int as libc::c_uchar,
    125 as libc::c_int as libc::c_uchar,
    118 as libc::c_int as libc::c_uchar,
    112 as libc::c_int as libc::c_uchar,
    106 as libc::c_int as libc::c_uchar,
    100 as libc::c_int as libc::c_uchar,
    94 as libc::c_int as libc::c_uchar,
    87 as libc::c_int as libc::c_uchar,
    81 as libc::c_int as libc::c_uchar,
    75 as libc::c_int as libc::c_uchar,
    69 as libc::c_int as libc::c_uchar,
    63 as libc::c_int as libc::c_uchar,
    56 as libc::c_int as libc::c_uchar,
    45 as libc::c_int as libc::c_uchar,
    20 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    200 as libc::c_int as libc::c_uchar,
    198 as libc::c_int as libc::c_uchar,
    193 as libc::c_int as libc::c_uchar,
    188 as libc::c_int as libc::c_uchar,
    183 as libc::c_int as libc::c_uchar,
    178 as libc::c_int as libc::c_uchar,
    173 as libc::c_int as libc::c_uchar,
    168 as libc::c_int as libc::c_uchar,
    163 as libc::c_int as libc::c_uchar,
    158 as libc::c_int as libc::c_uchar,
    153 as libc::c_int as libc::c_uchar,
    148 as libc::c_int as libc::c_uchar,
    129 as libc::c_int as libc::c_uchar,
    104 as libc::c_int as libc::c_uchar,
];
#[c2rust::src_loc = "224:1"]
pub unsafe extern "C" fn opus_custom_mode_create(
    Fs: i32,
    frame_size: libc::c_int,
    error: *mut libc::c_int,
) -> *mut OpusCustomMode {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < TOTAL_MODES {
        let mut j: libc::c_int = 0;
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            if Fs == (*static_mode_list[i as usize]).Fs
                && frame_size << j
                    == (*static_mode_list[i as usize]).shortMdctSize
                        * (*static_mode_list[i as usize]).nbShortMdcts
            {
                if !error.is_null() {
                    *error = OPUS_OK;
                }
                return static_mode_list[i as usize] as *mut OpusCustomMode;
            }
            j += 1;
        }
        i += 1;
    }
    if !error.is_null() {
        *error = OPUS_BAD_ARG;
    }
    return NULL as *mut OpusCustomMode;
}
