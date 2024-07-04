use crate::silk::float::corrMatrix_FLP::{silk_corrMatrix_FLP, silk_corrVector_FLP};
use crate::silk::float::energy_FLP::silk_energy_FLP;
use crate::silk::tuning_parameters::LTP_CORR_INV_MAX;
use crate::util::nalgebra::MatrixViewRMut;
use nalgebra::{Const, Dim, DimMul, DimProd, Dyn, VectorView};

const LTP_ORDER: usize = crate::silk::define::LTP_ORDER as usize;
type LTP_ORDER = Const<{ LTP_ORDER }>;

/// LTP analysis
///
/// ```text
/// XX[ MAX_NB_SUBFR * LTP_ORDER * LTP_ORDER ]  /* O    Weight for LTP quantization
/// xX[ MAX_NB_SUBFR * LTP_ORDER ]              /* O    Weight for LTP quantization
/// r_ptr[]                                     /* I    LPC residual
/// lag[ MAX_NB_SUBFR ]                         /* I    LTP lags
/// subfr_length                                /* I    Subframe length
/// nb_subfr                                    /* I    number of subframes
/// ```
pub fn silk_find_LTP_FLP<NbSubfr>(
    XX: &mut MatrixViewRMut<f32, DimProd<NbSubfr, LTP_ORDER>, LTP_ORDER>,
    xX: &mut MatrixViewRMut<f32, NbSubfr, LTP_ORDER>,
    r: &[f32],
    mut r_ptr: usize,
    lag: &VectorView<i32, NbSubfr>,
    subfr_length: usize,
) where
    NbSubfr: Dim,
    NbSubfr: DimMul<LTP_ORDER>,
{
    let (nb_subfr_x_order, _) = XX.shape_generic();
    let (nb_subfr, _) = xX.shape_generic();

    assert_eq!(nb_subfr_x_order.value(), nb_subfr.value() * LTP_ORDER);
    assert_eq!(lag.shape().0, nb_subfr.value());

    for k in 0..nb_subfr.value() {
        let r_frame = VectorView::<f32, Dyn>::from_slice(&r[r_ptr..], subfr_length);
        let lag_frame = VectorView::<f32, Dyn>::from_slice(
            &r[r_ptr - lag[k] as usize - LTP_ORDER / 2..],
            subfr_length + LTP_ORDER - 1,
        );

        let mut XX_ptr = XX.fixed_view_mut::<{ LTP_ORDER }, { LTP_ORDER }>(k * LTP_ORDER, 0);
        let mut xX_ptr = xX.fixed_view_mut::<1, { LTP_ORDER }>(k, 0);

        silk_corrMatrix_FLP(&lag_frame, Dyn(subfr_length), &mut XX_ptr);
        silk_corrVector_FLP(&lag_frame, &r_frame, &mut xX_ptr);

        let xx = silk_energy_FLP(&r[r_ptr..][..subfr_length + LTP_ORDER]) as f32;
        let temp = 1.0 / xx.max(LTP_CORR_INV_MAX * 0.5 * (XX_ptr[(0, 0)] + XX_ptr[(4, 4)]) + 1.0);
        XX_ptr *= temp;
        xX_ptr *= temp;

        r_ptr += subfr_length;
    }
}
