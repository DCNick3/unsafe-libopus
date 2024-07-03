use crate::silk::float::energy_FLP::silk_energy_FLP;
use crate::silk::float::inner_product_FLP::silk_inner_product_FLP;

// Correlation matrix computations for LS estimate.

/// Calculates correlation vector X'*t
///
/// ```text
/// x       I    x vector [L+order-1] used to create X
/// t       I    Target vector [L]
/// L       I    Length of vecors
/// Order   I    Max lag for correlation
/// *Xt     O    X'*t correlation vector [order]
/// ```
pub fn silk_corrVector_FLP(x: &[f32], t: &[f32], Xt: &mut [f32]) {
    let L = t.len();

    for (out, x) in itertools::zip_eq(Xt.iter_mut(), x.windows(L).rev()) {
        *out = silk_inner_product_FLP(x, t) as f32;
    }
}

pub unsafe fn silk_corrMatrix_FLP(x: *const f32, L: i32, Order: i32, XX: *mut f32) {
    let mut j: i32 = 0;
    let mut lag: i32 = 0;
    let mut energy: f64 = 0.;
    let mut ptr1: *const f32 = 0 as *const f32;
    let mut ptr2: *const f32 = 0 as *const f32;
    ptr1 = &*x.offset((Order - 1) as isize) as *const f32;
    energy = silk_energy_FLP(std::slice::from_raw_parts(ptr1, L as usize));
    *XX.offset((0 * Order + 0) as isize) = energy as f32;
    j = 1;
    while j < Order {
        energy += (*ptr1.offset(-j as isize) * *ptr1.offset(-j as isize)
            - *ptr1.offset((L - j) as isize) * *ptr1.offset((L - j) as isize))
            as f64;
        *XX.offset((j * Order + j) as isize) = energy as f32;
        j += 1;
    }
    ptr2 = &*x.offset((Order - 2) as isize) as *const f32;
    lag = 1;
    while lag < Order {
        energy = silk_inner_product_FLP(
            std::slice::from_raw_parts(ptr1, L as usize),
            std::slice::from_raw_parts(ptr2, L as usize),
        );
        *XX.offset((lag * Order + 0) as isize) = energy as f32;
        *XX.offset((0 * Order + lag) as isize) = energy as f32;
        j = 1;
        while j < Order - lag {
            energy += (*ptr1.offset(-j as isize) * *ptr2.offset(-j as isize)
                - *ptr1.offset((L - j) as isize) * *ptr2.offset((L - j) as isize))
                as f64;
            *XX.offset(((lag + j) * Order + j) as isize) = energy as f32;
            *XX.offset((j * Order + (lag + j)) as isize) = energy as f32;
            j += 1;
        }
        ptr2 = ptr2.offset(-1);
        lag += 1;
    }
}
