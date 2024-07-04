use crate::silk::define::LTP_ORDER;
use crate::silk::float::corrMatrix_FLP::{silk_corrMatrix_FLP, silk_corrVector_FLP};
use crate::silk::float::energy_FLP::silk_energy_FLP;
use crate::silk::float::scale_vector_FLP::silk_scale_vector_FLP;
use crate::silk::tuning_parameters::LTP_CORR_INV_MAX;
use nalgebra::{Const, Dyn, MatrixViewMut, VectorView, VectorViewMut, U1};

pub unsafe fn silk_find_LTP_FLP(
    XX: *mut f32,
    xX: *mut f32,
    mut r_ptr: *const f32,
    lag: *const i32,
    subfr_length: i32,
    nb_subfr: i32,
) {
    let mut k: i32 = 0;
    let mut xX_ptr: *mut f32 = 0 as *mut f32;
    let mut XX_ptr: *mut f32 = 0 as *mut f32;
    let mut lag_ptr: *const f32 = 0 as *const f32;
    let mut xx: f32 = 0.;
    xX_ptr = xX;
    XX_ptr = XX;
    k = 0;
    while k < nb_subfr {
        lag_ptr = r_ptr.offset(-((*lag.offset(k as isize) + LTP_ORDER / 2) as isize));

        let x = VectorView::<f32, Dyn>::from_slice_generic(
            std::slice::from_raw_parts(lag_ptr, (subfr_length + LTP_ORDER - 1) as usize),
            Dyn((subfr_length + LTP_ORDER - 1) as usize),
            U1,
        );

        let mut XX = MatrixViewMut::<
            f32,
            Const<{ LTP_ORDER as usize }>,
            Const<{ LTP_ORDER as usize }>,
        >::from_slice(std::slice::from_raw_parts_mut(
            XX_ptr,
            LTP_ORDER as usize * LTP_ORDER as usize,
        ));

        silk_corrMatrix_FLP(&x, Dyn(subfr_length as usize), &mut XX);

        let t = VectorView::<f32, Dyn>::from_slice_generic(
            std::slice::from_raw_parts(r_ptr, subfr_length as usize),
            Dyn(subfr_length as usize),
            U1,
        );

        let mut Xt = VectorViewMut::<f32, Const<{ LTP_ORDER as usize }>>::from_slice(
            std::slice::from_raw_parts_mut(xX_ptr, LTP_ORDER as usize),
        );

        silk_corrVector_FLP(&x, &t, &mut Xt);
        xx = silk_energy_FLP(std::slice::from_raw_parts(
            r_ptr,
            (subfr_length + LTP_ORDER) as usize,
        )) as f32;
        let temp = 1.0f32
            / xx.max(
                LTP_CORR_INV_MAX
                    * 0.5f32
                    * (*XX_ptr.offset(0 as isize) + *XX_ptr.offset(24 as isize))
                    + 1.0f32,
            );
        silk_scale_vector_FLP(XX_ptr, temp, LTP_ORDER * LTP_ORDER);
        silk_scale_vector_FLP(xX_ptr, temp, LTP_ORDER);
        r_ptr = r_ptr.offset(subfr_length as isize);
        XX_ptr = XX_ptr.offset((LTP_ORDER * LTP_ORDER) as isize);
        xX_ptr = xX_ptr.offset(LTP_ORDER as isize);
        k += 1;
    }
}
