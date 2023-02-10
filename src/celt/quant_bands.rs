use ::libc;
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/arch.h:33"]
pub mod arch_h {
    #[c2rust::src_loc = "179:1"]
    pub type opus_val16 = libc::c_float;
    #[c2rust::src_loc = "180:1"]
    pub type opus_val32 = libc::c_float;
    #[c2rust::src_loc = "185:1"]
    pub type celt_ener = libc::c_float;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/modes.h:33"]
pub mod modes_h {
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
    use super::arch_h::opus_val16;
    use crate::celt::mdct::mdct_lookup;
}

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:33"]
pub mod entcode_h {
    #[c2rust::src_loc = "45:1"]
    pub type ec_window = u32;
    #[derive(Copy, Clone)]
    #[repr(C)]
    #[c2rust::src_loc = "62:8"]
    pub struct ec_ctx {
        pub buf: *mut libc::c_uchar,
        pub storage: u32,
        pub end_offs: u32,
        pub end_window: ec_window,
        pub nend_bits: libc::c_int,
        pub nbits_total: libc::c_int,
        pub offs: u32,
        pub rng: u32,
        pub val: u32,
        pub ext: u32,
        pub rem: libc::c_int,
        pub error: libc::c_int,
    }
    #[c2rust::src_loc = "47:1"]
    pub type ec_enc = ec_ctx;
    #[c2rust::src_loc = "48:1"]
    pub type ec_dec = ec_ctx;
    #[inline]
    #[c2rust::src_loc = "93:1"]
    pub unsafe extern "C" fn ec_range_bytes(mut _this: *mut ec_ctx) -> u32 {
        return (*_this).offs;
    }
    #[inline]
    #[c2rust::src_loc = "97:1"]
    pub unsafe extern "C" fn ec_get_buffer(mut _this: *mut ec_ctx) -> *mut libc::c_uchar {
        return (*_this).buf;
    }
    #[inline]
    #[c2rust::src_loc = "111:1"]
    pub unsafe extern "C" fn ec_tell(mut _this: *mut ec_ctx) -> libc::c_int {
        return (*_this).nbits_total - (EC_CLZ0 - ((*_this).rng).leading_zeros() as i32);
    }

    use super::ecintrin_h::EC_CLZ0;
    extern "C" {
        #[c2rust::src_loc = "121:1"]
        pub fn ec_tell_frac(_this: *mut ec_ctx) -> u32;
    }
}
#[c2rust::header_src = "/usr/lib/clang/15.0.7/include/limits.h:33"]
pub mod limits_h {
    #[c2rust::src_loc = "63:9"]
    pub const CHAR_BIT: libc::c_int = __CHAR_BIT__;
    use super::internal::__CHAR_BIT__;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/ecintrin.h:33"]
pub mod ecintrin_h {
    #[c2rust::src_loc = "69:11"]
    pub const EC_CLZ0: libc::c_int =
        ::core::mem::size_of::<libc::c_uint>() as libc::c_ulong as libc::c_int * CHAR_BIT;
    use super::limits_h::CHAR_BIT;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entenc.h:33"]
pub mod entenc_h {
    use super::entcode_h::ec_enc;
    extern "C" {
        #[c2rust::src_loc = "56:1"]
        pub fn ec_enc_bit_logp(_this: *mut ec_enc, _val: libc::c_int, _logp: libc::c_uint);
        #[c2rust::src_loc = "65:1"]
        pub fn ec_enc_icdf(
            _this: *mut ec_enc,
            _s: libc::c_int,
            _icdf: *const libc::c_uchar,
            _ftb: libc::c_uint,
        );
        #[c2rust::src_loc = "77:1"]
        pub fn ec_enc_bits(_this: *mut ec_enc, _fl: u32, _ftb: libc::c_uint);
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entdec.h:33"]
pub mod entdec_h {
    use super::entcode_h::ec_dec;
    extern "C" {
        #[c2rust::src_loc = "72:1"]
        pub fn ec_dec_bit_logp(_this: *mut ec_dec, _logp: libc::c_uint) -> libc::c_int;
        #[c2rust::src_loc = "82:1"]
        pub fn ec_dec_icdf(
            _this: *mut ec_dec,
            _icdf: *const libc::c_uchar,
            _ftb: libc::c_uint,
        ) -> libc::c_int;
        #[c2rust::src_loc = "98:1"]
        pub fn ec_dec_bits(_this: *mut ec_dec, _ftb: libc::c_uint) -> u32;
    }
}

#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/laplace.h:34"]
pub mod laplace_h {
    use super::entcode_h::{ec_dec, ec_enc};
    extern "C" {
        #[c2rust::src_loc = "39:1"]
        pub fn ec_laplace_encode(
            enc: *mut ec_enc,
            value: *mut libc::c_int,
            fs: libc::c_uint,
            decay: libc::c_int,
        );
        #[c2rust::src_loc = "48:1"]
        pub fn ec_laplace_decode(
            dec: *mut ec_dec,
            fs: libc::c_uint,
            decay: libc::c_int,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "internal:0"]
pub mod internal {
    #[c2rust::src_loc = "36:9"]
    pub const __CHAR_BIT__: libc::c_int = 8 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/stack_alloc.h:39"]
pub mod stack_alloc_h {
    #[c2rust::src_loc = "99:9"]
    pub const ALLOC_NONE: libc::c_int = 1 as libc::c_int;
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/rate.h:40"]
pub mod rate_h {
    #[c2rust::src_loc = "37:9"]
    pub const MAX_FINE_BITS: libc::c_int = 8 as libc::c_int;
}
pub use self::arch_h::{celt_ener, opus_val16, opus_val32};
pub use self::ecintrin_h::EC_CLZ0;
pub use self::entcode_h::{
    ec_ctx, ec_dec, ec_enc, ec_get_buffer, ec_range_bytes, ec_tell, ec_tell_frac, ec_window,
};
use self::entdec_h::{ec_dec_bit_logp, ec_dec_bits, ec_dec_icdf};
use self::entenc_h::{ec_enc_bit_logp, ec_enc_bits, ec_enc_icdf};
pub use self::internal::__CHAR_BIT__;
use self::laplace_h::{ec_laplace_decode, ec_laplace_encode};
pub use self::limits_h::CHAR_BIT;
pub use self::modes_h::{OpusCustomMode, PulseCache};
pub use self::rate_h::MAX_FINE_BITS;
pub use self::stack_alloc_h::ALLOC_NONE;

use crate::externs::memcpy;
#[no_mangle]
#[c2rust::src_loc = "53:18"]
pub static mut eMeans: [opus_val16; 25] = [
    6.437500f32,
    6.250000f32,
    5.750000f32,
    5.312500f32,
    5.062500f32,
    4.812500f32,
    4.500000f32,
    4.375000f32,
    4.875000f32,
    4.687500f32,
    4.562500f32,
    4.437500f32,
    4.875000f32,
    4.625000f32,
    4.312500f32,
    4.500000f32,
    4.375000f32,
    4.625000f32,
    4.750000f32,
    4.437500f32,
    3.750000f32,
    3.750000f32,
    3.750000f32,
    3.750000f32,
    3.750000f32,
];
#[c2rust::src_loc = "67:25"]
static mut pred_coef: [opus_val16; 4] = [
    (29440 as libc::c_int as libc::c_double / 32768.0f64) as opus_val16,
    (26112 as libc::c_int as libc::c_double / 32768.0f64) as opus_val16,
    (21248 as libc::c_int as libc::c_double / 32768.0f64) as opus_val16,
    (16384 as libc::c_int as libc::c_double / 32768.0f64) as opus_val16,
];
#[c2rust::src_loc = "68:25"]
static mut beta_coef: [opus_val16; 4] = [
    (30147 as libc::c_int as libc::c_double / 32768.0f64) as opus_val16,
    (22282 as libc::c_int as libc::c_double / 32768.0f64) as opus_val16,
    (12124 as libc::c_int as libc::c_double / 32768.0f64) as opus_val16,
    (6554 as libc::c_int as libc::c_double / 32768.0f64) as opus_val16,
];
#[c2rust::src_loc = "69:25"]
static mut beta_intra: opus_val16 =
    (4915 as libc::c_int as libc::c_double / 32768.0f64) as opus_val16;
#[c2rust::src_loc = "77:28"]
static mut e_prob_model: [[[libc::c_uchar; 42]; 2]; 4] = [
    [
        [
            72 as libc::c_int as libc::c_uchar,
            127 as libc::c_int as libc::c_uchar,
            65 as libc::c_int as libc::c_uchar,
            129 as libc::c_int as libc::c_uchar,
            66 as libc::c_int as libc::c_uchar,
            128 as libc::c_int as libc::c_uchar,
            65 as libc::c_int as libc::c_uchar,
            128 as libc::c_int as libc::c_uchar,
            64 as libc::c_int as libc::c_uchar,
            128 as libc::c_int as libc::c_uchar,
            62 as libc::c_int as libc::c_uchar,
            128 as libc::c_int as libc::c_uchar,
            64 as libc::c_int as libc::c_uchar,
            128 as libc::c_int as libc::c_uchar,
            64 as libc::c_int as libc::c_uchar,
            128 as libc::c_int as libc::c_uchar,
            92 as libc::c_int as libc::c_uchar,
            78 as libc::c_int as libc::c_uchar,
            92 as libc::c_int as libc::c_uchar,
            79 as libc::c_int as libc::c_uchar,
            92 as libc::c_int as libc::c_uchar,
            78 as libc::c_int as libc::c_uchar,
            90 as libc::c_int as libc::c_uchar,
            79 as libc::c_int as libc::c_uchar,
            116 as libc::c_int as libc::c_uchar,
            41 as libc::c_int as libc::c_uchar,
            115 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
            114 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
            132 as libc::c_int as libc::c_uchar,
            26 as libc::c_int as libc::c_uchar,
            132 as libc::c_int as libc::c_uchar,
            26 as libc::c_int as libc::c_uchar,
            145 as libc::c_int as libc::c_uchar,
            17 as libc::c_int as libc::c_uchar,
            161 as libc::c_int as libc::c_uchar,
            12 as libc::c_int as libc::c_uchar,
            176 as libc::c_int as libc::c_uchar,
            10 as libc::c_int as libc::c_uchar,
            177 as libc::c_int as libc::c_uchar,
            11 as libc::c_int as libc::c_uchar,
        ],
        [
            24 as libc::c_int as libc::c_uchar,
            179 as libc::c_int as libc::c_uchar,
            48 as libc::c_int as libc::c_uchar,
            138 as libc::c_int as libc::c_uchar,
            54 as libc::c_int as libc::c_uchar,
            135 as libc::c_int as libc::c_uchar,
            54 as libc::c_int as libc::c_uchar,
            132 as libc::c_int as libc::c_uchar,
            53 as libc::c_int as libc::c_uchar,
            134 as libc::c_int as libc::c_uchar,
            56 as libc::c_int as libc::c_uchar,
            133 as libc::c_int as libc::c_uchar,
            55 as libc::c_int as libc::c_uchar,
            132 as libc::c_int as libc::c_uchar,
            55 as libc::c_int as libc::c_uchar,
            132 as libc::c_int as libc::c_uchar,
            61 as libc::c_int as libc::c_uchar,
            114 as libc::c_int as libc::c_uchar,
            70 as libc::c_int as libc::c_uchar,
            96 as libc::c_int as libc::c_uchar,
            74 as libc::c_int as libc::c_uchar,
            88 as libc::c_int as libc::c_uchar,
            75 as libc::c_int as libc::c_uchar,
            88 as libc::c_int as libc::c_uchar,
            87 as libc::c_int as libc::c_uchar,
            74 as libc::c_int as libc::c_uchar,
            89 as libc::c_int as libc::c_uchar,
            66 as libc::c_int as libc::c_uchar,
            91 as libc::c_int as libc::c_uchar,
            67 as libc::c_int as libc::c_uchar,
            100 as libc::c_int as libc::c_uchar,
            59 as libc::c_int as libc::c_uchar,
            108 as libc::c_int as libc::c_uchar,
            50 as libc::c_int as libc::c_uchar,
            120 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
            122 as libc::c_int as libc::c_uchar,
            37 as libc::c_int as libc::c_uchar,
            97 as libc::c_int as libc::c_uchar,
            43 as libc::c_int as libc::c_uchar,
            78 as libc::c_int as libc::c_uchar,
            50 as libc::c_int as libc::c_uchar,
        ],
    ],
    [
        [
            83 as libc::c_int as libc::c_uchar,
            78 as libc::c_int as libc::c_uchar,
            84 as libc::c_int as libc::c_uchar,
            81 as libc::c_int as libc::c_uchar,
            88 as libc::c_int as libc::c_uchar,
            75 as libc::c_int as libc::c_uchar,
            86 as libc::c_int as libc::c_uchar,
            74 as libc::c_int as libc::c_uchar,
            87 as libc::c_int as libc::c_uchar,
            71 as libc::c_int as libc::c_uchar,
            90 as libc::c_int as libc::c_uchar,
            73 as libc::c_int as libc::c_uchar,
            93 as libc::c_int as libc::c_uchar,
            74 as libc::c_int as libc::c_uchar,
            93 as libc::c_int as libc::c_uchar,
            74 as libc::c_int as libc::c_uchar,
            109 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
            114 as libc::c_int as libc::c_uchar,
            36 as libc::c_int as libc::c_uchar,
            117 as libc::c_int as libc::c_uchar,
            34 as libc::c_int as libc::c_uchar,
            117 as libc::c_int as libc::c_uchar,
            34 as libc::c_int as libc::c_uchar,
            143 as libc::c_int as libc::c_uchar,
            17 as libc::c_int as libc::c_uchar,
            145 as libc::c_int as libc::c_uchar,
            18 as libc::c_int as libc::c_uchar,
            146 as libc::c_int as libc::c_uchar,
            19 as libc::c_int as libc::c_uchar,
            162 as libc::c_int as libc::c_uchar,
            12 as libc::c_int as libc::c_uchar,
            165 as libc::c_int as libc::c_uchar,
            10 as libc::c_int as libc::c_uchar,
            178 as libc::c_int as libc::c_uchar,
            7 as libc::c_int as libc::c_uchar,
            189 as libc::c_int as libc::c_uchar,
            6 as libc::c_int as libc::c_uchar,
            190 as libc::c_int as libc::c_uchar,
            8 as libc::c_int as libc::c_uchar,
            177 as libc::c_int as libc::c_uchar,
            9 as libc::c_int as libc::c_uchar,
        ],
        [
            23 as libc::c_int as libc::c_uchar,
            178 as libc::c_int as libc::c_uchar,
            54 as libc::c_int as libc::c_uchar,
            115 as libc::c_int as libc::c_uchar,
            63 as libc::c_int as libc::c_uchar,
            102 as libc::c_int as libc::c_uchar,
            66 as libc::c_int as libc::c_uchar,
            98 as libc::c_int as libc::c_uchar,
            69 as libc::c_int as libc::c_uchar,
            99 as libc::c_int as libc::c_uchar,
            74 as libc::c_int as libc::c_uchar,
            89 as libc::c_int as libc::c_uchar,
            71 as libc::c_int as libc::c_uchar,
            91 as libc::c_int as libc::c_uchar,
            73 as libc::c_int as libc::c_uchar,
            91 as libc::c_int as libc::c_uchar,
            78 as libc::c_int as libc::c_uchar,
            89 as libc::c_int as libc::c_uchar,
            86 as libc::c_int as libc::c_uchar,
            80 as libc::c_int as libc::c_uchar,
            92 as libc::c_int as libc::c_uchar,
            66 as libc::c_int as libc::c_uchar,
            93 as libc::c_int as libc::c_uchar,
            64 as libc::c_int as libc::c_uchar,
            102 as libc::c_int as libc::c_uchar,
            59 as libc::c_int as libc::c_uchar,
            103 as libc::c_int as libc::c_uchar,
            60 as libc::c_int as libc::c_uchar,
            104 as libc::c_int as libc::c_uchar,
            60 as libc::c_int as libc::c_uchar,
            117 as libc::c_int as libc::c_uchar,
            52 as libc::c_int as libc::c_uchar,
            123 as libc::c_int as libc::c_uchar,
            44 as libc::c_int as libc::c_uchar,
            138 as libc::c_int as libc::c_uchar,
            35 as libc::c_int as libc::c_uchar,
            133 as libc::c_int as libc::c_uchar,
            31 as libc::c_int as libc::c_uchar,
            97 as libc::c_int as libc::c_uchar,
            38 as libc::c_int as libc::c_uchar,
            77 as libc::c_int as libc::c_uchar,
            45 as libc::c_int as libc::c_uchar,
        ],
    ],
    [
        [
            61 as libc::c_int as libc::c_uchar,
            90 as libc::c_int as libc::c_uchar,
            93 as libc::c_int as libc::c_uchar,
            60 as libc::c_int as libc::c_uchar,
            105 as libc::c_int as libc::c_uchar,
            42 as libc::c_int as libc::c_uchar,
            107 as libc::c_int as libc::c_uchar,
            41 as libc::c_int as libc::c_uchar,
            110 as libc::c_int as libc::c_uchar,
            45 as libc::c_int as libc::c_uchar,
            116 as libc::c_int as libc::c_uchar,
            38 as libc::c_int as libc::c_uchar,
            113 as libc::c_int as libc::c_uchar,
            38 as libc::c_int as libc::c_uchar,
            112 as libc::c_int as libc::c_uchar,
            38 as libc::c_int as libc::c_uchar,
            124 as libc::c_int as libc::c_uchar,
            26 as libc::c_int as libc::c_uchar,
            132 as libc::c_int as libc::c_uchar,
            27 as libc::c_int as libc::c_uchar,
            136 as libc::c_int as libc::c_uchar,
            19 as libc::c_int as libc::c_uchar,
            140 as libc::c_int as libc::c_uchar,
            20 as libc::c_int as libc::c_uchar,
            155 as libc::c_int as libc::c_uchar,
            14 as libc::c_int as libc::c_uchar,
            159 as libc::c_int as libc::c_uchar,
            16 as libc::c_int as libc::c_uchar,
            158 as libc::c_int as libc::c_uchar,
            18 as libc::c_int as libc::c_uchar,
            170 as libc::c_int as libc::c_uchar,
            13 as libc::c_int as libc::c_uchar,
            177 as libc::c_int as libc::c_uchar,
            10 as libc::c_int as libc::c_uchar,
            187 as libc::c_int as libc::c_uchar,
            8 as libc::c_int as libc::c_uchar,
            192 as libc::c_int as libc::c_uchar,
            6 as libc::c_int as libc::c_uchar,
            175 as libc::c_int as libc::c_uchar,
            9 as libc::c_int as libc::c_uchar,
            159 as libc::c_int as libc::c_uchar,
            10 as libc::c_int as libc::c_uchar,
        ],
        [
            21 as libc::c_int as libc::c_uchar,
            178 as libc::c_int as libc::c_uchar,
            59 as libc::c_int as libc::c_uchar,
            110 as libc::c_int as libc::c_uchar,
            71 as libc::c_int as libc::c_uchar,
            86 as libc::c_int as libc::c_uchar,
            75 as libc::c_int as libc::c_uchar,
            85 as libc::c_int as libc::c_uchar,
            84 as libc::c_int as libc::c_uchar,
            83 as libc::c_int as libc::c_uchar,
            91 as libc::c_int as libc::c_uchar,
            66 as libc::c_int as libc::c_uchar,
            88 as libc::c_int as libc::c_uchar,
            73 as libc::c_int as libc::c_uchar,
            87 as libc::c_int as libc::c_uchar,
            72 as libc::c_int as libc::c_uchar,
            92 as libc::c_int as libc::c_uchar,
            75 as libc::c_int as libc::c_uchar,
            98 as libc::c_int as libc::c_uchar,
            72 as libc::c_int as libc::c_uchar,
            105 as libc::c_int as libc::c_uchar,
            58 as libc::c_int as libc::c_uchar,
            107 as libc::c_int as libc::c_uchar,
            54 as libc::c_int as libc::c_uchar,
            115 as libc::c_int as libc::c_uchar,
            52 as libc::c_int as libc::c_uchar,
            114 as libc::c_int as libc::c_uchar,
            55 as libc::c_int as libc::c_uchar,
            112 as libc::c_int as libc::c_uchar,
            56 as libc::c_int as libc::c_uchar,
            129 as libc::c_int as libc::c_uchar,
            51 as libc::c_int as libc::c_uchar,
            132 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
            150 as libc::c_int as libc::c_uchar,
            33 as libc::c_int as libc::c_uchar,
            140 as libc::c_int as libc::c_uchar,
            29 as libc::c_int as libc::c_uchar,
            98 as libc::c_int as libc::c_uchar,
            35 as libc::c_int as libc::c_uchar,
            77 as libc::c_int as libc::c_uchar,
            42 as libc::c_int as libc::c_uchar,
        ],
    ],
    [
        [
            42 as libc::c_int as libc::c_uchar,
            121 as libc::c_int as libc::c_uchar,
            96 as libc::c_int as libc::c_uchar,
            66 as libc::c_int as libc::c_uchar,
            108 as libc::c_int as libc::c_uchar,
            43 as libc::c_int as libc::c_uchar,
            111 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
            117 as libc::c_int as libc::c_uchar,
            44 as libc::c_int as libc::c_uchar,
            123 as libc::c_int as libc::c_uchar,
            32 as libc::c_int as libc::c_uchar,
            120 as libc::c_int as libc::c_uchar,
            36 as libc::c_int as libc::c_uchar,
            119 as libc::c_int as libc::c_uchar,
            33 as libc::c_int as libc::c_uchar,
            127 as libc::c_int as libc::c_uchar,
            33 as libc::c_int as libc::c_uchar,
            134 as libc::c_int as libc::c_uchar,
            34 as libc::c_int as libc::c_uchar,
            139 as libc::c_int as libc::c_uchar,
            21 as libc::c_int as libc::c_uchar,
            147 as libc::c_int as libc::c_uchar,
            23 as libc::c_int as libc::c_uchar,
            152 as libc::c_int as libc::c_uchar,
            20 as libc::c_int as libc::c_uchar,
            158 as libc::c_int as libc::c_uchar,
            25 as libc::c_int as libc::c_uchar,
            154 as libc::c_int as libc::c_uchar,
            26 as libc::c_int as libc::c_uchar,
            166 as libc::c_int as libc::c_uchar,
            21 as libc::c_int as libc::c_uchar,
            173 as libc::c_int as libc::c_uchar,
            16 as libc::c_int as libc::c_uchar,
            184 as libc::c_int as libc::c_uchar,
            13 as libc::c_int as libc::c_uchar,
            184 as libc::c_int as libc::c_uchar,
            10 as libc::c_int as libc::c_uchar,
            150 as libc::c_int as libc::c_uchar,
            13 as libc::c_int as libc::c_uchar,
            139 as libc::c_int as libc::c_uchar,
            15 as libc::c_int as libc::c_uchar,
        ],
        [
            22 as libc::c_int as libc::c_uchar,
            178 as libc::c_int as libc::c_uchar,
            63 as libc::c_int as libc::c_uchar,
            114 as libc::c_int as libc::c_uchar,
            74 as libc::c_int as libc::c_uchar,
            82 as libc::c_int as libc::c_uchar,
            84 as libc::c_int as libc::c_uchar,
            83 as libc::c_int as libc::c_uchar,
            92 as libc::c_int as libc::c_uchar,
            82 as libc::c_int as libc::c_uchar,
            103 as libc::c_int as libc::c_uchar,
            62 as libc::c_int as libc::c_uchar,
            96 as libc::c_int as libc::c_uchar,
            72 as libc::c_int as libc::c_uchar,
            96 as libc::c_int as libc::c_uchar,
            67 as libc::c_int as libc::c_uchar,
            101 as libc::c_int as libc::c_uchar,
            73 as libc::c_int as libc::c_uchar,
            107 as libc::c_int as libc::c_uchar,
            72 as libc::c_int as libc::c_uchar,
            113 as libc::c_int as libc::c_uchar,
            55 as libc::c_int as libc::c_uchar,
            118 as libc::c_int as libc::c_uchar,
            52 as libc::c_int as libc::c_uchar,
            125 as libc::c_int as libc::c_uchar,
            52 as libc::c_int as libc::c_uchar,
            118 as libc::c_int as libc::c_uchar,
            52 as libc::c_int as libc::c_uchar,
            117 as libc::c_int as libc::c_uchar,
            55 as libc::c_int as libc::c_uchar,
            135 as libc::c_int as libc::c_uchar,
            49 as libc::c_int as libc::c_uchar,
            137 as libc::c_int as libc::c_uchar,
            39 as libc::c_int as libc::c_uchar,
            157 as libc::c_int as libc::c_uchar,
            32 as libc::c_int as libc::c_uchar,
            145 as libc::c_int as libc::c_uchar,
            29 as libc::c_int as libc::c_uchar,
            97 as libc::c_int as libc::c_uchar,
            33 as libc::c_int as libc::c_uchar,
            77 as libc::c_int as libc::c_uchar,
            40 as libc::c_int as libc::c_uchar,
        ],
    ],
];
#[c2rust::src_loc = "140:28"]
static mut small_energy_icdf: [libc::c_uchar; 3] = [
    2 as libc::c_int as libc::c_uchar,
    1 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[c2rust::src_loc = "142:1"]
unsafe extern "C" fn loss_distortion(
    eBands: *const opus_val16,
    oldEBands: *mut opus_val16,
    start: libc::c_int,
    end: libc::c_int,
    len: libc::c_int,
    C: libc::c_int,
) -> opus_val32 {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut dist: opus_val32 = 0 as libc::c_int as opus_val32;
    c = 0 as libc::c_int;
    loop {
        i = start;
        while i < end {
            let d: opus_val16 =
                *eBands.offset((i + c * len) as isize) - *oldEBands.offset((i + c * len) as isize);
            dist = dist + d * d;
            i += 1;
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
    return if (200 as libc::c_int as libc::c_float) < dist {
        200 as libc::c_int as libc::c_float
    } else {
        dist
    };
}
#[c2rust::src_loc = "156:1"]
unsafe extern "C" fn quant_coarse_energy_impl(
    m: *const OpusCustomMode,
    start: libc::c_int,
    end: libc::c_int,
    eBands: *const opus_val16,
    oldEBands: *mut opus_val16,
    budget: i32,
    mut tell: i32,
    prob_model: *const libc::c_uchar,
    error: *mut opus_val16,
    enc: *mut ec_enc,
    C: libc::c_int,
    LM: libc::c_int,
    intra: libc::c_int,
    max_decay: opus_val16,
    lfe: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut badness: libc::c_int = 0 as libc::c_int;
    let mut prev: [opus_val32; 2] = [
        0 as libc::c_int as opus_val32,
        0 as libc::c_int as opus_val32,
    ];
    let mut coef: opus_val16 = 0.;
    let mut beta: opus_val16 = 0.;
    if tell + 3 as libc::c_int <= budget {
        ec_enc_bit_logp(enc, intra, 3 as libc::c_int as libc::c_uint);
    }
    if intra != 0 {
        coef = 0 as libc::c_int as opus_val16;
        beta = beta_intra;
    } else {
        beta = beta_coef[LM as usize];
        coef = pred_coef[LM as usize];
    }
    i = start;
    while i < end {
        c = 0 as libc::c_int;
        loop {
            let mut bits_left: libc::c_int = 0;
            let mut qi: libc::c_int = 0;
            let mut qi0: libc::c_int = 0;
            let mut q: opus_val32 = 0.;
            let mut x: opus_val16 = 0.;
            let mut f: opus_val32 = 0.;
            let mut tmp: opus_val32 = 0.;
            let mut oldE: opus_val16 = 0.;
            let mut decay_bound: opus_val16 = 0.;
            x = *eBands.offset((i + c * (*m).nbEBands) as isize);
            oldE = if -9.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
                -9.0f32
            } else {
                *oldEBands.offset((i + c * (*m).nbEBands) as isize)
            };
            f = x - coef * oldE - prev[c as usize];
            qi = (0.5f32 + f).floor() as libc::c_int;
            decay_bound = (if -28.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
                -28.0f32
            } else {
                *oldEBands.offset((i + c * (*m).nbEBands) as isize)
            }) - max_decay;
            if qi < 0 as libc::c_int && x < decay_bound {
                qi += (decay_bound - x) as libc::c_int;
                if qi > 0 as libc::c_int {
                    qi = 0 as libc::c_int;
                }
            }
            qi0 = qi;
            tell = ec_tell(enc);
            bits_left = budget - tell - 3 as libc::c_int * C * (end - i);
            if i != start && bits_left < 30 as libc::c_int {
                if bits_left < 24 as libc::c_int {
                    qi = if (1 as libc::c_int) < qi {
                        1 as libc::c_int
                    } else {
                        qi
                    };
                }
                if bits_left < 16 as libc::c_int {
                    qi = if -(1 as libc::c_int) > qi {
                        -(1 as libc::c_int)
                    } else {
                        qi
                    };
                }
            }
            if lfe != 0 && i >= 2 as libc::c_int {
                qi = if qi < 0 as libc::c_int {
                    qi
                } else {
                    0 as libc::c_int
                };
            }
            if budget - tell >= 15 as libc::c_int {
                let mut pi: libc::c_int = 0;
                pi = 2 as libc::c_int
                    * (if i < 20 as libc::c_int {
                        i
                    } else {
                        20 as libc::c_int
                    });
                ec_laplace_encode(
                    enc,
                    &mut qi,
                    ((*prob_model.offset(pi as isize) as libc::c_int) << 7 as libc::c_int)
                        as libc::c_uint,
                    (*prob_model.offset((pi + 1 as libc::c_int) as isize) as libc::c_int)
                        << 6 as libc::c_int,
                );
            } else if budget - tell >= 2 as libc::c_int {
                qi = if -(1 as libc::c_int)
                    > (if qi < 1 as libc::c_int {
                        qi
                    } else {
                        1 as libc::c_int
                    }) {
                    -(1 as libc::c_int)
                } else if qi < 1 as libc::c_int {
                    qi
                } else {
                    1 as libc::c_int
                };
                ec_enc_icdf(
                    enc,
                    2 as libc::c_int * qi ^ -((qi < 0 as libc::c_int) as libc::c_int),
                    small_energy_icdf.as_ptr(),
                    2 as libc::c_int as libc::c_uint,
                );
            } else if budget - tell >= 1 as libc::c_int {
                qi = if (0 as libc::c_int) < qi {
                    0 as libc::c_int
                } else {
                    qi
                };
                ec_enc_bit_logp(enc, -qi, 1 as libc::c_int as libc::c_uint);
            } else {
                qi = -(1 as libc::c_int);
            }
            *error.offset((i + c * (*m).nbEBands) as isize) = f - qi as libc::c_float;
            badness += (qi0 - qi).abs();
            q = qi as opus_val32;
            tmp = coef * oldE + prev[c as usize] + q;
            *oldEBands.offset((i + c * (*m).nbEBands) as isize) = tmp;
            prev[c as usize] = prev[c as usize] + q - beta * q;
            c += 1;
            if !(c < C) {
                break;
            }
        }
        i += 1;
    }
    return if lfe != 0 { 0 as libc::c_int } else { badness };
}
#[no_mangle]
#[c2rust::src_loc = "261:1"]
pub unsafe extern "C" fn quant_coarse_energy(
    m: *const OpusCustomMode,
    start: libc::c_int,
    end: libc::c_int,
    effEnd: libc::c_int,
    eBands: *const opus_val16,
    oldEBands: *mut opus_val16,
    budget: u32,
    error: *mut opus_val16,
    enc: *mut ec_enc,
    C: libc::c_int,
    LM: libc::c_int,
    nbAvailableBytes: libc::c_int,
    force_intra: libc::c_int,
    delayedIntra: *mut opus_val32,
    mut two_pass: libc::c_int,
    loss_rate: libc::c_int,
    lfe: libc::c_int,
) {
    let mut intra: libc::c_int = 0;
    let mut max_decay: opus_val16 = 0.;
    let mut enc_start_state: ec_enc = ec_enc {
        buf: 0 as *mut libc::c_uchar,
        storage: 0,
        end_offs: 0,
        end_window: 0,
        nend_bits: 0,
        nbits_total: 0,
        offs: 0,
        rng: 0,
        val: 0,
        ext: 0,
        rem: 0,
        error: 0,
    };
    let mut tell: u32 = 0;
    let mut badness1: libc::c_int = 0 as libc::c_int;
    let mut intra_bias: i32 = 0;
    let mut new_distortion: opus_val32 = 0.;
    intra = (force_intra != 0
        || two_pass == 0
            && *delayedIntra > (2 as libc::c_int * C * (end - start)) as libc::c_float
            && nbAvailableBytes > (end - start) * C) as libc::c_int;
    intra_bias = (budget as libc::c_float * *delayedIntra * loss_rate as libc::c_float
        / (C * 512 as libc::c_int) as libc::c_float) as i32;
    new_distortion = loss_distortion(eBands, oldEBands, start, effEnd, (*m).nbEBands, C);
    tell = ec_tell(enc) as u32;
    if tell.wrapping_add(3 as libc::c_int as libc::c_uint) > budget {
        intra = 0 as libc::c_int;
        two_pass = intra;
    }
    max_decay = 16.0f32;
    if end - start > 10 as libc::c_int {
        max_decay = if max_decay < 0.125f32 * nbAvailableBytes as libc::c_float {
            max_decay
        } else {
            0.125f32 * nbAvailableBytes as libc::c_float
        };
    }
    if lfe != 0 {
        max_decay = 3.0f32;
    }
    enc_start_state = *enc;
    let vla = (C * (*m).nbEBands) as usize;
    let mut oldEBands_intra: Vec<opus_val16> = ::std::vec::from_elem(0., vla);
    let vla_0 = (C * (*m).nbEBands) as usize;
    let mut error_intra: Vec<opus_val16> = ::std::vec::from_elem(0., vla_0);
    memcpy(
        oldEBands_intra.as_mut_ptr() as *mut libc::c_void,
        oldEBands as *const libc::c_void,
        ((C * (*m).nbEBands) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
            .wrapping_add(
                (0 as libc::c_int as libc::c_long
                    * oldEBands_intra.as_mut_ptr().offset_from(oldEBands) as libc::c_long)
                    as libc::c_ulong,
            ),
    );
    if two_pass != 0 || intra != 0 {
        badness1 = quant_coarse_energy_impl(
            m,
            start,
            end,
            eBands,
            oldEBands_intra.as_mut_ptr(),
            budget as i32,
            tell as i32,
            (e_prob_model[LM as usize][1 as libc::c_int as usize]).as_ptr(),
            error_intra.as_mut_ptr(),
            enc,
            C,
            LM,
            1 as libc::c_int,
            max_decay,
            lfe,
        );
    }
    if intra == 0 {
        let mut intra_buf: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
        let mut enc_intra_state: ec_enc = ec_enc {
            buf: 0 as *mut libc::c_uchar,
            storage: 0,
            end_offs: 0,
            end_window: 0,
            nend_bits: 0,
            nbits_total: 0,
            offs: 0,
            rng: 0,
            val: 0,
            ext: 0,
            rem: 0,
            error: 0,
        };
        let mut tell_intra: i32 = 0;
        let mut nstart_bytes: u32 = 0;
        let mut nintra_bytes: u32 = 0;
        let mut save_bytes: u32 = 0;
        let mut badness2: libc::c_int = 0;
        tell_intra = ec_tell_frac(enc) as i32;
        enc_intra_state = *enc;
        nstart_bytes = ec_range_bytes(&mut enc_start_state);
        nintra_bytes = ec_range_bytes(&mut enc_intra_state);
        intra_buf = (ec_get_buffer(&mut enc_intra_state)).offset(nstart_bytes as isize);
        save_bytes = nintra_bytes.wrapping_sub(nstart_bytes);
        if save_bytes == 0 as libc::c_int as libc::c_uint {
            save_bytes = ALLOC_NONE as u32;
        }
        let vla_1 = save_bytes as usize;
        let mut intra_bits: Vec<libc::c_uchar> = ::std::vec::from_elem(0, vla_1);
        memcpy(
            intra_bits.as_mut_ptr() as *mut libc::c_void,
            intra_buf as *const libc::c_void,
            (nintra_bytes.wrapping_sub(nstart_bytes) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * intra_bits.as_mut_ptr().offset_from(intra_buf) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        *enc = enc_start_state;
        badness2 = quant_coarse_energy_impl(
            m,
            start,
            end,
            eBands,
            oldEBands,
            budget as i32,
            tell as i32,
            (e_prob_model[LM as usize][intra as usize]).as_ptr(),
            error,
            enc,
            C,
            LM,
            0 as libc::c_int,
            max_decay,
            lfe,
        );
        if two_pass != 0
            && (badness1 < badness2
                || badness1 == badness2 && ec_tell_frac(enc) as i32 + intra_bias > tell_intra)
        {
            *enc = enc_intra_state;
            memcpy(
                intra_buf as *mut libc::c_void,
                intra_bits.as_mut_ptr() as *const libc::c_void,
                (nintra_bytes.wrapping_sub(nstart_bytes) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<libc::c_uchar>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * intra_buf.offset_from(intra_bits.as_mut_ptr()) as libc::c_long)
                            as libc::c_ulong,
                    ),
            );
            memcpy(
                oldEBands as *mut libc::c_void,
                oldEBands_intra.as_mut_ptr() as *const libc::c_void,
                ((C * (*m).nbEBands) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * oldEBands.offset_from(oldEBands_intra.as_mut_ptr()) as libc::c_long)
                            as libc::c_ulong,
                    ),
            );
            memcpy(
                error as *mut libc::c_void,
                error_intra.as_mut_ptr() as *const libc::c_void,
                ((C * (*m).nbEBands) as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
                    .wrapping_add(
                        (0 as libc::c_int as libc::c_long
                            * error.offset_from(error_intra.as_mut_ptr()) as libc::c_long)
                            as libc::c_ulong,
                    ),
            );
            intra = 1 as libc::c_int;
        }
    } else {
        memcpy(
            oldEBands as *mut libc::c_void,
            oldEBands_intra.as_mut_ptr() as *const libc::c_void,
            ((C * (*m).nbEBands) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * oldEBands.offset_from(oldEBands_intra.as_mut_ptr()) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
        memcpy(
            error as *mut libc::c_void,
            error_intra.as_mut_ptr() as *const libc::c_void,
            ((C * (*m).nbEBands) as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<opus_val16>() as libc::c_ulong)
                .wrapping_add(
                    (0 as libc::c_int as libc::c_long
                        * error.offset_from(error_intra.as_mut_ptr()) as libc::c_long)
                        as libc::c_ulong,
                ),
        );
    }
    if intra != 0 {
        *delayedIntra = new_distortion;
    } else {
        *delayedIntra =
            pred_coef[LM as usize] * pred_coef[LM as usize] * *delayedIntra + new_distortion;
    };
}
#[no_mangle]
#[c2rust::src_loc = "361:1"]
pub unsafe extern "C" fn quant_fine_energy(
    m: *const OpusCustomMode,
    start: libc::c_int,
    end: libc::c_int,
    oldEBands: *mut opus_val16,
    error: *mut opus_val16,
    fine_quant: *mut libc::c_int,
    enc: *mut ec_enc,
    C: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    i = start;
    while i < end {
        let frac: i16 = ((1 as libc::c_int) << *fine_quant.offset(i as isize)) as i16;
        if !(*fine_quant.offset(i as isize) <= 0 as libc::c_int) {
            c = 0 as libc::c_int;
            loop {
                let mut q2: libc::c_int = 0;
                let mut offset: opus_val16 = 0.;
                q2 = ((*error.offset((i + c * (*m).nbEBands) as isize) + 0.5f32)
                    * frac as libc::c_int as libc::c_float)
                    .floor() as libc::c_int;
                if q2 > frac as libc::c_int - 1 as libc::c_int {
                    q2 = frac as libc::c_int - 1 as libc::c_int;
                }
                if q2 < 0 as libc::c_int {
                    q2 = 0 as libc::c_int;
                }
                ec_enc_bits(
                    enc,
                    q2 as u32,
                    *fine_quant.offset(i as isize) as libc::c_uint,
                );
                offset = (q2 as libc::c_float + 0.5f32)
                    * ((1 as libc::c_int) << 14 as libc::c_int - *fine_quant.offset(i as isize))
                        as libc::c_float
                    * (1.0f32 / 16384 as libc::c_int as libc::c_float)
                    - 0.5f32;
                let ref mut fresh0 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                *fresh0 += offset;
                let ref mut fresh1 = *error.offset((i + c * (*m).nbEBands) as isize);
                *fresh1 -= offset;
                c += 1;
                if !(c < C) {
                    break;
                }
            }
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "398:1"]
pub unsafe extern "C" fn quant_energy_finalise(
    m: *const OpusCustomMode,
    start: libc::c_int,
    end: libc::c_int,
    oldEBands: *mut opus_val16,
    error: *mut opus_val16,
    fine_quant: *mut libc::c_int,
    fine_priority: *mut libc::c_int,
    mut bits_left: libc::c_int,
    enc: *mut ec_enc,
    C: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut prio: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    prio = 0 as libc::c_int;
    while prio < 2 as libc::c_int {
        i = start;
        while i < end && bits_left >= C {
            if !(*fine_quant.offset(i as isize) >= MAX_FINE_BITS
                || *fine_priority.offset(i as isize) != prio)
            {
                c = 0 as libc::c_int;
                loop {
                    let mut q2: libc::c_int = 0;
                    let mut offset: opus_val16 = 0.;
                    q2 = if *error.offset((i + c * (*m).nbEBands) as isize)
                        < 0 as libc::c_int as libc::c_float
                    {
                        0 as libc::c_int
                    } else {
                        1 as libc::c_int
                    };
                    ec_enc_bits(enc, q2 as u32, 1 as libc::c_int as libc::c_uint);
                    offset = (q2 as libc::c_float - 0.5f32)
                        * ((1 as libc::c_int)
                            << 14 as libc::c_int
                                - *fine_quant.offset(i as isize)
                                - 1 as libc::c_int) as libc::c_float
                        * (1.0f32 / 16384 as libc::c_int as libc::c_float);
                    let ref mut fresh2 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                    *fresh2 += offset;
                    let ref mut fresh3 = *error.offset((i + c * (*m).nbEBands) as isize);
                    *fresh3 -= offset;
                    bits_left -= 1;
                    c += 1;
                    if !(c < C) {
                        break;
                    }
                }
            }
            i += 1;
        }
        prio += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "428:1"]
pub unsafe extern "C" fn unquant_coarse_energy(
    m: *const OpusCustomMode,
    start: libc::c_int,
    end: libc::c_int,
    oldEBands: *mut opus_val16,
    intra: libc::c_int,
    dec: *mut ec_dec,
    C: libc::c_int,
    LM: libc::c_int,
) {
    let prob_model: *const libc::c_uchar = (e_prob_model[LM as usize][intra as usize]).as_ptr();
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut prev: [opus_val32; 2] = [
        0 as libc::c_int as opus_val32,
        0 as libc::c_int as opus_val32,
    ];
    let mut coef: opus_val16 = 0.;
    let mut beta: opus_val16 = 0.;
    let mut budget: i32 = 0;
    let mut tell: i32 = 0;
    if intra != 0 {
        coef = 0 as libc::c_int as opus_val16;
        beta = beta_intra;
    } else {
        beta = beta_coef[LM as usize];
        coef = pred_coef[LM as usize];
    }
    budget = ((*dec).storage).wrapping_mul(8 as libc::c_int as libc::c_uint) as i32;
    i = start;
    while i < end {
        c = 0 as libc::c_int;
        loop {
            let mut qi: libc::c_int = 0;
            let mut q: opus_val32 = 0.;
            let mut tmp: opus_val32 = 0.;
            tell = ec_tell(dec);
            if budget - tell >= 15 as libc::c_int {
                let mut pi: libc::c_int = 0;
                pi = 2 as libc::c_int
                    * (if i < 20 as libc::c_int {
                        i
                    } else {
                        20 as libc::c_int
                    });
                qi = ec_laplace_decode(
                    dec,
                    ((*prob_model.offset(pi as isize) as libc::c_int) << 7 as libc::c_int)
                        as libc::c_uint,
                    (*prob_model.offset((pi + 1 as libc::c_int) as isize) as libc::c_int)
                        << 6 as libc::c_int,
                );
            } else if budget - tell >= 2 as libc::c_int {
                qi = ec_dec_icdf(
                    dec,
                    small_energy_icdf.as_ptr(),
                    2 as libc::c_int as libc::c_uint,
                );
                qi = qi >> 1 as libc::c_int ^ -(qi & 1 as libc::c_int);
            } else if budget - tell >= 1 as libc::c_int {
                qi = -ec_dec_bit_logp(dec, 1 as libc::c_int as libc::c_uint);
            } else {
                qi = -(1 as libc::c_int);
            }
            q = qi as opus_val32;
            *oldEBands.offset((i + c * (*m).nbEBands) as isize) =
                if -9.0f32 > *oldEBands.offset((i + c * (*m).nbEBands) as isize) {
                    -9.0f32
                } else {
                    *oldEBands.offset((i + c * (*m).nbEBands) as isize)
                };
            tmp = coef * *oldEBands.offset((i + c * (*m).nbEBands) as isize) + prev[c as usize] + q;
            *oldEBands.offset((i + c * (*m).nbEBands) as isize) = tmp;
            prev[c as usize] = prev[c as usize] + q - beta * q;
            c += 1;
            if !(c < C) {
                break;
            }
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "493:1"]
pub unsafe extern "C" fn unquant_fine_energy(
    m: *const OpusCustomMode,
    start: libc::c_int,
    end: libc::c_int,
    oldEBands: *mut opus_val16,
    fine_quant: *mut libc::c_int,
    dec: *mut ec_dec,
    C: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    i = start;
    while i < end {
        if !(*fine_quant.offset(i as isize) <= 0 as libc::c_int) {
            c = 0 as libc::c_int;
            loop {
                let mut q2: libc::c_int = 0;
                let mut offset: opus_val16 = 0.;
                q2 =
                    ec_dec_bits(dec, *fine_quant.offset(i as isize) as libc::c_uint) as libc::c_int;
                offset = (q2 as libc::c_float + 0.5f32)
                    * ((1 as libc::c_int) << 14 as libc::c_int - *fine_quant.offset(i as isize))
                        as libc::c_float
                    * (1.0f32 / 16384 as libc::c_int as libc::c_float)
                    - 0.5f32;
                let ref mut fresh4 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                *fresh4 += offset;
                c += 1;
                if !(c < C) {
                    break;
                }
            }
        }
        i += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "516:1"]
pub unsafe extern "C" fn unquant_energy_finalise(
    m: *const OpusCustomMode,
    start: libc::c_int,
    end: libc::c_int,
    oldEBands: *mut opus_val16,
    fine_quant: *mut libc::c_int,
    fine_priority: *mut libc::c_int,
    mut bits_left: libc::c_int,
    dec: *mut ec_dec,
    C: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut prio: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    prio = 0 as libc::c_int;
    while prio < 2 as libc::c_int {
        i = start;
        while i < end && bits_left >= C {
            if !(*fine_quant.offset(i as isize) >= MAX_FINE_BITS
                || *fine_priority.offset(i as isize) != prio)
            {
                c = 0 as libc::c_int;
                loop {
                    let mut q2: libc::c_int = 0;
                    let mut offset: opus_val16 = 0.;
                    q2 = ec_dec_bits(dec, 1 as libc::c_int as libc::c_uint) as libc::c_int;
                    offset = (q2 as libc::c_float - 0.5f32)
                        * ((1 as libc::c_int)
                            << 14 as libc::c_int
                                - *fine_quant.offset(i as isize)
                                - 1 as libc::c_int) as libc::c_float
                        * (1.0f32 / 16384 as libc::c_int as libc::c_float);
                    let ref mut fresh5 = *oldEBands.offset((i + c * (*m).nbEBands) as isize);
                    *fresh5 += offset;
                    bits_left -= 1;
                    c += 1;
                    if !(c < C) {
                        break;
                    }
                }
            }
            i += 1;
        }
        prio += 1;
    }
}
#[no_mangle]
#[c2rust::src_loc = "544:1"]
pub unsafe extern "C" fn amp2Log2(
    m: *const OpusCustomMode,
    effEnd: libc::c_int,
    end: libc::c_int,
    bandE: *mut celt_ener,
    bandLogE: *mut opus_val16,
    C: libc::c_int,
) {
    let mut c: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    c = 0 as libc::c_int;
    loop {
        i = 0 as libc::c_int;
        while i < effEnd {
            *bandLogE.offset((i + c * (*m).nbEBands) as isize) = (std::f32::consts::LOG2_E
                * (*bandE.offset((i + c * (*m).nbEBands) as isize)).ln())
                - eMeans[i as usize];
            i += 1;
        }
        i = effEnd;
        while i < end {
            *bandLogE.offset((c * (*m).nbEBands + i) as isize) = -14.0f32;
            i += 1;
        }
        c += 1;
        if !(c < C) {
            break;
        }
    }
}
