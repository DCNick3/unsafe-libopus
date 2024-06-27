use ndarray::{aview1, aview_mut1, azip};

// Interpolate two vectors
pub fn silk_interpolate(
    xi: &mut [i16],
    x0: &[i16],
    x1: &[i16],
    // interp. factor, weight on 2nd vector
    ifact_Q2: i32,
) {
    assert_eq!(xi.len(), x0.len());
    assert_eq!(xi.len(), x1.len());

    assert!((0..=4).contains(&ifact_Q2));

    let xi = aview_mut1(xi);
    let x0 = aview1(x0);
    let x1 = aview1(x1);

    azip!((xi in xi, &x0 in x0, &x1 in x1) {
        let x0 = x0 as i32;
        let x1 = x1 as i32;
        *xi = (x0
            + (((x1 - x0) as i16 as i32 * ifact_Q2 as i16 as i32) >> 2))
            as i16;
    });
}
