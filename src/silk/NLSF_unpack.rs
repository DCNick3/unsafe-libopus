use crate::silk::structs::silk_NLSF_CB_struct;

pub unsafe fn silk_NLSF_unpack(
    ec_ix: *mut i16,
    pred_Q8: *mut u8,
    psNLSF_CB: *const silk_NLSF_CB_struct,
    CB1_index: i32,
) {
    let mut i: i32 = 0;
    let mut entry: u8 = 0;
    let mut ec_sel_ptr: *const u8 = 0 as *const u8;
    ec_sel_ptr = &*((*psNLSF_CB).ec_sel)
        .offset((CB1_index * (*psNLSF_CB).order as i32 / 2) as isize) as *const u8;
    i = 0;
    while i < (*psNLSF_CB).order as i32 {
        let fresh0 = ec_sel_ptr;
        ec_sel_ptr = ec_sel_ptr.offset(1);
        entry = *fresh0;
        *ec_ix.offset(i as isize) =
            ((entry as i32 >> 1 & 7) as i16 as i32 * (2 * 4 + 1) as i16 as i32) as i16;
        *pred_Q8.offset(i as isize) = *((*psNLSF_CB).pred_Q8)
            .offset((i + (entry as i32 & 1) * ((*psNLSF_CB).order as i32 - 1)) as isize);
        *ec_ix.offset((i + 1) as isize) =
            ((entry as i32 >> 5 & 7) as i16 as i32 * (2 * 4 + 1) as i16 as i32) as i16;
        *pred_Q8.offset((i + 1) as isize) = *((*psNLSF_CB).pred_Q8)
            .offset((i + (entry as i32 >> 4 & 1) * ((*psNLSF_CB).order as i32 - 1) + 1) as isize);
        i += 2;
    }
}
