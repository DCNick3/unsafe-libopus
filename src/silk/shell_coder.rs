use ::libc;
#[c2rust::header_src = "/usr/include/bits/types.h:32"]
pub mod types_h {
    #[c2rust::src_loc = "38:1"]
    pub type __uint8_t = libc::c_uchar;
    #[c2rust::src_loc = "39:1"]
    pub type __int16_t = libc::c_short;
    #[c2rust::src_loc = "42:1"]
    pub type __uint32_t = libc::c_uint;
}
#[c2rust::header_src = "/usr/include/bits/stdint-intn.h:32"]
pub mod stdint_intn_h {
    #[c2rust::src_loc = "25:1"]
    pub type int16_t = __int16_t;
    use super::types_h::__int16_t;
}
#[c2rust::header_src = "/usr/include/bits/stdint-uintn.h:32"]
pub mod stdint_uintn_h {
    #[c2rust::src_loc = "24:1"]
    pub type uint8_t = __uint8_t;
    #[c2rust::src_loc = "26:1"]
    pub type uint32_t = __uint32_t;
    use super::types_h::{__uint32_t, __uint8_t};
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entcode.h:32"]
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
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entenc.h:32"]
pub mod entenc_h {
    use super::entcode_h::ec_enc;
    extern "C" {
        #[c2rust::src_loc = "65:1"]
        pub fn ec_enc_icdf(
            _this: *mut ec_enc,
            _s: libc::c_int,
            _icdf: *const libc::c_uchar,
            _ftb: libc::c_uint,
        );
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/celt/entdec.h:32"]
pub mod entdec_h {
    use super::entcode_h::ec_dec;
    extern "C" {
        #[c2rust::src_loc = "82:1"]
        pub fn ec_dec_icdf(
            _this: *mut ec_dec,
            _icdf: *const libc::c_uchar,
            _ftb: libc::c_uint,
        ) -> libc::c_int;
    }
}
#[c2rust::header_src = "/home/dcnick3/Downloads/opus-1.3.1/silk/tables.h:32"]
pub mod tables_h {
    extern "C" {
        #[c2rust::src_loc = "58:26"]
        pub static silk_shell_code_table0: [u8; 152];
        #[c2rust::src_loc = "59:26"]
        pub static silk_shell_code_table1: [u8; 152];
        #[c2rust::src_loc = "60:26"]
        pub static silk_shell_code_table2: [u8; 152];
        #[c2rust::src_loc = "61:26"]
        pub static silk_shell_code_table3: [u8; 152];
        #[c2rust::src_loc = "62:26"]
        pub static silk_shell_code_table_offsets: [u8; 17];
    }
}
pub use self::entcode_h::{ec_ctx, ec_dec, ec_enc, ec_window};
use self::entdec_h::ec_dec_icdf;
use self::entenc_h::ec_enc_icdf;
pub use self::stdint_intn_h::int16_t;
pub use self::stdint_uintn_h::{uint32_t, uint8_t};
use self::tables_h::{
    silk_shell_code_table0, silk_shell_code_table1, silk_shell_code_table2, silk_shell_code_table3,
    silk_shell_code_table_offsets,
};
pub use self::types_h::{__int16_t, __uint32_t, __uint8_t};
#[inline]
#[c2rust::src_loc = "36:1"]
unsafe extern "C" fn combine_pulses(
    out: *mut libc::c_int,
    in_0: *const libc::c_int,
    len: libc::c_int,
) {
    let mut k: libc::c_int = 0;
    k = 0 as libc::c_int;
    while k < len {
        *out.offset(k as isize) = *in_0.offset((2 as libc::c_int * k) as isize)
            + *in_0.offset((2 as libc::c_int * k + 1 as libc::c_int) as isize);
        k += 1;
    }
}
#[inline]
#[c2rust::src_loc = "48:1"]
unsafe extern "C" fn encode_split(
    psRangeEnc: *mut ec_enc,
    p_child1: libc::c_int,
    p: libc::c_int,
    shell_table: *const u8,
) {
    if p > 0 as libc::c_int {
        ec_enc_icdf(
            psRangeEnc,
            p_child1,
            &*shell_table
                .offset(*silk_shell_code_table_offsets.as_ptr().offset(p as isize) as isize),
            8 as libc::c_int as libc::c_uint,
        );
    }
}
#[inline]
#[c2rust::src_loc = "60:1"]
unsafe extern "C" fn decode_split(
    p_child1: *mut i16,
    p_child2: *mut i16,
    psRangeDec: *mut ec_dec,
    p: libc::c_int,
    shell_table: *const u8,
) {
    if p > 0 as libc::c_int {
        *p_child1.offset(0 as libc::c_int as isize) = ec_dec_icdf(
            psRangeDec,
            &*shell_table
                .offset(*silk_shell_code_table_offsets.as_ptr().offset(p as isize) as isize),
            8 as libc::c_int as libc::c_uint,
        ) as i16;
        *p_child2.offset(0 as libc::c_int as isize) =
            (p - *p_child1.offset(0 as libc::c_int as isize) as libc::c_int) as i16;
    } else {
        *p_child1.offset(0 as libc::c_int as isize) = 0 as libc::c_int as i16;
        *p_child2.offset(0 as libc::c_int as isize) = 0 as libc::c_int as i16;
    };
}
#[no_mangle]
#[c2rust::src_loc = "78:1"]
pub unsafe extern "C" fn silk_shell_encoder(psRangeEnc: *mut ec_enc, pulses0: *const libc::c_int) {
    let mut pulses1: [libc::c_int; 8] = [0; 8];
    let mut pulses2: [libc::c_int; 4] = [0; 4];
    let mut pulses3: [libc::c_int; 2] = [0; 2];
    let mut pulses4: [libc::c_int; 1] = [0; 1];
    combine_pulses(pulses1.as_mut_ptr(), pulses0, 8 as libc::c_int);
    combine_pulses(pulses2.as_mut_ptr(), pulses1.as_mut_ptr(), 4 as libc::c_int);
    combine_pulses(pulses3.as_mut_ptr(), pulses2.as_mut_ptr(), 2 as libc::c_int);
    combine_pulses(pulses4.as_mut_ptr(), pulses3.as_mut_ptr(), 1 as libc::c_int);
    encode_split(
        psRangeEnc,
        pulses3[0 as libc::c_int as usize],
        pulses4[0 as libc::c_int as usize],
        silk_shell_code_table3.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses2[0 as libc::c_int as usize],
        pulses3[0 as libc::c_int as usize],
        silk_shell_code_table2.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses1[0 as libc::c_int as usize],
        pulses2[0 as libc::c_int as usize],
        silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(0 as libc::c_int as isize),
        pulses1[0 as libc::c_int as usize],
        silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(2 as libc::c_int as isize),
        pulses1[1 as libc::c_int as usize],
        silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses1[2 as libc::c_int as usize],
        pulses2[1 as libc::c_int as usize],
        silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(4 as libc::c_int as isize),
        pulses1[2 as libc::c_int as usize],
        silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(6 as libc::c_int as isize),
        pulses1[3 as libc::c_int as usize],
        silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses2[2 as libc::c_int as usize],
        pulses3[1 as libc::c_int as usize],
        silk_shell_code_table2.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses1[4 as libc::c_int as usize],
        pulses2[2 as libc::c_int as usize],
        silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(8 as libc::c_int as isize),
        pulses1[4 as libc::c_int as usize],
        silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(10 as libc::c_int as isize),
        pulses1[5 as libc::c_int as usize],
        silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses1[6 as libc::c_int as usize],
        pulses2[3 as libc::c_int as usize],
        silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(12 as libc::c_int as isize),
        pulses1[6 as libc::c_int as usize],
        silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(14 as libc::c_int as isize),
        pulses1[7 as libc::c_int as usize],
        silk_shell_code_table0.as_ptr(),
    );
}
#[no_mangle]
#[c2rust::src_loc = "119:1"]
pub unsafe extern "C" fn silk_shell_decoder(
    pulses0: *mut i16,
    psRangeDec: *mut ec_dec,
    pulses4: libc::c_int,
) {
    let mut pulses3: [i16; 2] = [0; 2];
    let mut pulses2: [i16; 4] = [0; 4];
    let mut pulses1: [i16; 8] = [0; 8];
    decode_split(
        &mut *pulses3.as_mut_ptr().offset(0 as libc::c_int as isize),
        &mut *pulses3.as_mut_ptr().offset(1 as libc::c_int as isize),
        psRangeDec,
        pulses4,
        silk_shell_code_table3.as_ptr(),
    );
    decode_split(
        &mut *pulses2.as_mut_ptr().offset(0 as libc::c_int as isize),
        &mut *pulses2.as_mut_ptr().offset(1 as libc::c_int as isize),
        psRangeDec,
        pulses3[0 as libc::c_int as usize] as libc::c_int,
        silk_shell_code_table2.as_ptr(),
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(0 as libc::c_int as isize),
        &mut *pulses1.as_mut_ptr().offset(1 as libc::c_int as isize),
        psRangeDec,
        pulses2[0 as libc::c_int as usize] as libc::c_int,
        silk_shell_code_table1.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(0 as libc::c_int as isize),
        &mut *pulses0.offset(1 as libc::c_int as isize),
        psRangeDec,
        pulses1[0 as libc::c_int as usize] as libc::c_int,
        silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(2 as libc::c_int as isize),
        &mut *pulses0.offset(3 as libc::c_int as isize),
        psRangeDec,
        pulses1[1 as libc::c_int as usize] as libc::c_int,
        silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(2 as libc::c_int as isize),
        &mut *pulses1.as_mut_ptr().offset(3 as libc::c_int as isize),
        psRangeDec,
        pulses2[1 as libc::c_int as usize] as libc::c_int,
        silk_shell_code_table1.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(4 as libc::c_int as isize),
        &mut *pulses0.offset(5 as libc::c_int as isize),
        psRangeDec,
        pulses1[2 as libc::c_int as usize] as libc::c_int,
        silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(6 as libc::c_int as isize),
        &mut *pulses0.offset(7 as libc::c_int as isize),
        psRangeDec,
        pulses1[3 as libc::c_int as usize] as libc::c_int,
        silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses2.as_mut_ptr().offset(2 as libc::c_int as isize),
        &mut *pulses2.as_mut_ptr().offset(3 as libc::c_int as isize),
        psRangeDec,
        pulses3[1 as libc::c_int as usize] as libc::c_int,
        silk_shell_code_table2.as_ptr(),
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(4 as libc::c_int as isize),
        &mut *pulses1.as_mut_ptr().offset(5 as libc::c_int as isize),
        psRangeDec,
        pulses2[2 as libc::c_int as usize] as libc::c_int,
        silk_shell_code_table1.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(8 as libc::c_int as isize),
        &mut *pulses0.offset(9 as libc::c_int as isize),
        psRangeDec,
        pulses1[4 as libc::c_int as usize] as libc::c_int,
        silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(10 as libc::c_int as isize),
        &mut *pulses0.offset(11 as libc::c_int as isize),
        psRangeDec,
        pulses1[5 as libc::c_int as usize] as libc::c_int,
        silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(6 as libc::c_int as isize),
        &mut *pulses1.as_mut_ptr().offset(7 as libc::c_int as isize),
        psRangeDec,
        pulses2[3 as libc::c_int as usize] as libc::c_int,
        silk_shell_code_table1.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(12 as libc::c_int as isize),
        &mut *pulses0.offset(13 as libc::c_int as isize),
        psRangeDec,
        pulses1[6 as libc::c_int as usize] as libc::c_int,
        silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(14 as libc::c_int as isize),
        &mut *pulses0.offset(15 as libc::c_int as isize),
        psRangeDec,
        pulses1[7 as libc::c_int as usize] as libc::c_int,
        silk_shell_code_table0.as_ptr(),
    );
}
