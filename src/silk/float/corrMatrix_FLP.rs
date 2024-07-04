use crate::silk::float::energy_FLP::silk_energy_FLP;
use crate::silk::float::inner_product_FLP::silk_inner_product_FLP;
use nalgebra::{VectorView, VectorViewMut, U1};

// Correlation matrix computations for LS estimate.

/// Calculates correlation vector X'*t
///
/// ```text
/// x       I    x vector [L+order-1] used to create X
/// t       I    Target vector [L]
/// L       I    Length of vectors
/// Order   I    Max lag for correlation
/// Xt      O    X'*t correlation vector [order]
/// ```
pub fn silk_corrVector_FLP<Dx, L, Order>(
    x: &VectorView<f32, Dx>,
    t: &VectorView<f32, L>,
    Xt: &mut VectorViewMut<f32, Order>,
) where
    Dx: nalgebra::Dim,
    L: nalgebra::Dim,
    Order: nalgebra::Dim,
{
    let (L, _) = t.shape_generic();
    let (Order, _) = Xt.shape_generic();
    assert_eq!(x.shape().0, L.value() + Order.value() - 1);

    for lag in 0..Order.value() {
        let ptr1 = x.generic_view::<L, U1>((Order.value() - 1 - lag, 0), (L, U1));
        Xt[lag] = ptr1.dot(t);
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
