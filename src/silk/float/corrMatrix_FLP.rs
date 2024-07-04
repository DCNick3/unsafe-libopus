use crate::silk::float::inner_product_FLP::silk_inner_product2_FLP;
use crate::util::nalgebra::MatrixViewRMut;
use nalgebra::{Dim, DimAdd, DimDiff, DimSub, DimSum, VectorView, U1};

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
pub fn silk_corrVector_FLP<L, Order>(
    x: &VectorView<f32, DimDiff<DimSum<L, Order>, U1>>,
    t: &VectorView<f32, L>,
    // accept a row vector because it's more convenient
    Xt: &mut MatrixViewRMut<f32, U1, Order>,
) where
    L: Dim,
    Order: Dim,
    L: DimAdd<Order>,
    <L as DimAdd<Order>>::Output: DimSub<U1>,
{
    let (x_len, _) = x.shape_generic();
    let (L, _) = t.shape_generic();
    let (_, Order) = Xt.shape_generic();
    assert_eq!(x_len.value(), L.add(Order).sub(U1).value());

    for lag in 0..Order.value() {
        let ptr1 = x.generic_view::<L, U1>((Order.value() - 1 - lag, 0), (L, U1));
        Xt[lag] = silk_inner_product2_FLP(&ptr1, t) as f32;
    }
}

/// Calculates correlation matrix X'*X
///
/// ```text
/// x       I   x vector [ L+order-1 ] used to create X
/// L       I   Length of vectors
/// Order   I   Max lag for correlation
/// XX      O   X'*X correlation matrix [order x order]
/// ```
pub fn silk_corrMatrix_FLP<Dx, L, Order>(
    x: &VectorView<f32, Dx>,
    L: L,
    XX: &mut MatrixViewRMut<f32, Order, Order, Order, U1>,
) where
    Dx: Dim,
    L: Dim,
    Order: Dim,
{
    let (Order, _) = XX.shape_generic();
    assert_eq!(x.shape().0, L.value() + Order.value() - 1);

    let window_at = |lag: usize| x.generic_view((Order.value() - 1 - lag, 0), (L, U1));
    let hvalue_at = |lag: usize| x[Order.value() - 1 - lag];
    let tvalue_at = |lag: usize| x[Order.value() + L.value() - 1 - lag];

    let Order = Order.value();

    // calculate the diagonal by using a sliding window
    for lag in 0..Order {
        // use a sliding window
        let mut energy = silk_inner_product2_FLP(&window_at(0), &window_at(lag));
        XX[(lag, 0)] = energy as f32;
        XX[(0, lag)] = energy as f32;

        for j in 1..(Order - lag) {
            energy +=
                // yes, this is how it's done in the C impl: the sliding window diff is calculated as an f32
                (hvalue_at(j) * hvalue_at(lag + j) - tvalue_at(j) * tvalue_at(lag + j)) as f64;
            XX[(lag + j, j)] = energy as f32;
            XX[(j, lag + j)] = energy as f32;
        }
    }
}
