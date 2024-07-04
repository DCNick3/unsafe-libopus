use nalgebra::{Dim, Matrix, ViewStorage, ViewStorageMut, U1};

// provide type-aliases for row-major views and functions for their construction

// pub type VectorViewR<'a, T, D, RStride = D, CStride = U1> =
//     Matrix<T, D, U1, ViewStorage<'a, T, D, U1, RStride, CStride>>;
//
// pub type VectorViewRMut<'a, T, D, RStride = D, CStride = U1> =
//     Matrix<T, D, U1, ViewStorageMut<'a, T, D, U1, RStride, CStride>>;

pub type MatrixViewR<'a, T, R, C, RStride = C, CStride = U1> =
    Matrix<T, R, C, ViewStorage<'a, T, R, C, RStride, CStride>>;

pub type MatrixViewRMut<'a, T, R, C, RStride = C, CStride = U1> =
    Matrix<T, R, C, ViewStorageMut<'a, T, R, C, RStride, CStride>>;

pub fn make_viewr_mut_generic<T, R, C>(slice: &mut [T], rows: R, cols: C) -> MatrixViewRMut<T, R, C>
where
    R: Dim,
    C: Dim,
{
    assert!(slice.len() <= rows.value() * cols.value());
    Matrix::from_data(unsafe {
        ViewStorageMut::from_raw_parts(slice.as_mut_ptr(), (rows, cols), (cols, U1))
    })
}

pub fn make_viewr_generic<T, R, C>(slice: &[T], rows: R, cols: C) -> MatrixViewR<T, R, C>
where
    R: Dim,
    C: Dim,
{
    assert!(slice.len() <= rows.value() * cols.value());
    Matrix::from_data(unsafe {
        ViewStorage::from_raw_parts(slice.as_ptr(), (rows, cols), (cols, U1))
    })
}
