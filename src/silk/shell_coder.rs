use crate::celt::entdec::{ec_dec, ec_dec_icdf};
use crate::celt::entenc::{ec_enc, ec_enc_icdf};
use crate::silk::tables_pulses_per_block::{
    silk_shell_code_table0, silk_shell_code_table1, silk_shell_code_table2, silk_shell_code_table3,
    silk_shell_code_table_offsets,
};
use ::libc;

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
