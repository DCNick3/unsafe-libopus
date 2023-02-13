use crate::celt::entdec::{ec_dec, ec_dec_icdf};
use crate::celt::entenc::{ec_enc, ec_enc_icdf};
use crate::silk::tables_pulses_per_block::{
    silk_shell_code_table0, silk_shell_code_table1, silk_shell_code_table2, silk_shell_code_table3,
    silk_shell_code_table_offsets,
};

#[inline]
unsafe fn combine_pulses(out: *mut i32, in_0: *const i32, len: i32) {
    let mut k: i32 = 0;
    k = 0 as i32;
    while k < len {
        *out.offset(k as isize) = *in_0.offset((2 as i32 * k) as isize)
            + *in_0.offset((2 as i32 * k + 1 as i32) as isize);
        k += 1;
    }
}
#[inline]
unsafe fn encode_split(psRangeEnc: *mut ec_enc, p_child1: i32, p: i32, shell_table: *const u8) {
    if p > 0 as i32 {
        ec_enc_icdf(
            psRangeEnc,
            p_child1,
            &*shell_table
                .offset(*silk_shell_code_table_offsets.as_ptr().offset(p as isize) as isize),
            8 as i32 as u32,
        );
    }
}
#[inline]
unsafe fn decode_split(
    p_child1: *mut i16,
    p_child2: *mut i16,
    psRangeDec: *mut ec_dec,
    p: i32,
    shell_table: *const u8,
) {
    if p > 0 as i32 {
        *p_child1.offset(0 as i32 as isize) = ec_dec_icdf(
            psRangeDec,
            &*shell_table
                .offset(*silk_shell_code_table_offsets.as_ptr().offset(p as isize) as isize),
            8 as i32 as u32,
        ) as i16;
        *p_child2.offset(0 as i32 as isize) =
            (p - *p_child1.offset(0 as i32 as isize) as i32) as i16;
    } else {
        *p_child1.offset(0 as i32 as isize) = 0 as i32 as i16;
        *p_child2.offset(0 as i32 as isize) = 0 as i32 as i16;
    };
}
pub unsafe fn silk_shell_encoder(psRangeEnc: *mut ec_enc, pulses0: *const i32) {
    let mut pulses1: [i32; 8] = [0; 8];
    let mut pulses2: [i32; 4] = [0; 4];
    let mut pulses3: [i32; 2] = [0; 2];
    let mut pulses4: [i32; 1] = [0; 1];
    combine_pulses(pulses1.as_mut_ptr(), pulses0, 8 as i32);
    combine_pulses(pulses2.as_mut_ptr(), pulses1.as_mut_ptr(), 4 as i32);
    combine_pulses(pulses3.as_mut_ptr(), pulses2.as_mut_ptr(), 2 as i32);
    combine_pulses(pulses4.as_mut_ptr(), pulses3.as_mut_ptr(), 1 as i32);
    encode_split(
        psRangeEnc,
        pulses3[0 as i32 as usize],
        pulses4[0 as i32 as usize],
        silk_shell_code_table3.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses2[0 as i32 as usize],
        pulses3[0 as i32 as usize],
        silk_shell_code_table2.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses1[0 as i32 as usize],
        pulses2[0 as i32 as usize],
        silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(0 as i32 as isize),
        pulses1[0 as i32 as usize],
        silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(2 as i32 as isize),
        pulses1[1 as i32 as usize],
        silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses1[2 as i32 as usize],
        pulses2[1 as i32 as usize],
        silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(4 as i32 as isize),
        pulses1[2 as i32 as usize],
        silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(6 as i32 as isize),
        pulses1[3 as i32 as usize],
        silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses2[2 as i32 as usize],
        pulses3[1 as i32 as usize],
        silk_shell_code_table2.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses1[4 as i32 as usize],
        pulses2[2 as i32 as usize],
        silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(8 as i32 as isize),
        pulses1[4 as i32 as usize],
        silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(10 as i32 as isize),
        pulses1[5 as i32 as usize],
        silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        pulses1[6 as i32 as usize],
        pulses2[3 as i32 as usize],
        silk_shell_code_table1.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(12 as i32 as isize),
        pulses1[6 as i32 as usize],
        silk_shell_code_table0.as_ptr(),
    );
    encode_split(
        psRangeEnc,
        *pulses0.offset(14 as i32 as isize),
        pulses1[7 as i32 as usize],
        silk_shell_code_table0.as_ptr(),
    );
}
pub unsafe fn silk_shell_decoder(pulses0: *mut i16, psRangeDec: *mut ec_dec, pulses4: i32) {
    let mut pulses3: [i16; 2] = [0; 2];
    let mut pulses2: [i16; 4] = [0; 4];
    let mut pulses1: [i16; 8] = [0; 8];
    decode_split(
        &mut *pulses3.as_mut_ptr().offset(0 as i32 as isize),
        &mut *pulses3.as_mut_ptr().offset(1 as i32 as isize),
        psRangeDec,
        pulses4,
        silk_shell_code_table3.as_ptr(),
    );
    decode_split(
        &mut *pulses2.as_mut_ptr().offset(0 as i32 as isize),
        &mut *pulses2.as_mut_ptr().offset(1 as i32 as isize),
        psRangeDec,
        pulses3[0 as i32 as usize] as i32,
        silk_shell_code_table2.as_ptr(),
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(0 as i32 as isize),
        &mut *pulses1.as_mut_ptr().offset(1 as i32 as isize),
        psRangeDec,
        pulses2[0 as i32 as usize] as i32,
        silk_shell_code_table1.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(0 as i32 as isize),
        &mut *pulses0.offset(1 as i32 as isize),
        psRangeDec,
        pulses1[0 as i32 as usize] as i32,
        silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(2 as i32 as isize),
        &mut *pulses0.offset(3 as i32 as isize),
        psRangeDec,
        pulses1[1 as i32 as usize] as i32,
        silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(2 as i32 as isize),
        &mut *pulses1.as_mut_ptr().offset(3 as i32 as isize),
        psRangeDec,
        pulses2[1 as i32 as usize] as i32,
        silk_shell_code_table1.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(4 as i32 as isize),
        &mut *pulses0.offset(5 as i32 as isize),
        psRangeDec,
        pulses1[2 as i32 as usize] as i32,
        silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(6 as i32 as isize),
        &mut *pulses0.offset(7 as i32 as isize),
        psRangeDec,
        pulses1[3 as i32 as usize] as i32,
        silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses2.as_mut_ptr().offset(2 as i32 as isize),
        &mut *pulses2.as_mut_ptr().offset(3 as i32 as isize),
        psRangeDec,
        pulses3[1 as i32 as usize] as i32,
        silk_shell_code_table2.as_ptr(),
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(4 as i32 as isize),
        &mut *pulses1.as_mut_ptr().offset(5 as i32 as isize),
        psRangeDec,
        pulses2[2 as i32 as usize] as i32,
        silk_shell_code_table1.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(8 as i32 as isize),
        &mut *pulses0.offset(9 as i32 as isize),
        psRangeDec,
        pulses1[4 as i32 as usize] as i32,
        silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(10 as i32 as isize),
        &mut *pulses0.offset(11 as i32 as isize),
        psRangeDec,
        pulses1[5 as i32 as usize] as i32,
        silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses1.as_mut_ptr().offset(6 as i32 as isize),
        &mut *pulses1.as_mut_ptr().offset(7 as i32 as isize),
        psRangeDec,
        pulses2[3 as i32 as usize] as i32,
        silk_shell_code_table1.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(12 as i32 as isize),
        &mut *pulses0.offset(13 as i32 as isize),
        psRangeDec,
        pulses1[6 as i32 as usize] as i32,
        silk_shell_code_table0.as_ptr(),
    );
    decode_split(
        &mut *pulses0.offset(14 as i32 as isize),
        &mut *pulses0.offset(15 as i32 as isize),
        psRangeDec,
        pulses1[7 as i32 as usize] as i32,
        silk_shell_code_table0.as_ptr(),
    );
}
