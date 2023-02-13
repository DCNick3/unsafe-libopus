#[c2rust::src_loc = "35:1"]
pub unsafe fn silk_scale_copy_vector_FLP(
    data_out: *mut libc::c_float,
    data_in: *const libc::c_float,
    gain: libc::c_float,
    dataSize: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut dataSize4: libc::c_int = 0;
    dataSize4 = dataSize & 0xfffc as libc::c_int;
    i = 0 as libc::c_int;
    while i < dataSize4 {
        *data_out.offset((i + 0 as libc::c_int) as isize) =
            gain * *data_in.offset((i + 0 as libc::c_int) as isize);
        *data_out.offset((i + 1 as libc::c_int) as isize) =
            gain * *data_in.offset((i + 1 as libc::c_int) as isize);
        *data_out.offset((i + 2 as libc::c_int) as isize) =
            gain * *data_in.offset((i + 2 as libc::c_int) as isize);
        *data_out.offset((i + 3 as libc::c_int) as isize) =
            gain * *data_in.offset((i + 3 as libc::c_int) as isize);
        i += 4 as libc::c_int;
    }
    while i < dataSize {
        *data_out.offset(i as isize) = gain * *data_in.offset(i as isize);
        i += 1;
    }
}
