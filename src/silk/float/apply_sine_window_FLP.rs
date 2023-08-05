pub unsafe fn silk_apply_sine_window_FLP(
    px_win: *mut f32,
    px: *const f32,
    win_type: i32,
    length: i32,
) {
    let mut k: i32 = 0;
    let mut freq: f32 = 0.;
    let mut c: f32 = 0.;
    let mut S0: f32 = 0.;
    let mut S1: f32 = 0.;
    assert!(win_type == 1 as i32 || win_type == 2 as i32);
    assert!(length & 3 as i32 == 0 as i32);
    freq = std::f32::consts::PI / (length + 1 as i32) as f32;
    c = 2.0f32 - freq * freq;
    if win_type < 2 as i32 {
        S0 = 0.0f32;
        S1 = freq;
    } else {
        S0 = 1.0f32;
        S1 = 0.5f32 * c;
    }
    k = 0 as i32;
    while k < length {
        *px_win.offset((k + 0 as i32) as isize) =
            *px.offset((k + 0 as i32) as isize) * 0.5f32 * (S0 + S1);
        *px_win.offset((k + 1 as i32) as isize) = *px.offset((k + 1 as i32) as isize) * S1;
        S0 = c * S1 - S0;
        *px_win.offset((k + 2 as i32) as isize) =
            *px.offset((k + 2 as i32) as isize) * 0.5f32 * (S1 + S0);
        *px_win.offset((k + 3 as i32) as isize) = *px.offset((k + 3 as i32) as isize) * S0;
        S1 = c * S0 - S1;
        k += 4 as i32;
    }
}
