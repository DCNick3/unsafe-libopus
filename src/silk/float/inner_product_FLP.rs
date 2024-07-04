use nalgebra::constraint::{DimEq, ShapeConstraint};
use nalgebra::{Dim, Matrix, RawStorage, U1};
use ndarray::{aview1, azip};

/// Inner product of two silk_float arrays, with result as double
pub fn silk_inner_product_FLP(data1: &[f32], data2: &[f32]) -> f64 {
    let data1 = aview1(data1);
    let data2 = aview1(data2);

    let mut result = 0f64;
    azip!((&x1 in data1, &x2 in data2) {
        result += x1 as f64 * x2 as f64;
    });

    result
}

// Based on nalgebra's [`Matrix::dot`], but with the result as a `f64`.
/// Inner product, but operating on nalgebra's [`VectorView`]s
pub fn silk_inner_product2_FLP<D1, D2, S1, S2>(
    lhs: &Matrix<f32, D1, U1, S1>,
    rhs: &Matrix<f32, D2, U1, S2>,
) -> f64
where
    D1: Dim,
    D2: Dim,
    S1: RawStorage<f32, D1, U1>,
    S2: RawStorage<f32, D2, U1>,
    ShapeConstraint: DimEq<D1, D2>,
{
    assert_eq!(lhs.shape(), rhs.shape());

    // All this is inspired from the "unrolled version" discussed in:
    // https://blog.theincredibleholk.org/blog/2012/12/10/optimizing-dot-product/
    //
    // And this comment from bluss:
    // https://users.rust-lang.org/t/how-to-zip-two-slices-efficiently/2048/12
    let mut res = 0.0;

    // We have to define them outside of the loop (and not inside at first assignment)
    // otherwise vectorization won't kick in for some reason.
    let mut acc0;
    let mut acc1;
    let mut acc2;
    let mut acc3;
    let mut acc4;
    let mut acc5;
    let mut acc6;
    let mut acc7;

    for j in 0..lhs.ncols() {
        let mut i = 0;

        acc0 = 0.0;
        acc1 = 0.0;
        acc2 = 0.0;
        acc3 = 0.0;
        acc4 = 0.0;
        acc5 = 0.0;
        acc6 = 0.0;
        acc7 = 0.0;

        while lhs.nrows() - i >= 8 {
            acc0 +=
                unsafe { *lhs.get_unchecked((i, j)) as f64 * *rhs.get_unchecked((i, j)) as f64 };
            acc1 += unsafe {
                *lhs.get_unchecked((i + 1, j)) as f64 * *rhs.get_unchecked((i + 1, j)) as f64
            };
            acc2 += unsafe {
                *lhs.get_unchecked((i + 2, j)) as f64 * *rhs.get_unchecked((i + 2, j)) as f64
            };
            acc3 += unsafe {
                *lhs.get_unchecked((i + 3, j)) as f64 * *rhs.get_unchecked((i + 3, j)) as f64
            };
            acc4 += unsafe {
                *lhs.get_unchecked((i + 4, j)) as f64 * *rhs.get_unchecked((i + 4, j)) as f64
            };
            acc5 += unsafe {
                *lhs.get_unchecked((i + 5, j)) as f64 * *rhs.get_unchecked((i + 5, j)) as f64
            };
            acc6 += unsafe {
                *lhs.get_unchecked((i + 6, j)) as f64 * *rhs.get_unchecked((i + 6, j)) as f64
            };
            acc7 += unsafe {
                *lhs.get_unchecked((i + 7, j)) as f64 * *rhs.get_unchecked((i + 7, j)) as f64
            };
            i += 8;
        }

        res += acc0 + acc4;
        res += acc1 + acc5;
        res += acc2 + acc6;
        res += acc3 + acc7;

        for k in i..lhs.nrows() {
            res += unsafe { *lhs.get_unchecked((k, j)) as f64 * *rhs.get_unchecked((k, j)) as f64 }
        }
    }

    res
}
